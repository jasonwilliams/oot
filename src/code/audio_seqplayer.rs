#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn AudioHeap_AllocZeroed(pool: *mut AudioAllocPool, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_SearchCaches(tableType: s32, arg1: s32, id: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_LoadFilter(filter: *mut s16, filter1: s32, filter2: s32);
    #[no_mangle]
    fn AudioLoad_IsFontLoadComplete(fontId: s32) -> s32;
    #[no_mangle]
    fn AudioLoad_IsSeqLoadComplete(seqId: s32) -> s32;
    #[no_mangle]
    fn AudioLoad_SetFontLoadStatus(fontId: s32, status: s32);
    #[no_mangle]
    fn AudioLoad_SetSeqLoadStatus(seqId: s32, status: s32);
    #[no_mangle]
    fn AudioLoad_SyncInitSeqPlayer(playerIdx: s32, seqId: s32, arg2: s32)
     -> s32;
    #[no_mangle]
    fn AudioLoad_SlowLoadSample(arg0: s32, arg1: s32, arg2: *mut s8) -> s32;
    #[no_mangle]
    fn AudioLoad_SlowLoadSeq(playerIdx: s32, ramAddr: *mut u8_0,
                             arg2: *mut s8) -> s32;
    #[no_mangle]
    fn AudioLoad_ScriptLoad(tableType: s32, arg1: s32, arg2: *mut s8);
    #[no_mangle]
    fn Audio_NextRandom() -> u32_0;
    #[no_mangle]
    fn Audio_ProcessNotes();
    #[no_mangle]
    fn Audio_InstrumentGetSound(instrument: *mut Instrument, semitone: s32)
     -> *mut SoundFontSound;
    #[no_mangle]
    fn Audio_GetInstrumentInner(fontId: s32, instId: s32) -> *mut Instrument;
    #[no_mangle]
    fn Audio_GetDrum(fontId: s32, drumId: s32) -> *mut Drum;
    #[no_mangle]
    fn Audio_GetSfx(fontId: s32, sfxId: s32) -> *mut SoundFontSound;
    #[no_mangle]
    fn Audio_SeqLayerNoteDecay(layer: *mut SequenceLayer);
    #[no_mangle]
    fn Audio_SeqLayerNoteRelease(layer: *mut SequenceLayer);
    #[no_mangle]
    fn Audio_InitSyntheticWave(note: *mut Note, layer: *mut SequenceLayer);
    #[no_mangle]
    fn Audio_InitNoteLists(pool: *mut NotePool);
    #[no_mangle]
    fn Audio_NotePoolClear(pool: *mut NotePool);
    #[no_mangle]
    fn Audio_NotePoolFill(pool: *mut NotePool, count: s32);
    #[no_mangle]
    fn Audio_AllocNote(layer: *mut SequenceLayer) -> *mut Note;
    #[no_mangle]
    fn Audio_SequencePlayerProcessSound(seqPlayer: *mut SequencePlayer);
    #[no_mangle]
    fn Audio_NoteVibratoInit(note: *mut Note);
    #[no_mangle]
    fn Audio_NotePortamentoInit(note: *mut Note);
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    #[no_mangle]
    static mut gNoteFrequencies: [f32_0; 0];
    #[no_mangle]
    static mut gBendPitchTwoSemitonesFrequencies: [f32_0; 256];
    #[no_mangle]
    static mut gBendPitchOneOctaveFrequencies: [f32_0; 256];
    #[no_mangle]
    static mut gDefaultEnvelope: [AdsrEnvelope; 4];
    #[no_mangle]
    static mut gDefaultShortNoteGateTimeTable: [u8_0; 16];
    #[no_mangle]
    static mut gDefaultShortNoteVelocityTable: [u8_0; 16];
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SAMPLE_TABLE: C2RustUnnamed_2 = 2;
pub const FONT_TABLE: C2RustUnnamed_2 = 1;
pub const SEQUENCE_TABLE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CACHE_PERMANENT: C2RustUnnamed_3 = 3;
pub const CACHE_EITHER: C2RustUnnamed_3 = 2;
pub const CACHE_PERSISTENT: C2RustUnnamed_3 = 1;
pub const CACHE_TEMPORARY: C2RustUnnamed_3 = 0;
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
    pub bitField0: C2RustUnnamed_6,
    pub bitField1: C2RustUnnamed_5,
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
    pub sound: C2RustUnnamed_4,
    pub filter: *mut s16,
    pub pad_18: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    #[bitfield(name = "reverbIndex", ty = "u8_0", bits = "0..=2")]
    #[bitfield(name = "bookOffset", ty = "u8_0", bits = "3..=4")]
    #[bitfield(name = "isSyntheticWave", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "hasTwoParts", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "usesHeadsetPanEffects2", ty = "u8_0", bits = "7..=7")]
    pub reverbIndex_bookOffset_isSyntheticWave_hasTwoParts_usesHeadsetPanEffects2: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub changes: C2RustUnnamed_8,
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
    pub u: C2RustUnnamed_7,
    pub pool: *mut NotePool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub union C2RustUnnamed_8 {
    pub s: C2RustUnnamed_9,
    pub asByte: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub action: C2RustUnnamed_10,
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
pub union C2RustUnnamed_10 {
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
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
pub union C2RustUnnamed_12 {
    pub opArgs: u32_0,
    pub c2rust_unnamed: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub static mut D_80130520: [u8_0; 80] =
    [0x81 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x81 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x42 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0xc2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x42 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x81 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0x81 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x3 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x81 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x82 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x81 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x81 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0x81 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_GetScriptControlFlowArgument(mut state:
                                                                   *mut SeqScriptState,
                                                               mut arg1: u8_0)
 -> u16_0 {
    let mut temp_v0: u8_0 =
        D_80130520[(arg1 as libc::c_int - 0xb0 as libc::c_int) as usize];
    let mut loBits: u8_0 =
        (temp_v0 as libc::c_int & 3 as libc::c_int) as u8_0;
    let mut ret: u16_0 = 0 as libc::c_int as u16_0;
    if loBits as libc::c_int == 1 as libc::c_int {
        if temp_v0 as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            ret = AudioSeq_ScriptReadU8(state) as u16_0
        } else { ret = AudioSeq_ScriptReadS16(state) as u16_0 }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_HandleScriptFlowControl(mut seqPlayer:
                                                              *mut SequencePlayer,
                                                          mut state:
                                                              *mut SeqScriptState,
                                                          mut cmd: s32,
                                                          mut arg: s32)
 -> s32 {
    match cmd {
        255 => {
            if (*state).depth as libc::c_int == 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
            (*state).depth = (*state).depth.wrapping_sub(1);
            (*state).pc = (*state).stack[(*state).depth as usize]
        }
        253 => { return AudioSeq_ScriptReadCompressedU16(state) as s32 }
        254 => { return 1 as libc::c_int }
        252 => {
            let fresh0 = (*state).depth;
            (*state).depth = (*state).depth.wrapping_add(1);
            (*state).stack[fresh0 as usize] = (*state).pc;
            (*state).pc =
                (*seqPlayer).seqData.offset(arg as u16_0 as libc::c_int as
                                                isize)
        }
        248 => {
            (*state).remLoopIters[(*state).depth as usize] = arg as u8_0;
            let fresh1 = (*state).depth;
            (*state).depth = (*state).depth.wrapping_add(1);
            (*state).stack[fresh1 as usize] = (*state).pc
        }
        247 => {
            (*state).remLoopIters[((*state).depth as libc::c_int -
                                       1 as libc::c_int) as usize] =
                (*state).remLoopIters[((*state).depth as libc::c_int -
                                           1 as libc::c_int) as
                                          usize].wrapping_sub(1);
            if (*state).remLoopIters[((*state).depth as libc::c_int -
                                          1 as libc::c_int) as usize] as
                   libc::c_int != 0 as libc::c_int {
                (*state).pc =
                    (*state).stack[((*state).depth as libc::c_int -
                                        1 as libc::c_int) as usize]
            } else { (*state).depth = (*state).depth.wrapping_sub(1) }
        }
        246 => { (*state).depth = (*state).depth.wrapping_sub(1) }
        245 | 249 | 250 | 251 => {
            if !(cmd == 0xfa as libc::c_int &&
                     (*state).value as libc::c_int != 0 as libc::c_int) {
                if !(cmd == 0xf9 as libc::c_int &&
                         (*state).value as libc::c_int >= 0 as libc::c_int) {
                    if !(cmd == 0xf5 as libc::c_int &&
                             ((*state).value as libc::c_int) <
                                 0 as libc::c_int) {
                        (*state).pc =
                            (*seqPlayer).seqData.offset(arg as u16_0 as
                                                            libc::c_int as
                                                            isize)
                    }
                }
            }
        }
        242 | 243 | 244 => {
            if !(cmd == 0xf3 as libc::c_int &&
                     (*state).value as libc::c_int != 0 as libc::c_int) {
                if !(cmd == 0xf2 as libc::c_int &&
                         (*state).value as libc::c_int >= 0 as libc::c_int) {
                    (*state).pc =
                        (*state).pc.offset((arg & 0xff as libc::c_int) as s8
                                               as libc::c_int as isize)
                }
            }
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_InitSequenceChannel(mut channel:
                                                          *mut SequenceChannel) {
    let mut i: s32 = 0;
    if channel ==
           &mut gAudioContext.sequenceChannelNone as *mut SequenceChannel {
        return
    }
    (*channel).set_enabled(0 as libc::c_int as u8_0);
    (*channel).set_finished(0 as libc::c_int as u8_0);
    (*channel).set_stopScript(0 as libc::c_int as u8_0);
    (*channel).set_stopSomething2(0 as libc::c_int as u8_0);
    (*channel).set_hasInstrument(0 as libc::c_int as u8_0);
    (*channel).set_stereoHeadsetEffects(0 as libc::c_int as u8_0);
    (*channel).transposition = 0 as libc::c_int as s16;
    (*channel).set_largeNotes(0 as libc::c_int as u8_0);
    (*channel).bookOffset = 0 as libc::c_int as u8_0;
    (*channel).stereo.asByte = 0 as libc::c_int as u8_0;
    (*channel).changes.asByte = 0xff as libc::c_int as u8_0;
    (*channel).scriptState.depth = 0 as libc::c_int as u8_0;
    (*channel).newPan = 0x40 as libc::c_int as u8_0;
    (*channel).panChannelWeight = 0x80 as libc::c_int as u8_0;
    (*channel).velocityRandomVariance = 0 as libc::c_int as u8_0;
    (*channel).gateTimeRandomVariance = 0 as libc::c_int as u8_0;
    (*channel).noteUnused = 0 as *mut Note;
    (*channel).reverbIndex = 0 as libc::c_int as u8_0;
    (*channel).reverb = 0 as libc::c_int as u8_0;
    (*channel).unk_0C = 0 as libc::c_int as u8_0;
    (*channel).notePriority = 3 as libc::c_int as u8_0;
    (*channel).someOtherPriority = 1 as libc::c_int as u8_0;
    (*channel).delay = 0 as libc::c_int as u16_0;
    (*channel).adsr.envelope = gDefaultEnvelope.as_mut_ptr();
    (*channel).adsr.releaseRate = 0xf0 as libc::c_int as u8_0;
    (*channel).adsr.sustain = 0 as libc::c_int as u8_0;
    (*channel).vibratoRateTarget = 0x800 as libc::c_int as u16_0;
    (*channel).vibratoRateStart = 0x800 as libc::c_int as u16_0;
    (*channel).vibratoExtentTarget = 0 as libc::c_int as u16_0;
    (*channel).vibratoExtentStart = 0 as libc::c_int as u16_0;
    (*channel).vibratoRateChangeDelay = 0 as libc::c_int as u16_0;
    (*channel).vibratoExtentChangeDelay = 0 as libc::c_int as u16_0;
    (*channel).vibratoDelay = 0 as libc::c_int as u16_0;
    (*channel).filter = 0 as *mut s16;
    (*channel).unk_20 = 0 as libc::c_int as u16_0;
    (*channel).unk_0F = 0 as libc::c_int as u8_0;
    (*channel).volume = 1.0f32;
    (*channel).volumeScale = 1.0f32;
    (*channel).freqScale = 1.0f32;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        (*channel).soundScriptIO[i as usize] = -(1 as libc::c_int) as s8;
        i += 1
    }
    (*channel).set_unused(0 as libc::c_int as u8_0);
    Audio_InitNoteLists(&mut (*channel).notePool);
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqChannelSetLayer(mut channel:
                                                         *mut SequenceChannel,
                                                     mut layerIdx: s32)
 -> s32 {
    let mut layer: *mut SequenceLayer = 0 as *mut SequenceLayer;
    if (*channel).layers[layerIdx as usize].is_null() {
        let mut layer_0: *mut SequenceLayer = 0 as *mut SequenceLayer;
        layer_0 =
            AudioSeq_AudioListPopBack(&mut gAudioContext.layerFreeList) as
                *mut SequenceLayer;
        (*channel).layers[layerIdx as usize] = layer_0;
        if layer_0.is_null() {
            (*channel).layers[layerIdx as usize] = 0 as *mut SequenceLayer;
            return -(1 as libc::c_int)
        }
    } else { Audio_SeqLayerNoteDecay((*channel).layers[layerIdx as usize]); }
    layer = (*channel).layers[layerIdx as usize];
    (*layer).channel = channel;
    (*layer).adsr = (*channel).adsr;
    (*layer).adsr.releaseRate = 0 as libc::c_int as u8_0;
    (*layer).set_enabled(1 as libc::c_int as u8_0);
    (*layer).set_finished(0 as libc::c_int as u8_0);
    (*layer).set_stopSomething(0 as libc::c_int as u8_0);
    (*layer).set_continuousNotes(0 as libc::c_int as u8_0);
    (*layer).set_bit3(0 as libc::c_int as u8_0);
    (*layer).set_ignoreDrumPan(0 as libc::c_int as u8_0);
    (*layer).set_bit1(0 as libc::c_int as u8_0);
    (*layer).set_notePropertiesNeedInit(0 as libc::c_int as u8_0);
    (*layer).stereo.asByte = 0 as libc::c_int as u8_0;
    (*layer).portamento.mode = 0 as libc::c_int as u8_0;
    (*layer).scriptState.depth = 0 as libc::c_int as u8_0;
    (*layer).gateTime = 0x80 as libc::c_int as u8_0;
    (*layer).pan = 0x40 as libc::c_int as u8_0;
    (*layer).transposition = 0 as libc::c_int as s16;
    (*layer).delay = 0 as libc::c_int as s16;
    (*layer).gateDelay = 0 as libc::c_int as s16;
    (*layer).delay2 = 0 as libc::c_int as s16;
    (*layer).note = 0 as *mut Note;
    (*layer).instrument = 0 as *mut Instrument;
    (*layer).freqScale = 1.0f32;
    (*layer).unk_34 = 1.0f32;
    (*layer).velocitySquare2 = 0.0f32;
    (*layer).instOrWave = 0xff as libc::c_int as u8_0;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerDisable(mut layer:
                                                      *mut SequenceLayer) {
    if !layer.is_null() {
        if (*layer).channel !=
               &mut gAudioContext.sequenceChannelNone as *mut SequenceChannel
               &&
               (*(*(*layer).channel).seqPlayer).finished() as libc::c_int ==
                   1 as libc::c_int {
            Audio_SeqLayerNoteRelease(layer);
        } else { Audio_SeqLayerNoteDecay(layer); }
        (*layer).set_enabled(0 as libc::c_int as u8_0);
        (*layer).set_finished(1 as libc::c_int as u8_0)
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerFree(mut channel:
                                                   *mut SequenceChannel,
                                               mut layerIdx: s32) {
    let mut layer: *mut SequenceLayer = (*channel).layers[layerIdx as usize];
    if !layer.is_null() {
        AudioSeq_AudioListPushBack(&mut gAudioContext.layerFreeList,
                                   &mut (*layer).listItem);
        AudioSeq_SeqLayerDisable(layer);
        (*channel).layers[layerIdx as usize] = 0 as *mut SequenceLayer
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequenceChannelDisable(mut channel:
                                                             *mut SequenceChannel) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int { AudioSeq_SeqLayerFree(channel, i); i += 1 }
    Audio_NotePoolClear(&mut (*channel).notePool);
    (*channel).set_enabled(0 as libc::c_int as u8_0);
    (*channel).set_finished(1 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequencePlayerSetupChannels(mut seqPlayer:
                                                                  *mut SequencePlayer,
                                                              mut channelBits:
                                                                  u16_0) {
    let mut channel: *mut SequenceChannel = 0 as *mut SequenceChannel;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 0x10 as libc::c_int {
        if channelBits as libc::c_int & 1 as libc::c_int != 0 {
            channel = (*seqPlayer).channels[i as usize];
            (*channel).fontId = (*seqPlayer).defaultFont;
            (*channel).muteBehavior = (*seqPlayer).muteBehavior;
            (*channel).noteAllocPolicy = (*seqPlayer).noteAllocPolicy
        }
        channelBits =
            (channelBits as libc::c_int >> 1 as libc::c_int) as u16_0;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequencePlayerDisableChannels(mut seqPlayer:
                                                                    *mut SequencePlayer,
                                                                mut channelBitsUnused:
                                                                    u16_0) {
    let mut channel: *mut SequenceChannel = 0 as *mut SequenceChannel;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 0x10 as libc::c_int {
        channel = (*seqPlayer).channels[i as usize];
        if (channel as u32_0 !=
                &mut gAudioContext.sequenceChannelNone as *mut SequenceChannel
                    as u32_0) as libc::c_int == 1 as libc::c_int {
            AudioSeq_SequenceChannelDisable(channel);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequenceChannelEnable(mut seqPlayer:
                                                            *mut SequencePlayer,
                                                        mut channelIdx: u8_0,
                                                        mut script:
                                                            *mut libc::c_void) {
    let mut channel: *mut SequenceChannel =
        (*seqPlayer).channels[channelIdx as usize];
    let mut i: s32 = 0;
    (*channel).set_enabled(1 as libc::c_int as u8_0);
    (*channel).set_finished(0 as libc::c_int as u8_0);
    (*channel).scriptState.depth = 0 as libc::c_int as u8_0;
    (*channel).scriptState.pc = script as *mut u8_0;
    (*channel).delay = 0 as libc::c_int as u16_0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !(*channel).layers[i as usize].is_null() {
            AudioSeq_SeqLayerFree(channel, i);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequencePlayerDisableAsFinished(mut seqPlayer:
                                                                      *mut SequencePlayer) {
    (*seqPlayer).set_finished(1 as libc::c_int as u8_0);
    AudioSeq_SequencePlayerDisable(seqPlayer);
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequencePlayerDisable(mut seqPlayer:
                                                            *mut SequencePlayer) {
    AudioSeq_SequencePlayerDisableChannels(seqPlayer,
                                           0xffff as libc::c_int as u16_0);
    Audio_NotePoolClear(&mut (*seqPlayer).notePool);
    if (*seqPlayer).enabled() == 0 { return }
    (*seqPlayer).set_enabled(0 as libc::c_int as u8_0);
    (*seqPlayer).set_finished(1 as libc::c_int as u8_0);
    if AudioLoad_IsSeqLoadComplete((*seqPlayer).seqId as s32) != 0 {
        AudioLoad_SetSeqLoadStatus((*seqPlayer).seqId as s32,
                                   3 as libc::c_int);
    }
    if AudioLoad_IsFontLoadComplete((*seqPlayer).defaultFont as s32) != 0 {
        AudioLoad_SetFontLoadStatus((*seqPlayer).defaultFont as s32,
                                    4 as libc::c_int);
    }
    if (*seqPlayer).defaultFont as libc::c_int ==
           gAudioContext.fontCache.temporary.entries[0 as libc::c_int as
                                                         usize].id as
               libc::c_int {
        gAudioContext.fontCache.temporary.nextSide = 0 as libc::c_int as u32_0
    } else if (*seqPlayer).defaultFont as libc::c_int ==
                  gAudioContext.fontCache.temporary.entries[1 as libc::c_int
                                                                as usize].id
                      as libc::c_int {
        gAudioContext.fontCache.temporary.nextSide = 1 as libc::c_int as u32_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_AudioListPushBack(mut list:
                                                        *mut AudioListItem,
                                                    mut item:
                                                        *mut AudioListItem) {
    if (*item).prev.is_null() {
        (*(*list).prev).next = item;
        (*item).prev = (*list).prev;
        (*item).next = list;
        (*list).prev = item;
        (*list).u.count += 1;
        (*item).pool = (*list).pool
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_AudioListPopBack(mut list:
                                                       *mut AudioListItem)
 -> *mut libc::c_void {
    let mut item: *mut AudioListItem = (*list).prev;
    if item == list { return 0 as *mut libc::c_void }
    (*(*item).prev).next = list;
    (*list).prev = (*item).prev;
    (*item).prev = 0 as *mut AudioListItem;
    (*list).u.count -= 1;
    return (*item).u.value;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_InitLayerFreelist() {
    let mut i: s32 = 0;
    gAudioContext.layerFreeList.prev = &mut gAudioContext.layerFreeList;
    gAudioContext.layerFreeList.next = &mut gAudioContext.layerFreeList;
    gAudioContext.layerFreeList.u.count = 0 as libc::c_int;
    gAudioContext.layerFreeList.pool = 0 as *mut NotePool;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[SequenceLayer; 64]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<SequenceLayer>()
                                                   as libc::c_ulong) as s32 {
        gAudioContext.sequenceLayers[i as usize].listItem.u.value =
            &mut *gAudioContext.sequenceLayers.as_mut_ptr().offset(i as isize)
                as *mut SequenceLayer as *mut libc::c_void;
        gAudioContext.sequenceLayers[i as usize].listItem.prev =
            0 as *mut AudioListItem;
        AudioSeq_AudioListPushBack(&mut gAudioContext.layerFreeList,
                                   &mut (*gAudioContext.sequenceLayers.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).listItem);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_ScriptReadU8(mut state: *mut SeqScriptState)
 -> u8_0 {
    let fresh2 = (*state).pc;
    (*state).pc = (*state).pc.offset(1);
    return *fresh2;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_ScriptReadS16(mut state:
                                                    *mut SeqScriptState)
 -> s16 {
    let fresh3 = (*state).pc;
    (*state).pc = (*state).pc.offset(1);
    let mut ret: s16 = ((*fresh3 as libc::c_int) << 8 as libc::c_int) as s16;
    let fresh4 = (*state).pc;
    (*state).pc = (*state).pc.offset(1);
    ret = (*fresh4 as libc::c_int | ret as libc::c_int) as s16;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_ScriptReadCompressedU16(mut state:
                                                              *mut SeqScriptState)
 -> u16_0 {
    let fresh5 = (*state).pc;
    (*state).pc = (*state).pc.offset(1);
    let mut ret: u16_0 = *fresh5 as u16_0;
    if ret as libc::c_int & 0x80 as libc::c_int != 0 {
        ret =
            ((ret as libc::c_int) << 8 as libc::c_int & 0x7f00 as libc::c_int)
                as u16_0;
        let fresh6 = (*state).pc;
        (*state).pc = (*state).pc.offset(1);
        ret = (*fresh6 as libc::c_int | ret as libc::c_int) as u16_0
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerProcessScript(mut layer:
                                                            *mut SequenceLayer) {
    let mut val: s32 = 0;
    if (*layer).enabled() == 0 { return }
    if (*layer).delay as libc::c_int > 1 as libc::c_int {
        (*layer).delay -= 1;
        if (*layer).stopSomething() == 0 &&
               (*layer).delay as libc::c_int <=
                   (*layer).gateDelay as libc::c_int {
            Audio_SeqLayerNoteDecay(layer);
            (*layer).set_stopSomething(1 as libc::c_int as u8_0)
        }
        return
    }
    AudioSeq_SeqLayerProcessScriptStep1(layer);
    val = AudioSeq_SeqLayerProcessScriptStep2(layer);
    if val == -(1 as libc::c_int) { return }
    val = AudioSeq_SeqLayerProcessScriptStep3(layer, val);
    if val != -(1 as libc::c_int) {
        val = AudioSeq_SeqLayerProcessScriptStep4(layer, val)
    }
    if val != -(1 as libc::c_int) {
        AudioSeq_SeqLayerProcessScriptStep5(layer, val);
    }
    if (*layer).stopSomething() as libc::c_int == 1 as libc::c_int {
        if !(*layer).note.is_null() ||
               (*layer).continuousNotes() as libc::c_int != 0 {
            Audio_SeqLayerNoteDecay(layer);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerProcessScriptStep1(mut layer:
                                                                 *mut SequenceLayer) {
    if (*layer).continuousNotes() == 0 {
        Audio_SeqLayerNoteDecay(layer);
    } else if !(*layer).note.is_null() &&
                  (*(*layer).note).playbackState.wantedParentLayer == layer {
        Audio_SeqLayerNoteDecay(layer);
    }
    if (*layer).portamento.mode as libc::c_int & !(0x80 as libc::c_int) ==
           1 as libc::c_int ||
           (*layer).portamento.mode as libc::c_int & !(0x80 as libc::c_int) ==
               2 as libc::c_int {
        (*layer).portamento.mode = 0 as libc::c_int as u8_0
    }
    (*layer).set_notePropertiesNeedInit(1 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerProcessScriptStep5(mut layer:
                                                                 *mut SequenceLayer,
                                                             mut sameSound:
                                                                 s32) -> s32 {
    if (*layer).stopSomething() == 0 && !(*layer).sound.is_null() &&
           (*(*(*layer).sound).sample).codec() as libc::c_int ==
               CODEC_S16_INMEMORY as libc::c_int &&
           (*(*(*layer).sound).sample).medium() as libc::c_int !=
               MEDIUM_RAM as libc::c_int {
        (*layer).set_stopSomething(1 as libc::c_int as u8_0);
        return -(1 as libc::c_int)
    }
    if (*layer).continuousNotes() as libc::c_int == 1 as libc::c_int &&
           (*layer).bit1() as libc::c_int == 1 as libc::c_int {
        return 0 as libc::c_int
    }
    if (*layer).continuousNotes() as libc::c_int == 1 as libc::c_int &&
           !(*layer).note.is_null() && (*layer).bit3() as libc::c_int != 0 &&
           sameSound == 1 as libc::c_int &&
           (*(*layer).note).playbackState.parentLayer == layer {
        if (*layer).sound.is_null() {
            Audio_InitSyntheticWave((*layer).note, layer);
        }
    } else {
        if sameSound == 0 as libc::c_int { Audio_SeqLayerNoteDecay(layer); }
        (*layer).note = Audio_AllocNote(layer);
        if !(*layer).note.is_null() &&
               (*(*layer).note).playbackState.parentLayer == layer {
            Audio_NoteVibratoInit((*layer).note);
        }
    }
    if !(*layer).note.is_null() &&
           (*(*layer).note).playbackState.parentLayer == layer {
        let mut note: *mut Note = (*layer).note;
        Audio_NotePortamentoInit(note);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerProcessScriptStep2(mut layer:
                                                                 *mut SequenceLayer)
 -> s32 {
    let mut channel: *mut SequenceChannel = (*layer).channel;
    let mut state: *mut SeqScriptState = &mut (*layer).scriptState;
    let mut seqPlayer: *mut SequencePlayer = (*channel).seqPlayer;
    let mut sp3A: u16_0 = 0;
    let mut cmd: u8_0 = 0;
    loop  {
        cmd = AudioSeq_ScriptReadU8(state);
        if (cmd as libc::c_int) < 0xc1 as libc::c_int { return cmd as s32 }
        if cmd as libc::c_int >= 0xf2 as libc::c_int {
            let mut arg: u16_0 =
                AudioSeq_GetScriptControlFlowArgument(state, cmd);
            if AudioSeq_HandleScriptFlowControl(seqPlayer, state, cmd as s32,
                                                arg as s32) ==
                   0 as libc::c_int {
                continue ;
            }
            AudioSeq_SeqLayerDisable(layer);
            return -(1 as libc::c_int)
        } else {
            let mut current_block_63: u64;
            match cmd as libc::c_int {
                193 => { current_block_63 = 4166486009154926805; }
                202 => {
                    // layer_setpan
                    current_block_63 = 4166486009154926805;
                }
                201 => {
                    // layer_setshortnotegatetime
                    current_block_63 = 26972500619410423;
                }
                194 => { current_block_63 = 26972500619410423; }
                196 | 197 => {
                    // layer_continuousnoteson
                    // layer_continuousnotesoff
                    if cmd as libc::c_int == 0xc4 as libc::c_int {
                        (*layer).set_continuousNotes(1 as libc::c_int as u8_0)
                    } else {
                        (*layer).set_continuousNotes(0 as libc::c_int as u8_0)
                    }
                    (*layer).set_bit1(0 as libc::c_int as u8_0);
                    Audio_SeqLayerNoteDecay(layer);
                    current_block_63 = 8716029205547827362;
                }
                195 => {
                    // layer_setshortnotedefaultdelay
                    sp3A = AudioSeq_ScriptReadCompressedU16(state);
                    (*layer).shortNoteDefaultDelay = sp3A as s16;
                    current_block_63 = 8716029205547827362;
                }
                198 => {
                    // layer_setinstr
                    cmd = AudioSeq_ScriptReadU8(state);
                    if cmd as libc::c_int >= 0x7e as libc::c_int {
                        if cmd as libc::c_int == 0x7e as libc::c_int {
                            (*layer).instOrWave = 1 as libc::c_int as u8_0
                        } else if cmd as libc::c_int == 0x7f as libc::c_int {
                            (*layer).instOrWave = 0 as libc::c_int as u8_0
                        } else {
                            (*layer).instOrWave = cmd;
                            (*layer).instrument = 0 as *mut Instrument
                        }
                        if cmd as libc::c_int == 0xff as libc::c_int {
                            (*layer).adsr.releaseRate =
                                0 as libc::c_int as u8_0
                        }
                    } else {
                        (*layer).instOrWave =
                            AudioSeq_GetInstrument(channel, cmd,
                                                   &mut (*layer).instrument,
                                                   &mut (*layer).adsr);
                        if (*layer).instOrWave as libc::c_int ==
                               0 as libc::c_int {
                            (*layer).instOrWave = 0xff as libc::c_int as u8_0
                        }
                    }
                    current_block_63 = 8716029205547827362;
                }
                199 => {
                    // layer_portamento
                    (*layer).portamento.mode = AudioSeq_ScriptReadU8(state);
                    cmd = AudioSeq_ScriptReadU8(state);
                    cmd =
                        (cmd as libc::c_int +
                             (*channel).transposition as libc::c_int) as u8_0;
                    cmd =
                        (cmd as libc::c_int +
                             (*layer).transposition as libc::c_int) as u8_0;
                    cmd =
                        (cmd as libc::c_int +
                             (*seqPlayer).transposition as libc::c_int) as
                            u8_0;
                    if cmd as libc::c_int >= 0x80 as libc::c_int {
                        cmd = 0 as libc::c_int as u8_0
                    }
                    (*layer).portamentoTargetNote = cmd;
                    // If special, the next param is u8 instead of var
                    if (*layer).portamento.mode as libc::c_int &
                           0x80 as libc::c_int != 0 {
                        let fresh9 = (*state).pc;
                        (*state).pc = (*state).pc.offset(1);
                        (*layer).portamentoTime = *fresh9 as u16_0
                    } else {
                        sp3A = AudioSeq_ScriptReadCompressedU16(state);
                        (*layer).portamentoTime = sp3A
                    }
                    current_block_63 = 8716029205547827362;
                }
                200 => {
                    // layer_disableportamento
                    (*layer).portamento.mode = 0 as libc::c_int as u8_0;
                    current_block_63 = 8716029205547827362;
                }
                203 => {
                    sp3A = AudioSeq_ScriptReadS16(state) as u16_0;
                    (*layer).adsr.envelope =
                        (*seqPlayer).seqData.offset(sp3A as libc::c_int as
                                                        isize) as
                            *mut AdsrEnvelope;
                    current_block_63 = 13546961742289887181;
                }
                207 => { current_block_63 = 13546961742289887181; }
                204 => {
                    (*layer).set_ignoreDrumPan(1 as libc::c_int as u8_0);
                    current_block_63 = 8716029205547827362;
                }
                205 => {
                    (*layer).stereo.asByte = AudioSeq_ScriptReadU8(state);
                    current_block_63 = 8716029205547827362;
                }
                206 => {
                    let mut tempByte_1: u8_0 = AudioSeq_ScriptReadU8(state);
                    (*layer).unk_34 =
                        gBendPitchTwoSemitonesFrequencies[(tempByte_1 as
                                                               libc::c_int +
                                                               0x80 as
                                                                   libc::c_int
                                                               &
                                                               0xff as
                                                                   libc::c_int)
                                                              as usize];
                    current_block_63 = 8716029205547827362;
                }
                _ => {
                    match cmd as libc::c_int & 0xf0 as libc::c_int {
                        208 => {
                            // layer_setshortnotevelocityfromtable
                            sp3A =
                                *(*seqPlayer).shortNoteVelocityTable.offset((cmd
                                                                                 as
                                                                                 libc::c_int
                                                                                 &
                                                                                 0xf
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                isize)
                                    as u16_0;
                            (*layer).velocitySquare =
                                (sp3A as libc::c_int * sp3A as libc::c_int) as
                                    f32_0 / 16129.0f32
                        }
                        224 => {
                            // layer_setshortnotegatetimefromtable
                            (*layer).gateTime =
                                *(*seqPlayer).shortNoteGateTimeTable.offset((cmd
                                                                                 as
                                                                                 libc::c_int
                                                                                 &
                                                                                 0xf
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                isize)
                        }
                        _ => { }
                    }
                    current_block_63 = 8716029205547827362;
                }
            }
            match current_block_63 {
                26972500619410423 =>
                // layer_transpose; set transposition in semitones
                {
                    let fresh8 = (*state).pc;
                    (*state).pc = (*state).pc.offset(1);
                    let mut tempByte_0: u8_0 = *fresh8;
                    if cmd as libc::c_int == 0xc9 as libc::c_int {
                        (*layer).gateTime = tempByte_0
                    } else { (*layer).transposition = tempByte_0 as s16 }
                }
                4166486009154926805 =>
                // layer_setshortnotevelocity
                {
                    let fresh7 = (*state).pc;
                    (*state).pc = (*state).pc.offset(1);
                    let mut tempByte: u8_0 = *fresh7;
                    if cmd as libc::c_int == 0xc1 as libc::c_int {
                        (*layer).velocitySquare =
                            (tempByte as libc::c_int *
                                 tempByte as libc::c_int) as f32_0 /
                                16129.0f32
                    } else { (*layer).pan = tempByte }
                }
                13546961742289887181 =>
                // fallthrough
                {
                    (*layer).adsr.releaseRate = AudioSeq_ScriptReadU8(state)
                }
                _ => { }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerProcessScriptStep4(mut layer:
                                                                 *mut SequenceLayer,
                                                             mut cmd: s32)
 -> s32 {
    let mut sameSound: s32 = 1 as libc::c_int;
    let mut instOrWave: s32 = 0;
    let mut speed: s32 = 0;
    let mut temp_f14: f32_0 = 0.;
    let mut temp_f2: f32_0 = 0.;
    let mut portamento: *mut Portamento = 0 as *mut Portamento;
    let mut freqScale: f32_0 = 0.;
    let mut freqScale2: f32_0 = 0.;
    let mut sound: *mut SoundFontSound = 0 as *mut SoundFontSound;
    let mut instrument: *mut Instrument = 0 as *mut Instrument;
    let mut drum: *mut Drum = 0 as *mut Drum;
    let mut pad: s32 = 0;
    let mut channel: *mut SequenceChannel = 0 as *mut SequenceChannel;
    let mut seqPlayer: *mut SequencePlayer = 0 as *mut SequencePlayer;
    let mut semitone: u8_0 = cmd as u8_0;
    let mut sfxId: u16_0 = 0;
    let mut semitone2: s32 = 0;
    let mut vel: s32 = 0;
    let mut time: f32_0 = 0.;
    let mut tuning: f32_0 = 0.;
    instOrWave = (*layer).instOrWave as s32;
    channel = (*layer).channel;
    seqPlayer = (*channel).seqPlayer;
    if instOrWave == 0xff as libc::c_int {
        if (*channel).hasInstrument() == 0 { return -(1 as libc::c_int) }
        instOrWave = (*channel).instOrWave as s32
    }
    match instOrWave {
        0 => {
            semitone =
                (semitone as libc::c_int +
                     ((*channel).transposition as libc::c_int +
                          (*layer).transposition as libc::c_int)) as u8_0;
            (*layer).semitone = semitone;
            drum = Audio_GetDrum((*channel).fontId as s32, semitone as s32);
            if drum.is_null() {
                (*layer).set_stopSomething(1 as libc::c_int as u8_0);
                (*layer).delay2 = (*layer).delay;
                return -(1 as libc::c_int)
            }
            sound = &mut (*drum).sound;
            (*layer).adsr.envelope = (*drum).envelope;
            (*layer).adsr.releaseRate = (*drum).releaseRate;
            if (*layer).ignoreDrumPan() == 0 { (*layer).pan = (*drum).pan }
            (*layer).sound = sound;
            (*layer).freqScale = (*sound).tuning
        }
        1 => {
            (*layer).semitone = semitone;
            sfxId =
                ((((*layer).transposition as libc::c_int) << 6 as libc::c_int)
                     + semitone as libc::c_int) as u16_0;
            sound = Audio_GetSfx((*channel).fontId as s32, sfxId as s32);
            if sound.is_null() {
                (*layer).set_stopSomething(1 as libc::c_int as u8_0);
                (*layer).delay2 =
                    ((*layer).delay as libc::c_int + 1 as libc::c_int) as s16;
                return -(1 as libc::c_int)
            }
            (*layer).sound = sound;
            (*layer).freqScale = (*sound).tuning
        }
        _ => {
            semitone =
                (semitone as libc::c_int +
                     ((*seqPlayer).transposition as libc::c_int +
                          (*channel).transposition as libc::c_int +
                          (*layer).transposition as libc::c_int)) as u8_0;
            semitone2 = semitone as s32;
            (*layer).semitone = semitone;
            if semitone as libc::c_int >= 0x80 as libc::c_int {
                (*layer).set_stopSomething(1 as libc::c_int as u8_0);
                return -(1 as libc::c_int)
            }
            if (*layer).instOrWave as libc::c_int == 0xff as libc::c_int {
                instrument = (*channel).instrument
            } else { instrument = (*layer).instrument }
            if (*layer).portamento.mode as libc::c_int != 0 as libc::c_int {
                portamento = &mut (*layer).portamento;
                vel =
                    if semitone as libc::c_int >
                           (*layer).portamentoTargetNote as libc::c_int {
                        semitone as libc::c_int
                    } else { (*layer).portamentoTargetNote as libc::c_int };
                if !instrument.is_null() {
                    sound = Audio_InstrumentGetSound(instrument, vel);
                    sameSound = ((*layer).sound == sound) as libc::c_int;
                    (*layer).sound = sound;
                    tuning = (*sound).tuning
                } else {
                    (*layer).sound = 0 as *mut SoundFontSound;
                    tuning = 1.0f32;
                    if instOrWave >= 0xc0 as libc::c_int {
                        (*layer).sound =
                            &mut (*gAudioContext.synthesisReverbs.as_mut_ptr().offset((instOrWave
                                                                                           -
                                                                                           0xc0
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          isize)).sound
                    }
                }
                temp_f2 =
                    *gNoteFrequencies.as_mut_ptr().offset(semitone2 as isize)
                        * tuning;
                temp_f14 =
                    *gNoteFrequencies.as_mut_ptr().offset((*layer).portamentoTargetNote
                                                              as isize) *
                        tuning;
                match (*portamento).mode as libc::c_int &
                          !(0x80 as libc::c_int) {
                    1 | 3 | 5 => {
                        freqScale2 = temp_f2;
                        freqScale = temp_f14
                    }
                    2 | 4 => { freqScale = temp_f2; freqScale2 = temp_f14 }
                    _ => { freqScale = temp_f2; freqScale2 = temp_f2 }
                }
                (*portamento).extent = freqScale2 / freqScale - 1.0f32;
                if (*portamento).mode as libc::c_int & 0x80 as libc::c_int !=
                       0 {
                    speed =
                        (*seqPlayer).tempo as libc::c_int *
                            0x8000 as libc::c_int /
                            gAudioContext.tempoInternalToExternal as
                                libc::c_int;
                    if (*layer).delay as libc::c_int != 0 as libc::c_int {
                        speed =
                            speed * 0x100 as libc::c_int /
                                ((*layer).delay as libc::c_int *
                                     (*layer).portamentoTime as libc::c_int)
                    }
                } else {
                    speed =
                        0x20000 as libc::c_int /
                            ((*layer).portamentoTime as libc::c_int *
                                 gAudioContext.audioBufferParameters.updatesPerFrame
                                     as libc::c_int)
                }
                if speed >= 0x7fff as libc::c_int {
                    speed = 0x7fff as libc::c_int
                } else if speed < 1 as libc::c_int {
                    speed = 1 as libc::c_int
                }
                (*portamento).speed = speed as u16_0;
                (*portamento).cur = 0 as libc::c_int as u16_0;
                (*layer).freqScale = freqScale;
                if (*portamento).mode as libc::c_int & !(0x80 as libc::c_int)
                       == 5 as libc::c_int {
                    (*layer).portamentoTargetNote = semitone
                }
            } else if !instrument.is_null() {
                sound = Audio_InstrumentGetSound(instrument, semitone as s32);
                sameSound = (sound == (*layer).sound) as libc::c_int;
                (*layer).sound = sound;
                (*layer).freqScale =
                    *gNoteFrequencies.as_mut_ptr().offset(semitone2 as isize)
                        * (*sound).tuning
            } else {
                (*layer).sound = 0 as *mut SoundFontSound;
                (*layer).freqScale =
                    *gNoteFrequencies.as_mut_ptr().offset(semitone2 as isize);
                if instOrWave >= 0xc0 as libc::c_int {
                    (*layer).sound =
                        &mut (*gAudioContext.synthesisReverbs.as_mut_ptr().offset((instOrWave
                                                                                       -
                                                                                       0xc0
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      isize)).sound
                }
            }
        }
    }
    (*layer).delay2 = (*layer).delay;
    (*layer).freqScale *= (*layer).unk_34;
    if (*layer).delay as libc::c_int == 0 as libc::c_int {
        if !(*layer).sound.is_null() {
            time = (*(*(*(*layer).sound).sample).loop_0).end as f32_0
        } else { time = 0.0f32 }
        time *= (*seqPlayer).tempo as libc::c_int as libc::c_float;
        time *= gAudioContext.unk_2870;
        time /= (*layer).freqScale;
        if time > 32766.0f32 { time = 32766.0f32 }
        (*layer).gateDelay = 0 as libc::c_int as s16;
        (*layer).delay =
            (time as s32 as u16_0 as libc::c_int + 1 as libc::c_int) as s16;
        if (*layer).portamento.mode as libc::c_int != 0 as libc::c_int {
            // (It's a bit unclear if 'portamento' has actually always been
            // set when this is reached...)
            if (*portamento).mode as libc::c_int & 0x80 as libc::c_int != 0 {
                let mut speed2: s32 = 0;
                speed2 =
                    (*seqPlayer).tempo as libc::c_int * 0x8000 as libc::c_int
                        /
                        gAudioContext.tempoInternalToExternal as libc::c_int;
                speed2 =
                    speed2 * 0x100 as libc::c_int /
                        ((*layer).delay as libc::c_int *
                             (*layer).portamentoTime as libc::c_int);
                if speed2 >= 0x7fff as libc::c_int {
                    speed2 = 0x7fff as libc::c_int
                } else if speed2 < 1 as libc::c_int {
                    speed2 = 1 as libc::c_int
                }
                (*portamento).speed = speed2 as u16_0
            }
        }
    }
    return sameSound;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SeqLayerProcessScriptStep3(mut layer:
                                                                 *mut SequenceLayer,
                                                             mut cmd: s32)
 -> s32 {
    let mut state: *mut SeqScriptState = &mut (*layer).scriptState;
    let mut delay: u16_0 = 0;
    let mut velocity: s32 = 0;
    let mut channel: *mut SequenceChannel = (*layer).channel;
    let mut seqPlayer: *mut SequencePlayer = (*channel).seqPlayer;
    let mut intDelta: s32 = 0;
    let mut floatDelta: f32_0 = 0.;
    if cmd == 0xc0 as libc::c_int {
        (*layer).delay = AudioSeq_ScriptReadCompressedU16(state) as s16;
        (*layer).set_stopSomething(1 as libc::c_int as u8_0);
        (*layer).set_bit1(0 as libc::c_int as u8_0);
        return -(1 as libc::c_int)
    }
    (*layer).set_stopSomething(0 as libc::c_int as u8_0);
    if (*channel).largeNotes() as libc::c_int == 1 as libc::c_int {
        match cmd & 0xc0 as libc::c_int {
            0 => {
                delay = AudioSeq_ScriptReadCompressedU16(state);
                let fresh10 = (*state).pc;
                (*state).pc = (*state).pc.offset(1);
                velocity = *fresh10 as s32;
                let fresh11 = (*state).pc;
                (*state).pc = (*state).pc.offset(1);
                (*layer).gateTime = *fresh11;
                (*layer).lastDelay = delay as s16
            }
            64 => {
                delay = AudioSeq_ScriptReadCompressedU16(state);
                let fresh12 = (*state).pc;
                (*state).pc = (*state).pc.offset(1);
                velocity = *fresh12 as s32;
                (*layer).gateTime = 0 as libc::c_int as u8_0;
                (*layer).lastDelay = delay as s16
            }
            128 => {
                delay = (*layer).lastDelay as u16_0;
                let fresh13 = (*state).pc;
                (*state).pc = (*state).pc.offset(1);
                velocity = *fresh13 as s32;
                let fresh14 = (*state).pc;
                (*state).pc = (*state).pc.offset(1);
                (*layer).gateTime = *fresh14
            }
            _ => { }
        }
        if velocity > 0x7f as libc::c_int || velocity < 0 as libc::c_int {
            velocity = 0x7f as libc::c_int
        }
        (*layer).velocitySquare =
            velocity as f32_0 * velocity as f32_0 / 16129.0f32;
        cmd -= cmd & 0xc0 as libc::c_int
    } else {
        match cmd & 0xc0 as libc::c_int {
            0 => {
                delay = AudioSeq_ScriptReadCompressedU16(state);
                (*layer).lastDelay = delay as s16
            }
            64 => { delay = (*layer).shortNoteDefaultDelay as u16_0 }
            128 => { delay = (*layer).lastDelay as u16_0 }
            _ => { }
        }
        cmd -= cmd & 0xc0 as libc::c_int
    }
    if (*channel).velocityRandomVariance as libc::c_int != 0 as libc::c_int {
        floatDelta =
            (*layer).velocitySquare *
                gAudioContext.audioRandom.wrapping_rem((*channel).velocityRandomVariance
                                                           as libc::c_uint) as
                    f32_0 / 100.0f32;
        if gAudioContext.audioRandom & 0x8000 as libc::c_int as libc::c_uint
               != 0 as libc::c_int as libc::c_uint {
            floatDelta = -floatDelta
        }
        (*layer).velocitySquare2 = (*layer).velocitySquare + floatDelta;
        if (*layer).velocitySquare2 < 0.0f32 {
            (*layer).velocitySquare2 = 0.0f32
        } else if (*layer).velocitySquare2 > 1.0f32 {
            (*layer).velocitySquare2 = 1.0f32
        }
    } else { (*layer).velocitySquare2 = (*layer).velocitySquare }
    (*layer).delay = delay as s16;
    (*layer).gateDelay =
        ((*layer).gateTime as libc::c_int * delay as libc::c_int >>
             8 as libc::c_int) as s16;
    if (*channel).gateTimeRandomVariance as libc::c_int != 0 as libc::c_int {
        // ! @bug should probably be gateTimeRandomVariance
        intDelta =
            ((*layer).gateDelay as
                 libc::c_uint).wrapping_mul(gAudioContext.audioRandom.wrapping_rem((*channel).velocityRandomVariance
                                                                                       as
                                                                                       libc::c_uint)).wrapping_div(100
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)
                as s32;
        if gAudioContext.audioRandom & 0x4000 as libc::c_int as libc::c_uint
               != 0 as libc::c_int as libc::c_uint {
            intDelta = -intDelta
        }
        (*layer).gateDelay =
            ((*layer).gateDelay as libc::c_int + intDelta) as s16;
        if ((*layer).gateDelay as libc::c_int) < 0 as libc::c_int {
            (*layer).gateDelay = 0 as libc::c_int as s16
        } else if (*layer).gateDelay as libc::c_int >
                      (*layer).delay as libc::c_int {
            (*layer).gateDelay = (*layer).delay
        }
    }
    if (*seqPlayer).muted() as libc::c_int != 0 &&
           (*channel).muteBehavior as libc::c_int &
               (0x40 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int
           || (*channel).stopSomething2() as libc::c_int != 0 {
        (*layer).set_stopSomething(1 as libc::c_int as u8_0);
        return -(1 as libc::c_int)
    }
    if (*seqPlayer).skipTicks != 0 as libc::c_int {
        (*layer).set_stopSomething(1 as libc::c_int as u8_0);
        return -(1 as libc::c_int)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SetChannelPriorities(mut channel:
                                                           *mut SequenceChannel,
                                                       mut arg1: u8_0) {
    if arg1 as libc::c_int & 0xf as libc::c_int != 0 as libc::c_int {
        (*channel).notePriority =
            (arg1 as libc::c_int & 0xf as libc::c_int) as u8_0
    }
    arg1 = (arg1 as libc::c_int >> 4 as libc::c_int) as u8_0;
    if arg1 as libc::c_int != 0 as libc::c_int {
        (*channel).someOtherPriority = arg1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_GetInstrument(mut channel:
                                                    *mut SequenceChannel,
                                                mut instId: u8_0,
                                                mut instOut:
                                                    *mut *mut Instrument,
                                                mut adsr: *mut AdsrSettings)
 -> u8_0 {
    let mut inst: *mut Instrument =
        Audio_GetInstrumentInner((*channel).fontId as s32, instId as s32);
    if inst.is_null() {
        *instOut = 0 as *mut Instrument;
        return 0 as libc::c_int as u8_0
    }
    (*adsr).envelope = (*inst).envelope;
    (*adsr).releaseRate = (*inst).releaseRate;
    *instOut = inst;
    instId = (instId as libc::c_int + 2 as libc::c_int) as u8_0;
    return instId;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SetInstrument(mut channel:
                                                    *mut SequenceChannel,
                                                mut instId: u8_0) {
    if instId as libc::c_int >= 0x80 as libc::c_int {
        (*channel).instOrWave = instId as s16;
        (*channel).instrument = 0 as *mut Instrument
    } else if instId as libc::c_int == 0x7f as libc::c_int {
        (*channel).instOrWave = 0 as libc::c_int as s16;
        (*channel).instrument = 1 as libc::c_int as *mut Instrument
    } else if instId as libc::c_int == 0x7e as libc::c_int {
        (*channel).instOrWave = 1 as libc::c_int as s16;
        (*channel).instrument = 2 as libc::c_int as *mut Instrument
    } else {
        (*channel).instOrWave =
            AudioSeq_GetInstrument(channel, instId,
                                   &mut (*channel).instrument,
                                   &mut (*channel).adsr) as s16;
        if (*channel).instOrWave as libc::c_int == 0 as libc::c_int {
            (*channel).set_hasInstrument(0 as libc::c_int as u8_0);
            return
        }
    }
    (*channel).set_hasInstrument(1 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequenceChannelSetVolume(mut channel:
                                                               *mut SequenceChannel,
                                                           mut volume: u8_0) {
    (*channel).volume = volume as s32 as f32_0 / 127.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequenceChannelProcessScript(mut channel:
                                                                   *mut SequenceChannel) {
    let mut i: s32 = 0;
    let mut data: *mut u8_0 = 0 as *mut u8_0;
    let mut test: *mut u8_0 = 0 as *mut u8_0;
    let mut seqPlayer: *mut SequencePlayer = 0 as *mut SequencePlayer;
    if !((*channel).stopScript() != 0) {
        seqPlayer = (*channel).seqPlayer;
        if (*seqPlayer).muted() as libc::c_int != 0 &&
               (*channel).muteBehavior as libc::c_int & 0x80 as libc::c_int !=
                   0 {
            return
        }
        if (*channel).delay as libc::c_int >= 2 as libc::c_int {
            (*channel).delay = (*channel).delay.wrapping_sub(1)
        } else {
            loop  {
                let mut scriptState: *mut SeqScriptState =
                    &mut (*channel).scriptState;
                let mut param: s32 = 0;
                let mut pad1: s16 = 0;
                let mut offset: u16_0 = 0;
                let mut parameters: [u32_0; 3] = [0; 3];
                let mut signedParam: s8 = 0;
                let mut command: u8_0 = AudioSeq_ScriptReadU8(scriptState);
                let mut lowBits: u8_0 = 0;
                let mut highBits: u8_0 = 0;
                let mut result: s32 = 0;
                let mut pad2: s32 = 0;
                if command as libc::c_int >= 0xb0 as libc::c_int {
                    highBits =
                        D_80130520[(command as s32 - 0xb0 as libc::c_int) as
                                       usize];
                    lowBits =
                        (highBits as libc::c_int & 3 as libc::c_int) as u8_0;
                    i = 0 as libc::c_int;
                    while i < lowBits as libc::c_int {
                        if highBits as libc::c_int & 0x80 as libc::c_int == 0
                           {
                            parameters[i as usize] =
                                AudioSeq_ScriptReadU8(scriptState) as u32_0
                        } else {
                            parameters[i as usize] =
                                AudioSeq_ScriptReadS16(scriptState) as u32_0
                        }
                        i += 1;
                        highBits =
                            ((highBits as libc::c_int) << 1 as libc::c_int) as
                                u8_0
                    }
                    if command as libc::c_int >= 0xf2 as libc::c_int {
                        result =
                            AudioSeq_HandleScriptFlowControl(seqPlayer,
                                                             scriptState,
                                                             command as s32,
                                                             parameters[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                                 as s32);
                        if !(result != 0 as libc::c_int) { continue ; }
                        if result == -(1 as libc::c_int) {
                            AudioSeq_SequenceChannelDisable(channel);
                        } else { (*channel).delay = result as u16_0 }
                        break ;
                    } else {
                        match command as libc::c_int {
                            234 => {
                                (*channel).set_stopScript(1 as libc::c_int as
                                                              u8_0);
                                break ;
                            }
                            241 => {
                                Audio_NotePoolClear(&mut (*channel).notePool);
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                Audio_NotePoolFill(&mut (*channel).notePool,
                                                   command as s32);
                                continue ;
                            }
                            240 => {
                                Audio_NotePoolClear(&mut (*channel).notePool);
                                continue ;
                            }
                            194 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                (*channel).dynTable =
                                    &mut *(*seqPlayer).seqData.offset(offset
                                                                          as
                                                                          isize)
                                        as *mut u8_0 as *mut libc::c_void as
                                        *mut [[u8_0; 2]; 0];
                                continue ;
                            }
                            197 => {
                                if (*scriptState).value as libc::c_int !=
                                       -(1 as libc::c_int) {
                                    data =
                                        (*(*(*channel).dynTable).as_mut_ptr().offset((*scriptState).value
                                                                                         as
                                                                                         isize)).as_mut_ptr();
                                    offset =
                                        (((*data.offset(0 as libc::c_int as
                                                            isize) as
                                               libc::c_int) <<
                                              8 as libc::c_int) +
                                             *data.offset(1 as libc::c_int as
                                                              isize) as
                                                 libc::c_int) as u16_0;
                                    (*channel).dynTable =
                                        &mut *(*seqPlayer).seqData.offset(offset
                                                                              as
                                                                              isize)
                                            as *mut u8_0 as *mut libc::c_void
                                            as *mut [[u8_0; 2]; 0]
                                }
                                continue ;
                            }
                            235 => {
                                result =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0 as s32;
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                if (*seqPlayer).defaultFont as libc::c_int !=
                                       0xff as libc::c_int {
                                    offset =
                                        *(gAudioContext.sequenceFontTable as
                                              *mut u16_0).offset((*seqPlayer).seqId
                                                                     as
                                                                     isize);
                                    lowBits =
                                        *gAudioContext.sequenceFontTable.offset(offset
                                                                                    as
                                                                                    isize);
                                    command =
                                        *gAudioContext.sequenceFontTable.offset((offset
                                                                                     as
                                                                                     libc::c_int
                                                                                     +
                                                                                     lowBits
                                                                                         as
                                                                                         libc::c_int
                                                                                     -
                                                                                     result)
                                                                                    as
                                                                                    isize)
                                }
                                if !AudioHeap_SearchCaches(FONT_TABLE as
                                                               libc::c_int,
                                                           CACHE_EITHER as
                                                               libc::c_int,
                                                           command as
                                                               s32).is_null()
                                   {
                                    (*channel).fontId = command
                                }
                                parameters[0 as libc::c_int as usize] =
                                    parameters[1 as libc::c_int as usize]
                            }
                            193 => { }
                            195 => {
                                (*channel).set_largeNotes(0 as libc::c_int as
                                                              u8_0);
                                continue ;
                            }
                            196 => {
                                (*channel).set_largeNotes(1 as libc::c_int as
                                                              u8_0);
                                continue ;
                            }
                            223 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                AudioSeq_SequenceChannelSetVolume(channel,
                                                                  command);
                                (*channel).changes.s.set_volume(1 as
                                                                    libc::c_int
                                                                    as u8_0);
                                continue ;
                            }
                            224 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).volumeScale =
                                    command as s32 as f32_0 / 128.0f32;
                                (*channel).changes.s.set_volume(1 as
                                                                    libc::c_int
                                                                    as u8_0);
                                continue ;
                            }
                            222 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                (*channel).freqScale =
                                    offset as s32 as f32_0 / 32768.0f32;
                                (*channel).changes.s.set_freqScale(1 as
                                                                       libc::c_int
                                                                       as
                                                                       u8_0);
                                continue ;
                            }
                            211 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                command =
                                    (command as libc::c_int +
                                         0x80 as libc::c_int) as u8_0;
                                (*channel).freqScale =
                                    gBendPitchOneOctaveFrequencies[command as
                                                                       usize];
                                (*channel).changes.s.set_freqScale(1 as
                                                                       libc::c_int
                                                                       as
                                                                       u8_0);
                                continue ;
                            }
                            238 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                command =
                                    (command as libc::c_int +
                                         0x80 as libc::c_int) as u8_0;
                                (*channel).freqScale =
                                    gBendPitchTwoSemitonesFrequencies[command
                                                                          as
                                                                          usize];
                                (*channel).changes.s.set_freqScale(1 as
                                                                       libc::c_int
                                                                       as
                                                                       u8_0);
                                continue ;
                            }
                            221 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).newPan = command;
                                (*channel).changes.s.set_pan(1 as libc::c_int
                                                                 as u8_0);
                                continue ;
                            }
                            220 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).panChannelWeight = command;
                                (*channel).changes.s.set_pan(1 as libc::c_int
                                                                 as u8_0);
                                continue ;
                            }
                            219 => {
                                signedParam =
                                    parameters[0 as libc::c_int as usize] as
                                        s8;
                                (*channel).transposition = signedParam as s16;
                                continue ;
                            }
                            218 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                (*channel).adsr.envelope =
                                    &mut *(*seqPlayer).seqData.offset(offset
                                                                          as
                                                                          isize)
                                        as *mut u8_0 as *mut AdsrEnvelope;
                                continue ;
                            }
                            217 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).adsr.releaseRate = command;
                                continue ;
                            }
                            216 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoExtentTarget =
                                    (command as libc::c_int *
                                         8 as libc::c_int) as u16_0;
                                (*channel).vibratoExtentStart =
                                    0 as libc::c_int as u16_0;
                                (*channel).vibratoExtentChangeDelay =
                                    0 as libc::c_int as u16_0;
                                continue ;
                            }
                            215 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoRateChangeDelay =
                                    0 as libc::c_int as u16_0;
                                (*channel).vibratoRateTarget =
                                    (command as libc::c_int *
                                         32 as libc::c_int) as u16_0;
                                (*channel).vibratoRateStart =
                                    (command as libc::c_int *
                                         32 as libc::c_int) as u16_0;
                                continue ;
                            }
                            226 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoExtentStart =
                                    (command as libc::c_int *
                                         8 as libc::c_int) as u16_0;
                                command =
                                    parameters[1 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoExtentTarget =
                                    (command as libc::c_int *
                                         8 as libc::c_int) as u16_0;
                                command =
                                    parameters[2 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoExtentChangeDelay =
                                    (command as libc::c_int *
                                         16 as libc::c_int) as u16_0;
                                continue ;
                            }
                            225 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoRateStart =
                                    (command as libc::c_int *
                                         32 as libc::c_int) as u16_0;
                                command =
                                    parameters[1 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoRateTarget =
                                    (command as libc::c_int *
                                         32 as libc::c_int) as u16_0;
                                command =
                                    parameters[2 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoRateChangeDelay =
                                    (command as libc::c_int *
                                         16 as libc::c_int) as u16_0;
                                continue ;
                            }
                            227 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).vibratoDelay =
                                    (command as libc::c_int *
                                         16 as libc::c_int) as u16_0;
                                continue ;
                            }
                            212 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).reverb = command;
                                continue ;
                            }
                            198 => {
                                result =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0 as s32;
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                if (*seqPlayer).defaultFont as libc::c_int !=
                                       0xff as libc::c_int {
                                    offset =
                                        *(gAudioContext.sequenceFontTable as
                                              *mut u16_0).offset((*seqPlayer).seqId
                                                                     as
                                                                     isize);
                                    lowBits =
                                        *gAudioContext.sequenceFontTable.offset(offset
                                                                                    as
                                                                                    isize);
                                    command =
                                        *gAudioContext.sequenceFontTable.offset((offset
                                                                                     as
                                                                                     libc::c_int
                                                                                     +
                                                                                     lowBits
                                                                                         as
                                                                                         libc::c_int
                                                                                     -
                                                                                     result)
                                                                                    as
                                                                                    isize)
                                }
                                if !AudioHeap_SearchCaches(FONT_TABLE as
                                                               libc::c_int,
                                                           CACHE_EITHER as
                                                               libc::c_int,
                                                           command as
                                                               s32).is_null()
                                   {
                                    (*channel).fontId = command
                                }
                                continue ;
                            }
                            199 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                offset =
                                    parameters[1 as libc::c_int as usize] as
                                        u16_0;
                                test =
                                    &mut *(*seqPlayer).seqData.offset(offset
                                                                          as
                                                                          isize)
                                        as *mut u8_0;
                                *test.offset(0 as libc::c_int as isize) =
                                    ((*scriptState).value as u8_0 as
                                         libc::c_int + command as libc::c_int)
                                        as u8_0;
                                continue ;
                            }
                            200 | 204 | 201 => {
                                signedParam =
                                    parameters[0 as libc::c_int as usize] as
                                        s8;
                                if command as libc::c_int ==
                                       0xc8 as libc::c_int {
                                    (*scriptState).value =
                                        ((*scriptState).value as libc::c_int -
                                             signedParam as libc::c_int) as s8
                                } else if command as libc::c_int ==
                                              0xcc as libc::c_int {
                                    (*scriptState).value = signedParam
                                } else {
                                    (*scriptState).value =
                                        ((*scriptState).value as libc::c_int &
                                             signedParam as libc::c_int) as s8
                                }
                                continue ;
                            }
                            205 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                AudioSeq_SequenceChannelDisable((*seqPlayer).channels[command
                                                                                          as
                                                                                          usize]);
                                continue ;
                            }
                            202 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).muteBehavior = command;
                                (*channel).changes.s.set_volume(1 as
                                                                    libc::c_int
                                                                    as u8_0);
                                continue ;
                            }
                            203 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                (*scriptState).value =
                                    *(*seqPlayer).seqData.offset((offset as
                                                                      libc::c_int
                                                                      +
                                                                      (*scriptState).value
                                                                          as
                                                                          libc::c_int)
                                                                     as u32_0
                                                                     as isize)
                                        as s8;
                                continue ;
                            }
                            206 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                (*channel).unk_22 = offset;
                                continue ;
                            }
                            207 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                test =
                                    &mut *(*seqPlayer).seqData.offset(offset
                                                                          as
                                                                          isize)
                                        as *mut u8_0;
                                *test.offset(0 as libc::c_int as isize) =
                                    ((*channel).unk_22 as libc::c_int >>
                                         8 as libc::c_int &
                                         0xff as libc::c_int) as u8_0;
                                *test.offset(1 as libc::c_int as isize) =
                                    ((*channel).unk_22 as libc::c_int &
                                         0xff as libc::c_int) as u8_0;
                                continue ;
                            }
                            208 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                if command as libc::c_int &
                                       0x80 as libc::c_int != 0 {
                                    (*channel).set_stereoHeadsetEffects(1 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0)
                                } else {
                                    (*channel).set_stereoHeadsetEffects(0 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0)
                                }
                                (*channel).stereo.asByte =
                                    (command as libc::c_int &
                                         0x7f as libc::c_int) as u8_0;
                                continue ;
                            }
                            209 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).noteAllocPolicy = command;
                                continue ;
                            }
                            210 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).adsr.sustain = command;
                                continue ;
                            }
                            229 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).reverbIndex = command;
                                continue ;
                            }
                            228 => {
                                if (*scriptState).value as libc::c_int !=
                                       -(1 as libc::c_int) {
                                    data =
                                        (*(*(*channel).dynTable).as_mut_ptr().offset((*scriptState).value
                                                                                         as
                                                                                         isize)).as_mut_ptr();
                                    // ! @bug: Missing a stack depth check here
                                    let fresh15 = (*scriptState).depth;
                                    (*scriptState).depth =
                                        (*scriptState).depth.wrapping_add(1);
                                    (*scriptState).stack[fresh15 as usize] =
                                        (*scriptState).pc;
                                    offset =
                                        (((*data.offset(0 as libc::c_int as
                                                            isize) as
                                               libc::c_int) <<
                                              8 as libc::c_int) +
                                             *data.offset(1 as libc::c_int as
                                                              isize) as
                                                 libc::c_int) as u16_0;
                                    (*scriptState).pc =
                                        (*seqPlayer).seqData.offset(offset as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                }
                                continue ;
                            }
                            230 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).bookOffset = command;
                                continue ;
                            }
                            231 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                data =
                                    &mut *(*seqPlayer).seqData.offset(offset
                                                                          as
                                                                          isize)
                                        as *mut u8_0;
                                (*channel).muteBehavior =
                                    *data.offset(0 as libc::c_int as isize);
                                data = data.offset(3 as libc::c_int as isize);
                                (*channel).noteAllocPolicy =
                                    *data.offset(-(2 as libc::c_int) as
                                                     isize);
                                AudioSeq_SetChannelPriorities(channel,
                                                              *data.offset(-(1
                                                                                 as
                                                                                 libc::c_int)
                                                                               as
                                                                               isize));
                                (*channel).transposition =
                                    *data.offset(0 as libc::c_int as isize) as
                                        s8 as s16;
                                data = data.offset(4 as libc::c_int as isize);
                                (*channel).newPan =
                                    *data.offset(-(3 as libc::c_int) as
                                                     isize);
                                (*channel).panChannelWeight =
                                    *data.offset(-(2 as libc::c_int) as
                                                     isize);
                                (*channel).reverb =
                                    *data.offset(-(1 as libc::c_int) as
                                                     isize);
                                (*channel).reverbIndex =
                                    *data.offset(0 as libc::c_int as isize);
                                // ! @bug: Not marking reverb state as changed
                                (*channel).changes.s.set_pan(1 as libc::c_int
                                                                 as u8_0);
                                continue ;
                            }
                            232 => {
                                (*channel).muteBehavior =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).noteAllocPolicy =
                                    parameters[1 as libc::c_int as usize] as
                                        u8_0;
                                command =
                                    parameters[2 as libc::c_int as usize] as
                                        u8_0;
                                AudioSeq_SetChannelPriorities(channel,
                                                              command);
                                (*channel).transposition =
                                    AudioSeq_ScriptReadU8(scriptState) as s8
                                        as s16;
                                (*channel).newPan =
                                    AudioSeq_ScriptReadU8(scriptState);
                                (*channel).panChannelWeight =
                                    AudioSeq_ScriptReadU8(scriptState);
                                (*channel).reverb =
                                    AudioSeq_ScriptReadU8(scriptState);
                                (*channel).reverbIndex =
                                    AudioSeq_ScriptReadU8(scriptState);
                                // ! @bug: Not marking reverb state as changed
                                (*channel).changes.s.set_pan(1 as libc::c_int
                                                                 as
                                                                 u8_0); // i is wrong here
                                continue ;
                            }
                            236 => {
                                (*channel).vibratoExtentTarget =
                                    0 as libc::c_int as u16_0;
                                (*channel).vibratoExtentStart =
                                    0 as libc::c_int as u16_0;
                                (*channel).vibratoExtentChangeDelay =
                                    0 as libc::c_int as u16_0;
                                (*channel).vibratoRateTarget =
                                    0 as libc::c_int as u16_0;
                                (*channel).vibratoRateStart =
                                    0 as libc::c_int as u16_0;
                                (*channel).vibratoRateChangeDelay =
                                    0 as libc::c_int as u16_0;
                                (*channel).filter = 0 as *mut s16;
                                (*channel).unk_0C = 0 as libc::c_int as u8_0;
                                (*channel).adsr.sustain =
                                    0 as libc::c_int as u8_0;
                                (*channel).velocityRandomVariance =
                                    0 as libc::c_int as u8_0;
                                (*channel).gateTimeRandomVariance =
                                    0 as libc::c_int as u8_0;
                                (*channel).unk_0F = 0 as libc::c_int as u8_0;
                                (*channel).unk_20 = 0 as libc::c_int as u16_0;
                                (*channel).bookOffset =
                                    0 as libc::c_int as u8_0;
                                (*channel).freqScale = 1.0f32;
                                continue ;
                            }
                            233 => {
                                AudioSeq_SetChannelPriorities(channel,
                                                              parameters[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                                  as u8_0);
                                continue ;
                            }
                            237 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).unk_0C = command;
                                continue ;
                            }
                            176 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                data =
                                    (*seqPlayer).seqData.offset(offset as
                                                                    libc::c_int
                                                                    as isize);
                                (*channel).filter = data as *mut s16;
                                continue ;
                            }
                            177 => {
                                (*channel).filter = 0 as *mut s16;
                                continue ;
                            }
                            179 => {
                                command =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                if !(*channel).filter.is_null() {
                                    lowBits =
                                        (command as libc::c_int >>
                                             4 as libc::c_int &
                                             0xf as libc::c_int) as u8_0;
                                    command =
                                        (command as libc::c_int &
                                             0xf as libc::c_int) as u8_0;
                                    AudioHeap_LoadFilter((*channel).filter,
                                                         lowBits as s32,
                                                         command as s32);
                                }
                                continue ;
                            }
                            178 => {
                                offset =
                                    parameters[0 as libc::c_int as usize] as
                                        u16_0;
                                (*channel).unk_22 =
                                    *((*seqPlayer).seqData.offset((offset as
                                                                       libc::c_int
                                                                       +
                                                                       (*scriptState).value
                                                                           as
                                                                           libc::c_int
                                                                           *
                                                                           2
                                                                               as
                                                                               libc::c_int)
                                                                      as u32_0
                                                                      as
                                                                      isize)
                                          as *mut u16_0);
                                continue ;
                            }
                            180 => {
                                (*channel).dynTable =
                                    &mut *(*seqPlayer).seqData.offset((*channel).unk_22
                                                                          as
                                                                          isize)
                                        as *mut u8_0 as *mut libc::c_void as
                                        *mut [[u8_0; 2]; 0];
                                continue ;
                            }
                            181 => {
                                (*channel).unk_22 =
                                    *((*channel).dynTable as
                                          *mut u16_0).offset((*scriptState).value
                                                                 as isize);
                                continue ;
                            }
                            182 => {
                                (*scriptState).value =
                                    (*(*(*channel).dynTable).as_mut_ptr().offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize))[(*scriptState).value
                                                                                                 as
                                                                                                 usize]
                                        as s8;
                                continue ;
                            }
                            183 => {
                                (*channel).unk_22 =
                                    if parameters[0 as libc::c_int as usize]
                                           == 0 as libc::c_int as libc::c_uint
                                       {
                                        (gAudioContext.audioRandom) &
                                            0xffff as libc::c_int as
                                                libc::c_uint
                                    } else {
                                        gAudioContext.audioRandom.wrapping_rem(parameters[0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize])
                                    } as u16_0;
                                continue ;
                            }
                            184 => {
                                (*scriptState).value =
                                    if parameters[0 as libc::c_int as usize]
                                           == 0 as libc::c_int as libc::c_uint
                                       {
                                        (gAudioContext.audioRandom) &
                                            0xffff as libc::c_int as
                                                libc::c_uint
                                    } else {
                                        gAudioContext.audioRandom.wrapping_rem(parameters[0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize])
                                    } as s8;
                                continue ;
                            }
                            189 => {
                                result = Audio_NextRandom() as s32;
                                (*channel).unk_22 =
                                    if parameters[0 as libc::c_int as usize]
                                           == 0 as libc::c_int as libc::c_uint
                                       {
                                        (result as u32_0) &
                                            0xffff as libc::c_int as
                                                libc::c_uint
                                    } else {
                                        (result as
                                             u32_0).wrapping_rem(parameters[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize])
                                    } as u16_0;
                                (*channel).unk_22 =
                                    ((*channel).unk_22 as
                                         libc::c_uint).wrapping_add(parameters[1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize])
                                        as u16_0 as u16_0;
                                pad2 =
                                    (*channel).unk_22 as libc::c_int /
                                        0x100 as libc::c_int +
                                        0x80 as libc::c_int;
                                param =
                                    (*channel).unk_22 as libc::c_int %
                                        0x100 as libc::c_int;
                                (*channel).unk_22 =
                                    (pad2 << 8 as libc::c_int | param) as
                                        u16_0;
                                continue ;
                            }
                            185 => {
                                (*channel).velocityRandomVariance =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                continue ;
                            }
                            186 => {
                                (*channel).gateTimeRandomVariance =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                continue ;
                            }
                            187 => {
                                (*channel).unk_0F =
                                    parameters[0 as libc::c_int as usize] as
                                        u8_0;
                                (*channel).unk_20 =
                                    parameters[1 as libc::c_int as usize] as
                                        u16_0;
                                continue ;
                            }
                            188 => {
                                (*channel).unk_22 =
                                    ((*channel).unk_22 as
                                         libc::c_uint).wrapping_add(parameters[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize])
                                        as u16_0 as u16_0;
                                continue ;
                            }
                            _ => { continue ; }
                        }
                        // NOTE: Intentional fallthrough
                        command =
                            parameters[0 as libc::c_int as usize] as u8_0;
                        AudioSeq_SetInstrument(channel, command);
                    }
                } else if command as libc::c_int >= 0x70 as libc::c_int {
                    lowBits =
                        (command as libc::c_int & 0x7 as libc::c_int) as u8_0;
                    if command as libc::c_int & 0xf8 as libc::c_int !=
                           0x70 as libc::c_int &&
                           lowBits as libc::c_int >= 4 as libc::c_int {
                        lowBits = 0 as libc::c_int as u8_0
                    }
                    match command as libc::c_int & 0xf8 as libc::c_int {
                        128 => {
                            if !(*channel).layers[lowBits as usize].is_null()
                               {
                                (*scriptState).value =
                                    (*(*channel).layers[lowBits as
                                                            usize]).finished()
                                        as s8
                            } else {
                                (*scriptState).value =
                                    -(1 as libc::c_int) as s8
                            }
                        }
                        136 => {
                            offset =
                                AudioSeq_ScriptReadS16(scriptState) as u16_0;
                            if AudioSeq_SeqChannelSetLayer(channel,
                                                           lowBits as s32) ==
                                   0 {
                                (*(*channel).layers[lowBits as
                                                        usize]).scriptState.pc
                                    =
                                    &mut *(*seqPlayer).seqData.offset(offset
                                                                          as
                                                                          isize)
                                        as *mut u8_0
                            }
                        }
                        144 => {
                            AudioSeq_SeqLayerFree(channel, lowBits as s32);
                        }
                        152 => {
                            if !((*scriptState).value as libc::c_int ==
                                     -(1 as libc::c_int) ||
                                     AudioSeq_SeqChannelSetLayer(channel,
                                                                 lowBits as
                                                                     s32) ==
                                         -(1 as libc::c_int)) {
                                data =
                                    (*(*(*channel).dynTable).as_mut_ptr().offset((*scriptState).value
                                                                                     as
                                                                                     isize)).as_mut_ptr();
                                offset =
                                    (((*data.offset(0 as libc::c_int as isize)
                                           as libc::c_int) <<
                                          8 as libc::c_int) +
                                         *data.offset(1 as libc::c_int as
                                                          isize) as
                                             libc::c_int) as u16_0;
                                (*(*channel).layers[lowBits as
                                                        usize]).scriptState.pc
                                    =
                                    &mut *(*seqPlayer).seqData.offset(offset
                                                                          as
                                                                          isize)
                                        as *mut u8_0
                            }
                        }
                        112 => {
                            (*channel).soundScriptIO[lowBits as usize] =
                                (*scriptState).value
                        }
                        120 => {
                            pad1 = AudioSeq_ScriptReadS16(scriptState);
                            if AudioSeq_SeqChannelSetLayer(channel,
                                                           lowBits as s32) ==
                                   0 {
                                (*(*channel).layers[lowBits as
                                                        usize]).scriptState.pc
                                    =
                                    &mut *(*scriptState).pc.offset(pad1 as
                                                                       isize)
                                        as *mut u8_0
                            }
                        }
                        _ => { }
                    }
                } else {
                    lowBits =
                        (command as libc::c_int & 0xf as libc::c_int) as u8_0;
                    match command as libc::c_int & 0xf0 as libc::c_int {
                        0 => { (*channel).delay = lowBits as u16_0; break ; }
                        16 => {
                            if (lowBits as libc::c_int) < 8 as libc::c_int {
                                (*channel).soundScriptIO[lowBits as usize] =
                                    -(1 as libc::c_int) as s8;
                                (AudioLoad_SlowLoadSample((*channel).fontId as
                                                              s32,
                                                          (*scriptState).value
                                                              as s32,
                                                          &mut *(*channel).soundScriptIO.as_mut_ptr().offset(lowBits
                                                                                                                 as
                                                                                                                 isize)))
                                    == -(1 as libc::c_int);
                            } else {
                                lowBits =
                                    (lowBits as libc::c_int -
                                         8 as libc::c_int) as u8_0;
                                (*channel).soundScriptIO[lowBits as usize] =
                                    -(1 as libc::c_int) as s8;
                                (AudioLoad_SlowLoadSample((*channel).fontId as
                                                              s32,
                                                          (*channel).unk_22 as
                                                              libc::c_int +
                                                              0x100 as
                                                                  libc::c_int,
                                                          &mut *(*channel).soundScriptIO.as_mut_ptr().offset(lowBits
                                                                                                                 as
                                                                                                                 isize)))
                                    == -(1 as libc::c_int);
                            }
                        }
                        96 => {
                            (*scriptState).value =
                                (*channel).soundScriptIO[lowBits as usize];
                            if (lowBits as libc::c_int) < 2 as libc::c_int {
                                (*channel).soundScriptIO[lowBits as usize] =
                                    -(1 as libc::c_int) as s8
                            }
                        }
                        80 => {
                            (*scriptState).value =
                                ((*scriptState).value as libc::c_int -
                                     (*channel).soundScriptIO[lowBits as
                                                                  usize] as
                                         libc::c_int) as s8
                        }
                        32 => {
                            offset =
                                AudioSeq_ScriptReadS16(scriptState) as u16_0;
                            AudioSeq_SequenceChannelEnable(seqPlayer, lowBits,
                                                           &mut *(*seqPlayer).seqData.offset(offset
                                                                                                 as
                                                                                                 isize)
                                                               as *mut u8_0 as
                                                               *mut libc::c_void);
                        }
                        48 => {
                            command = AudioSeq_ScriptReadU8(scriptState);
                            (*(*seqPlayer).channels[lowBits as
                                                        usize]).soundScriptIO[command
                                                                                  as
                                                                                  usize]
                                = (*scriptState).value
                        }
                        64 => {
                            command = AudioSeq_ScriptReadU8(scriptState);
                            (*scriptState).value =
                                (*(*seqPlayer).channels[lowBits as
                                                            usize]).soundScriptIO[command
                                                                                      as
                                                                                      usize]
                        }
                        _ => { }
                    }
                }
            }
        }
    }
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[*mut SequenceLayer; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SequenceLayer>()
                                                   as libc::c_ulong) as s32 {
        if !(*channel).layers[i as usize].is_null() {
            AudioSeq_SeqLayerProcessScript((*channel).layers[i as usize]);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SequencePlayerProcessSequence(mut seqPlayer:
                                                                    *mut SequencePlayer) {
    let mut command: u8_0 = 0;
    let mut commandLow: u8_0 = 0;
    let mut seqScript: *mut SeqScriptState = &mut (*seqPlayer).scriptState;
    let mut tempS: s16 = 0;
    let mut temp: u16_0 = 0;
    let mut i: s32 = 0;
    let mut value: s32 = 0;
    let mut data: *mut u8_0 = 0 as *mut u8_0;
    let mut data2: *mut u8_0 = 0 as *mut u8_0;
    let mut data3: *mut u8_0 = 0 as *mut u8_0;
    let mut pad3: s32 = 0;
    let mut dummy: s32 = 0;
    if (*seqPlayer).enabled() == 0 { return }
    if AudioLoad_IsSeqLoadComplete((*seqPlayer).seqId as s32) == 0 ||
           AudioLoad_IsFontLoadComplete((*seqPlayer).defaultFont as s32) == 0
       {
        AudioSeq_SequencePlayerDisable(seqPlayer);
        return
    }
    AudioLoad_SetSeqLoadStatus((*seqPlayer).seqId as s32, 2 as libc::c_int);
    AudioLoad_SetFontLoadStatus((*seqPlayer).defaultFont as s32,
                                2 as libc::c_int);
    if (*seqPlayer).muted() as libc::c_int != 0 &&
           (*seqPlayer).muteBehavior as libc::c_int & 0x80 as libc::c_int != 0
       {
        return
    }
    (*seqPlayer).scriptCounter = (*seqPlayer).scriptCounter.wrapping_add(1);
    (*seqPlayer).tempoAcc =
        ((*seqPlayer).tempoAcc as libc::c_int +
             (*seqPlayer).tempo as libc::c_int) as u16_0;
    (*seqPlayer).tempoAcc =
        ((*seqPlayer).tempoAcc as libc::c_int +
             (*seqPlayer).unk_0C as s16 as libc::c_int) as u16_0;
    if ((*seqPlayer).tempoAcc as libc::c_int) <
           gAudioContext.tempoInternalToExternal as libc::c_int {
        return
    }
    (*seqPlayer).tempoAcc =
        ((*seqPlayer).tempoAcc as libc::c_int -
             gAudioContext.tempoInternalToExternal as u16_0 as libc::c_int) as
            u16_0;
    if (*seqPlayer).stopScript() as libc::c_int == 1 as libc::c_int { return }
    if (*seqPlayer).delay as libc::c_int > 1 as libc::c_int {
        (*seqPlayer).delay = (*seqPlayer).delay.wrapping_sub(1)
    } else {
        (*seqPlayer).set_recalculateVolume(1 as libc::c_int as u8_0);
        loop  {
            command = AudioSeq_ScriptReadU8(seqScript);
            // 0xF2 and above are "flow control" commands, including termination.
            if command as libc::c_int >= 0xf2 as libc::c_int {
                let mut scriptHandled: s32 =
                    AudioSeq_HandleScriptFlowControl(seqPlayer, seqScript,
                                                     command as s32,
                                                     AudioSeq_GetScriptControlFlowArgument(&mut (*seqPlayer).scriptState,
                                                                                           command)
                                                         as s32);
                if !(scriptHandled != 0 as libc::c_int) { continue ; }
                if scriptHandled == -(1 as libc::c_int) {
                    AudioSeq_SequencePlayerDisable(seqPlayer);
                } else { (*seqPlayer).delay = scriptHandled as u16_0 }
                break ;
            } else if command as libc::c_int >= 0xc0 as libc::c_int {
                let mut current_block_123: u64;
                match command as libc::c_int {
                    241 => {
                        Audio_NotePoolClear(&mut (*seqPlayer).notePool);
                        command = AudioSeq_ScriptReadU8(seqScript);
                        Audio_NotePoolFill(&mut (*seqPlayer).notePool,
                                           command as s32);
                        // Fake-match: the asm has two breaks in a row here,
                        // which the compiler normally optimizes out.
                        dummy = -(1 as libc::c_int);
                        if dummy < 0 as libc::c_int {
                            dummy = 0 as libc::c_int
                        }
                        if dummy > 1 as libc::c_int {
                            dummy = 1 as libc::c_int
                        }
                        (dummy) != 0;
                        current_block_123 = 1830138855519935310;
                    }
                    240 => {
                        Audio_NotePoolClear(&mut (*seqPlayer).notePool);
                        current_block_123 = 1830138855519935310;
                    }
                    223 => {
                        (*seqPlayer).transposition = 0 as libc::c_int as s16;
                        current_block_123 = 15293521073039913846;
                    }
                    222 => { current_block_123 = 15293521073039913846; }
                    221 => {
                        (*seqPlayer).tempo =
                            (AudioSeq_ScriptReadU8(seqScript) as libc::c_int *
                                 48 as libc::c_int) as u16_0;
                        if (*seqPlayer).tempo as libc::c_int >
                               gAudioContext.tempoInternalToExternal as
                                   libc::c_int {
                            (*seqPlayer).tempo =
                                gAudioContext.tempoInternalToExternal as u16_0
                        }
                        if (*seqPlayer).tempo as s16 as libc::c_int <=
                               0 as libc::c_int {
                            (*seqPlayer).tempo = 1 as libc::c_int as u16_0
                        }
                        current_block_123 = 1830138855519935310;
                    }
                    220 => {
                        (*seqPlayer).unk_0C =
                            (AudioSeq_ScriptReadU8(seqScript) as s8 as
                                 libc::c_int * 48 as libc::c_int) as u16_0;
                        current_block_123 = 1830138855519935310;
                    }
                    218 => {
                        command = AudioSeq_ScriptReadU8(seqScript);
                        temp = AudioSeq_ScriptReadS16(seqScript) as u16_0;
                        match command as libc::c_int {
                            0 | 1 => {
                                if (*seqPlayer).state as libc::c_int !=
                                       2 as libc::c_int {
                                    (*seqPlayer).fadeTimerUnkEu = temp;
                                    (*seqPlayer).state = command
                                }
                            }
                            2 => {
                                (*seqPlayer).fadeTimer = temp;
                                (*seqPlayer).state = command;
                                (*seqPlayer).fadeVelocity =
                                    (0 as libc::c_int as libc::c_float -
                                         (*seqPlayer).fadeVolume) /
                                        (*seqPlayer).fadeTimer as s32 as
                                            libc::c_float
                            }
                            _ => { }
                        }
                        current_block_123 = 1830138855519935310;
                    }
                    219 => {
                        value = AudioSeq_ScriptReadU8(seqScript) as s32;
                        let mut current_block_72: u64;
                        match (*seqPlayer).state as libc::c_int {
                            1 => {
                                (*seqPlayer).state = 0 as libc::c_int as u8_0;
                                (*seqPlayer).fadeVolume = 0.0f32;
                                current_block_72 = 2830256955663249166;
                            }
                            0 => { current_block_72 = 2830256955663249166; }
                            2 | _ => {
                                current_block_72 = 8102658916883067714;
                            }
                        }
                        match current_block_72 {
                            2830256955663249166 =>
                            // NOTE: Intentional fallthrough
                            {
                                (*seqPlayer).fadeTimer =
                                    (*seqPlayer).fadeTimerUnkEu;
                                if (*seqPlayer).fadeTimerUnkEu as libc::c_int
                                       != 0 as libc::c_int {
                                    (*seqPlayer).fadeVelocity =
                                        (value as libc::c_float / 127.0f32 -
                                             (*seqPlayer).fadeVolume) /
                                            (*seqPlayer).fadeTimer as s32 as
                                                libc::c_float
                                } else {
                                    (*seqPlayer).fadeVolume =
                                        value as libc::c_float / 127.0f32
                                }
                            }
                            _ => { }
                        }
                        current_block_123 = 1830138855519935310;
                    }
                    217 => {
                        (*seqPlayer).fadeVolumeScale =
                            AudioSeq_ScriptReadU8(seqScript) as s8 as
                                libc::c_int as libc::c_float / 127.0f32;
                        current_block_123 = 1830138855519935310;
                    }
                    215 => {
                        temp = AudioSeq_ScriptReadS16(seqScript) as u16_0;
                        AudioSeq_SequencePlayerSetupChannels(seqPlayer, temp);
                        current_block_123 = 1830138855519935310;
                    }
                    214 => {
                        AudioSeq_ScriptReadS16(seqScript);
                        current_block_123 = 1830138855519935310;
                    }
                    213 => {
                        (*seqPlayer).muteVolumeScale =
                            AudioSeq_ScriptReadU8(seqScript) as s8 as
                                libc::c_int as libc::c_float / 127.0f32;
                        current_block_123 = 1830138855519935310;
                    }
                    212 => {
                        (*seqPlayer).set_muted(1 as libc::c_int as u8_0);
                        current_block_123 = 1830138855519935310;
                    }
                    211 => {
                        (*seqPlayer).muteBehavior =
                            AudioSeq_ScriptReadU8(seqScript);
                        current_block_123 = 1830138855519935310;
                    }
                    209 | 210 => {
                        temp = AudioSeq_ScriptReadS16(seqScript) as u16_0;
                        data3 =
                            &mut *(*seqPlayer).seqData.offset(temp as isize)
                                as *mut u8_0;
                        if command as libc::c_int == 0xd2 as libc::c_int {
                            (*seqPlayer).shortNoteVelocityTable = data3
                        } else { (*seqPlayer).shortNoteGateTimeTable = data3 }
                        current_block_123 = 1830138855519935310;
                    }
                    208 => {
                        (*seqPlayer).noteAllocPolicy =
                            AudioSeq_ScriptReadU8(seqScript);
                        current_block_123 = 1830138855519935310;
                    }
                    206 => {
                        command = AudioSeq_ScriptReadU8(seqScript);
                        if command as libc::c_int == 0 as libc::c_int {
                            (*seqScript).value =
                                (gAudioContext.audioRandom >> 2 as libc::c_int
                                     & 0xff as libc::c_int as libc::c_uint) as
                                    s8
                        } else {
                            (*seqScript).value =
                                (gAudioContext.audioRandom >>
                                     2 as
                                         libc::c_int).wrapping_rem(command as
                                                                       libc::c_uint)
                                    as s8
                        }
                        current_block_123 = 1830138855519935310;
                    }
                    205 => {
                        temp = AudioSeq_ScriptReadS16(seqScript) as u16_0;
                        if (*seqScript).value as libc::c_int !=
                               -(1 as libc::c_int) &&
                               (*seqScript).depth as libc::c_int !=
                                   3 as libc::c_int {
                            data =
                                (*seqPlayer).seqData.offset((temp as
                                                                 libc::c_int +
                                                                 (((*seqScript).value
                                                                       as
                                                                       libc::c_int)
                                                                      <<
                                                                      1 as
                                                                          libc::c_int))
                                                                as u32_0 as
                                                                isize);
                            (*seqScript).stack[(*seqScript).depth as usize] =
                                (*seqScript).pc;
                            (*seqScript).depth =
                                (*seqScript).depth.wrapping_add(1);
                            temp =
                                (((*data.offset(0 as libc::c_int as isize) as
                                       libc::c_int) << 8 as libc::c_int) +
                                     *data.offset(1 as libc::c_int as isize)
                                         as libc::c_int) as u16_0;
                            (*seqScript).pc =
                                &mut *(*seqPlayer).seqData.offset(temp as
                                                                      isize)
                                    as *mut u8_0
                        }
                        current_block_123 = 1830138855519935310;
                    }
                    204 => {
                        (*seqScript).value =
                            AudioSeq_ScriptReadU8(seqScript) as s8;
                        current_block_123 = 1830138855519935310;
                    }
                    201 => {
                        (*seqScript).value =
                            ((*seqScript).value as libc::c_int &
                                 AudioSeq_ScriptReadU8(seqScript) as
                                     libc::c_int) as s8;
                        current_block_123 = 1830138855519935310;
                    }
                    200 => {
                        (*seqScript).value =
                            ((*seqScript).value as libc::c_int -
                                 AudioSeq_ScriptReadU8(seqScript) as
                                     libc::c_int) as s8;
                        current_block_123 = 1830138855519935310;
                    }
                    199 => {
                        command = AudioSeq_ScriptReadU8(seqScript);
                        temp = AudioSeq_ScriptReadS16(seqScript) as u16_0;
                        data2 =
                            &mut *(*seqPlayer).seqData.offset(temp as isize)
                                as *mut u8_0;
                        *data2 =
                            ((*seqScript).value as u8_0 as libc::c_int +
                                 command as libc::c_int) as u8_0;
                        current_block_123 = 1830138855519935310;
                    }
                    198 => {
                        (*seqPlayer).set_stopScript(1 as libc::c_int as u8_0);
                        return
                    }
                    197 => {
                        (*seqPlayer).scriptCounter =
                            AudioSeq_ScriptReadS16(seqScript) as u16_0 as
                                u32_0;
                        current_block_123 = 1830138855519935310;
                    }
                    239 => {
                        AudioSeq_ScriptReadS16(seqScript);
                        AudioSeq_ScriptReadU8(seqScript);
                        current_block_123 = 1830138855519935310;
                    }
                    196 => {
                        command = AudioSeq_ScriptReadU8(seqScript);
                        if command as libc::c_int == 0xff as libc::c_int {
                            command = (*seqPlayer).playerIdx as u8_0
                        }
                        commandLow = AudioSeq_ScriptReadU8(seqScript);
                        AudioLoad_SyncInitSeqPlayer(command as s32,
                                                    commandLow as s32,
                                                    0 as libc::c_int);
                        if command as libc::c_int ==
                               (*seqPlayer).playerIdx as u8_0 as libc::c_int {
                            return
                        }
                        current_block_123 = 1830138855519935310;
                    }
                    _ => { current_block_123 = 1830138855519935310; }
                }
                match current_block_123 {
                    15293521073039913846 =>
                    // Note: intentional fallthrough, also executes below command
                    {
                        (*seqPlayer).transposition =
                            ((*seqPlayer).transposition as libc::c_int +
                                 AudioSeq_ScriptReadU8(seqScript) as s8 as
                                     libc::c_int) as s16
                    }
                    _ => { }
                }
            } else {
                commandLow =
                    (command as libc::c_int & 0xf as libc::c_int) as
                        u8_0; // 120 BPM
                match command as libc::c_int & 0xf0 as libc::c_int {
                    0 => {
                        (*seqScript).value =
                            ((*(*seqPlayer).channels[commandLow as
                                                         usize]).enabled() as
                                 libc::c_int ^ 1 as libc::c_int) as s8
                    }
                    80 => {
                        (*seqScript).value =
                            ((*seqScript).value as libc::c_int -
                                 (*seqPlayer).soundScriptIO[commandLow as
                                                                usize] as
                                     libc::c_int) as s8
                    }
                    112 => {
                        (*seqPlayer).soundScriptIO[commandLow as usize] =
                            (*seqScript).value
                    }
                    128 => {
                        (*seqScript).value =
                            (*seqPlayer).soundScriptIO[commandLow as usize];
                        if (commandLow as libc::c_int) < 2 as libc::c_int {
                            (*seqPlayer).soundScriptIO[commandLow as usize] =
                                -(1 as libc::c_int) as s8
                        }
                    }
                    64 => {
                        AudioSeq_SequenceChannelDisable((*seqPlayer).channels[commandLow
                                                                                  as
                                                                                  usize]);
                    }
                    144 => {
                        temp = AudioSeq_ScriptReadS16(seqScript) as u16_0;
                        AudioSeq_SequenceChannelEnable(seqPlayer, commandLow,
                                                       &mut *(*seqPlayer).seqData.offset(temp
                                                                                             as
                                                                                             isize)
                                                           as *mut u8_0 as
                                                           *mut libc::c_void);
                    }
                    160 => {
                        tempS = AudioSeq_ScriptReadS16(seqScript);
                        AudioSeq_SequenceChannelEnable(seqPlayer, commandLow,
                                                       &mut *(*seqScript).pc.offset(tempS
                                                                                        as
                                                                                        isize)
                                                           as *mut u8_0 as
                                                           *mut libc::c_void);
                    }
                    176 => {
                        command = AudioSeq_ScriptReadU8(seqScript);
                        temp = AudioSeq_ScriptReadS16(seqScript) as u16_0;
                        data2 =
                            &mut *(*seqPlayer).seqData.offset(temp as isize)
                                as *mut u8_0;
                        AudioLoad_SlowLoadSeq(command as s32, data2,
                                              &mut *(*seqPlayer).soundScriptIO.as_mut_ptr().offset(commandLow
                                                                                                       as
                                                                                                       isize));
                    }
                    96 => {
                        command = AudioSeq_ScriptReadU8(seqScript);
                        value = command as s32;
                        temp = AudioSeq_ScriptReadU8(seqScript) as u16_0;
                        AudioLoad_ScriptLoad(value, temp as s32,
                                             &mut *(*seqPlayer).soundScriptIO.as_mut_ptr().offset(commandLow
                                                                                                      as
                                                                                                      isize));
                    }
                    _ => { }
                }
            }
        }
    }
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[*mut SequenceChannel; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SequenceChannel>()
                                                   as libc::c_ulong) as s32 {
        if (*(*seqPlayer).channels[i as usize]).enabled() != 0 {
            AudioSeq_SequenceChannelProcessScript((*seqPlayer).channels[i as
                                                                            usize]);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_ProcessSequences(mut arg0: s32) {
    let mut seqPlayer: *mut SequencePlayer = 0 as *mut SequencePlayer;
    let mut i: u32_0 = 0;
    gAudioContext.noteSubEuOffset =
        (gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int -
             arg0 - 1 as libc::c_int) * gAudioContext.numNotes;
    i = 0 as libc::c_int as u32_0;
    while i < gAudioContext.audioBufferParameters.numSequencePlayers as u32_0
          {
        seqPlayer =
            &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(i as isize) as
                *mut SequencePlayer;
        if (*seqPlayer).enabled() as libc::c_int == 1 as libc::c_int {
            AudioSeq_SequencePlayerProcessSequence(seqPlayer);
            Audio_SequencePlayerProcessSound(seqPlayer);
        }
        i = i.wrapping_add(1)
    }
    Audio_ProcessNotes();
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_SkipForwardSequence(mut seqPlayer:
                                                          *mut SequencePlayer) {
    while (*seqPlayer).skipTicks > 0 as libc::c_int {
        AudioSeq_SequencePlayerProcessSequence(seqPlayer);
        Audio_SequencePlayerProcessSound(seqPlayer);
        (*seqPlayer).skipTicks -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_ResetSequencePlayer(mut seqPlayer:
                                                          *mut SequencePlayer) {
    let mut i: s32 = 0;
    AudioSeq_SequencePlayerDisable(seqPlayer);
    (*seqPlayer).set_stopScript(0 as libc::c_int as u8_0);
    (*seqPlayer).delay = 0 as libc::c_int as u16_0;
    (*seqPlayer).state = 1 as libc::c_int as u8_0;
    (*seqPlayer).fadeTimer = 0 as libc::c_int as u16_0;
    (*seqPlayer).fadeTimerUnkEu = 0 as libc::c_int as u16_0;
    (*seqPlayer).tempoAcc = 0 as libc::c_int as u16_0;
    (*seqPlayer).tempo = (120 as libc::c_int * 48 as libc::c_int) as u16_0;
    (*seqPlayer).unk_0C = 0 as libc::c_int as u16_0;
    (*seqPlayer).transposition = 0 as libc::c_int as s16;
    (*seqPlayer).noteAllocPolicy = 0 as libc::c_int as u8_0;
    (*seqPlayer).shortNoteVelocityTable =
        gDefaultShortNoteVelocityTable.as_mut_ptr();
    (*seqPlayer).shortNoteGateTimeTable =
        gDefaultShortNoteGateTimeTable.as_mut_ptr();
    (*seqPlayer).scriptCounter = 0 as libc::c_int as u32_0;
    (*seqPlayer).fadeVolume = 1.0f32;
    (*seqPlayer).fadeVelocity = 0.0f32;
    (*seqPlayer).volume = 0.0f32;
    (*seqPlayer).muteVolumeScale = 0.5f32;
    i = 0 as libc::c_int;
    while i < 0x10 as libc::c_int {
        AudioSeq_InitSequenceChannel((*seqPlayer).channels[i as usize]);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_InitSequencePlayerChannels(mut playerIdx:
                                                                 s32) {
    let mut channel: *mut SequenceChannel = 0 as *mut SequenceChannel;
    let mut seqPlayer: *mut SequencePlayer =
        &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(playerIdx as isize)
            as *mut SequencePlayer;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    i = 0 as libc::c_int;
    while i < 0x10 as libc::c_int {
        (*seqPlayer).channels[i as usize] =
            AudioHeap_AllocZeroed(&mut gAudioContext.notesAndBuffersPool,
                                  ::std::mem::size_of::<SequenceChannel>() as
                                      libc::c_ulong) as *mut SequenceChannel;
        if (*seqPlayer).channels[i as usize].is_null() {
            (*seqPlayer).channels[i as usize] =
                &mut gAudioContext.sequenceChannelNone
        } else {
            channel = (*seqPlayer).channels[i as usize];
            (*channel).seqPlayer = seqPlayer;
            (*channel).set_enabled(0 as libc::c_int as u8_0);
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                (*channel).layers[j as usize] = 0 as *mut SequenceLayer;
                j += 1
            }
        }
        AudioSeq_InitSequenceChannel((*seqPlayer).channels[i as usize]);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_InitSequencePlayer(mut seqPlayer:
                                                         *mut SequencePlayer) {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    i = 0 as libc::c_int;
    while i < 0x10 as libc::c_int {
        (*seqPlayer).channels[i as usize] =
            &mut gAudioContext.sequenceChannelNone;
        i += 1
    }
    (*seqPlayer).set_enabled(0 as libc::c_int as u8_0);
    (*seqPlayer).set_muted(0 as libc::c_int as u8_0);
    (*seqPlayer).set_fontDmaInProgress(0 as libc::c_int as u8_0);
    (*seqPlayer).set_seqDmaInProgress(0 as libc::c_int as u8_0);
    (*seqPlayer).set_unk_0b1(0 as libc::c_int as u8_0);
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        (*seqPlayer).soundScriptIO[j as usize] = -(1 as libc::c_int) as s8;
        j += 1
    }
    (*seqPlayer).muteBehavior =
        (0x40 as libc::c_int | 0x20 as libc::c_int) as u8_0;
    (*seqPlayer).fadeVolumeScale = 1.0f32;
    (*seqPlayer).unk_34 = 1.0f32;
    Audio_InitNoteLists(&mut (*seqPlayer).notePool);
    AudioSeq_ResetSequencePlayer(seqPlayer);
}
#[no_mangle]
pub unsafe extern "C" fn AudioSeq_InitSequencePlayers() {
    let mut i: s32 = 0;
    AudioSeq_InitLayerFreelist();
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        gAudioContext.sequenceLayers[i as usize].channel =
            0 as *mut SequenceChannel;
        gAudioContext.sequenceLayers[i as
                                         usize].set_enabled(0 as libc::c_int
                                                                as u8_0);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        AudioSeq_InitSequencePlayer(&mut *gAudioContext.seqPlayers.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize));
        i += 1
    };
}
