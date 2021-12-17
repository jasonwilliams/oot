#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osEPiStartDma(handle: *mut OSPiHandle, mb: *mut OSIoMesg,
                     direction: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osCartRomInit() -> *mut OSPiHandle;
    #[no_mangle]
    fn AudioHeap_DiscardFont(fontId: s32);
    #[no_mangle]
    fn AudioHeap_WritebackDCache(mem: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn AudioHeap_AllocAttemptExternal(pool: *mut AudioAllocPool, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_AllocZeroed(pool: *mut AudioAllocPool, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_Alloc(pool: *mut AudioAllocPool, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_AllocPoolInit(pool: *mut AudioAllocPool,
                               mem: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn AudioHeap_InitMainPools(sizeForAudioInitPool: s32);
    #[no_mangle]
    fn AudioHeap_AllocCached(tableType: s32, size: s32, cache: s32, id: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_SearchCaches(tableType: s32, arg1: s32, id: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_ResetStep() -> s32;
    #[no_mangle]
    fn AudioHeap_SearchPermanentCache(tableType: s32, id: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_AllocPermanent(tableType: s32, id: s32, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_AllocSampleCache(size: u32_0, fontId: s32,
                                  sampleAddr: *mut libc::c_void, medium: s8,
                                  cache: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn AudioHeap_ApplySampleBankCache(sampleBankId: s32);
    #[no_mangle]
    fn Audio_InitMesgQueues();
    #[no_mangle]
    fn Audio_InvalDCache(buf: *mut libc::c_void, size: s32);
    #[no_mangle]
    fn Audio_GetInstrumentInner(fontId: s32, instId: s32) -> *mut Instrument;
    #[no_mangle]
    fn Audio_GetDrum(fontId: s32, drumId: s32) -> *mut Drum;
    #[no_mangle]
    fn Audio_GetSfx(fontId: s32, sfxId: s32) -> *mut SoundFontSound;
    #[no_mangle]
    fn AudioSeq_SequencePlayerDisable(seqPlayer: *mut SequencePlayer);
    #[no_mangle]
    fn AudioSeq_SkipForwardSequence(seqPlayer: *mut SequencePlayer);
    #[no_mangle]
    fn AudioSeq_ResetSequencePlayer(seqPlayer: *mut SequencePlayer);
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    #[no_mangle]
    static D_8014A6C4: AudioContextInitSizes;
    #[no_mangle]
    static mut _AudiotableSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _AudiobankSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _AudioseqSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut gSequenceFontTable: [u8_0; 0];
    #[no_mangle]
    static mut gSampleBankTable: [u8_0; 0];
    #[no_mangle]
    static mut gSoundFontTable: [u8_0; 0];
    #[no_mangle]
    static mut gSequenceTable: [u8_0; 0];
    #[no_mangle]
    static mut gAudioHeap: [u8_0; 229376];
    #[no_mangle]
    static mut osTvType: u32_0;
    #[no_mangle]
    static mut D_801755D0: Option<unsafe extern "C" fn() -> ()>;
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
pub type DmaHandler
    =
    Option<unsafe extern "C" fn(_: *mut OSPiHandle, _: *mut OSIoMesg, _: s32)
               -> s32>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioContextInitSizes {
    pub heapSize: u32_0,
    pub initPoolSize: u32_0,
    pub permanentPoolSize: u32_0,
}
// size = 0x18
// opaque type for unpatched sound font data (should maybe get rid of this?)
pub type SoundFontData = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RelocInfo {
    pub sampleBankId1: s32,
    pub sampleBankId2: s32,
    pub baseAddr1: s32,
    pub baseAddr2: s32,
    pub medium1: u32_0,
    pub medium2: u32_0,
}
pub const LOAD_STATUS_START: C2RustUnnamed_14 = 1;
pub const LOAD_STATUS_WAITING: C2RustUnnamed_14 = 0;
pub const LOAD_STATUS_DONE: C2RustUnnamed_14 = 3;
pub const LOAD_STATUS_LOADING: C2RustUnnamed_14 = 2;
pub type C2RustUnnamed_14 = libc::c_uint;
#[no_mangle]
pub static mut sScriptLoadQueue: OSMesgQueue =
    OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                fullqueue: 0 as *const OSThread as *mut OSThread,
                validCount: 0,
                first: 0,
                msgCount: 0,
                msg: 0 as *const OSMesg as *mut OSMesg,};
#[no_mangle]
pub static mut sScriptLoadMesgBuf: [OSMesg; 16] =
    [0 as *const libc::c_void as *mut libc::c_void; 16];
#[no_mangle]
pub static mut sScriptLoadDonePointers: [*mut s8; 16] =
    [0 as *const s8 as *mut s8; 16];
#[no_mangle]
pub static mut sAudioLoadPad1: [s32; 2] = [0; 2];
// file padding
#[no_mangle]
pub static mut D_8016B780: s32 = 0;
#[no_mangle]
pub static mut sAudioLoadPad2: [s32; 4] = [0; 4];
// double file padding?
#[no_mangle]
pub static mut sDmaHandler: DmaHandler =
    unsafe {
        Some(osEPiStartDma as
                 unsafe extern "C" fn(_: *mut OSPiHandle, _: *mut OSIoMesg,
                                      _: s32) -> s32)
    };
#[no_mangle]
pub static mut sUnusedHandler: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut gAudioContextInitalized: s32 = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_DecreaseSampleDmaTtls() {
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < gAudioContext.sampleDmaListSize1 {
        let mut dma: *mut SampleDma =
            &mut *gAudioContext.sampleDmas.offset(i as isize) as
                *mut SampleDma;
        if (*dma).ttl as libc::c_int != 0 as libc::c_int {
            (*dma).ttl = (*dma).ttl.wrapping_sub(1);
            if (*dma).ttl as libc::c_int == 0 as libc::c_int {
                (*dma).reuseIndex = gAudioContext.sampleDmaReuseQueue1WrPos;
                gAudioContext.sampleDmaReuseQueue1[gAudioContext.sampleDmaReuseQueue1WrPos
                                                       as usize] = i as u8_0;
                gAudioContext.sampleDmaReuseQueue1WrPos =
                    gAudioContext.sampleDmaReuseQueue1WrPos.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    i = gAudioContext.sampleDmaListSize1;
    while i < gAudioContext.sampleDmaCount {
        let mut dma_0: *mut SampleDma =
            &mut *gAudioContext.sampleDmas.offset(i as isize) as
                *mut SampleDma;
        if (*dma_0).ttl as libc::c_int != 0 as libc::c_int {
            (*dma_0).ttl = (*dma_0).ttl.wrapping_sub(1);
            if (*dma_0).ttl as libc::c_int == 0 as libc::c_int {
                (*dma_0).reuseIndex = gAudioContext.sampleDmaReuseQueue2WrPos;
                gAudioContext.sampleDmaReuseQueue2[gAudioContext.sampleDmaReuseQueue2WrPos
                                                       as usize] = i as u8_0;
                gAudioContext.sampleDmaReuseQueue2WrPos =
                    gAudioContext.sampleDmaReuseQueue2WrPos.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    gAudioContext.unused2628 = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_DmaSampleData(mut devAddr: u32_0,
                                                 mut size: u32_0,
                                                 mut arg2: s32,
                                                 mut dmaIndexRef: *mut u8_0,
                                                 mut medium: s32)
 -> *mut libc::c_void {
    let mut pad1: s32 = 0;
    let mut dma: *mut SampleDma = 0 as *mut SampleDma;
    let mut hasDma: s32 = 0 as libc::c_int;
    let mut dmaDevAddr: u32_0 = 0;
    let mut pad2: u32_0 = 0;
    let mut dmaIndex: u32_0 = 0;
    let mut transfer: u32_0 = 0;
    let mut bufferPos: s32 = 0;
    let mut i: u32_0 = 0;
    let mut current_block_37: u64;
    if arg2 != 0 as libc::c_int ||
           *dmaIndexRef as libc::c_uint >= gAudioContext.sampleDmaListSize1 {
        i = gAudioContext.sampleDmaListSize1;
        while i < gAudioContext.sampleDmaCount {
            dma =
                &mut *gAudioContext.sampleDmas.offset(i as isize) as
                    *mut SampleDma;
            bufferPos = devAddr.wrapping_sub((*dma).devAddr) as s32;
            if 0 as libc::c_int <= bufferPos &&
                   bufferPos as u32_0 <=
                       ((*dma).size as libc::c_uint).wrapping_sub(size) {
                // We already have a DMA request for this memory range.
                if (*dma).ttl as libc::c_int == 0 as libc::c_int &&
                       gAudioContext.sampleDmaReuseQueue2RdPos as libc::c_int
                           !=
                           gAudioContext.sampleDmaReuseQueue2WrPos as
                               libc::c_int {
                    // Move the DMA out of the reuse queue, by swapping it with the
                    // read pos, and then incrementing the read pos.
                    if (*dma).reuseIndex as libc::c_int !=
                           gAudioContext.sampleDmaReuseQueue2RdPos as
                               libc::c_int {
                        gAudioContext.sampleDmaReuseQueue2[(*dma).reuseIndex
                                                               as usize] =
                            gAudioContext.sampleDmaReuseQueue2[gAudioContext.sampleDmaReuseQueue2RdPos
                                                                   as usize];
                        (*gAudioContext.sampleDmas.offset(gAudioContext.sampleDmaReuseQueue2[gAudioContext.sampleDmaReuseQueue2RdPos
                                                                                                 as
                                                                                                 usize]
                                                              as
                                                              isize)).reuseIndex
                            = (*dma).reuseIndex
                    }
                    gAudioContext.sampleDmaReuseQueue2RdPos =
                        gAudioContext.sampleDmaReuseQueue2RdPos.wrapping_add(1)
                }
                (*dma).ttl = 32 as libc::c_int as u8_0;
                *dmaIndexRef = i as u8_0;
                return &mut *(*dma).ramAddr.offset(devAddr.wrapping_sub((*dma).devAddr)
                                                       as isize) as *mut u8_0
                           as *mut libc::c_void
            }
            i = i.wrapping_add(1)
        }
        if arg2 == 0 as libc::c_int {
            current_block_37 = 12725306015342310682;
        } else {
            if gAudioContext.sampleDmaReuseQueue2RdPos as libc::c_int !=
                   gAudioContext.sampleDmaReuseQueue2WrPos as libc::c_int &&
                   arg2 != 0 as libc::c_int {
                // Allocate a DMA from reuse queue 2, unless full.
                dmaIndex =
                    gAudioContext.sampleDmaReuseQueue2[gAudioContext.sampleDmaReuseQueue2RdPos
                                                           as usize] as u32_0;
                gAudioContext.sampleDmaReuseQueue2RdPos =
                    gAudioContext.sampleDmaReuseQueue2RdPos.wrapping_add(1);
                dma = gAudioContext.sampleDmas.offset(dmaIndex as isize);
                hasDma = 1 as libc::c_int
            }
            current_block_37 = 9441801433784995173;
        }
    } else { current_block_37 = 12725306015342310682; }
    match current_block_37 {
        12725306015342310682 => {
            dma =
                gAudioContext.sampleDmas.offset(*dmaIndexRef as libc::c_int as
                                                    isize);
            i = 0 as libc::c_int as u32_0;
            loop  {
                bufferPos = devAddr.wrapping_sub((*dma).devAddr) as s32;
                if 0 as libc::c_int <= bufferPos &&
                       bufferPos as u32_0 <=
                           ((*dma).size as libc::c_uint).wrapping_sub(size) {
                    // We already have DMA for this memory range.
                    if (*dma).ttl as libc::c_int == 0 as libc::c_int {
                        // Move the DMA out of the reuse queue, by swapping it with the
                // read pos, and then incrementing the read pos.
                        if (*dma).reuseIndex as libc::c_int !=
                               gAudioContext.sampleDmaReuseQueue1RdPos as
                                   libc::c_int {
                            gAudioContext.sampleDmaReuseQueue1[(*dma).reuseIndex
                                                                   as usize] =
                                gAudioContext.sampleDmaReuseQueue1[gAudioContext.sampleDmaReuseQueue1RdPos
                                                                       as
                                                                       usize];
                            (*gAudioContext.sampleDmas.offset(gAudioContext.sampleDmaReuseQueue1[gAudioContext.sampleDmaReuseQueue1RdPos
                                                                                                     as
                                                                                                     usize]
                                                                  as
                                                                  isize)).reuseIndex
                                = (*dma).reuseIndex
                        }
                        gAudioContext.sampleDmaReuseQueue1RdPos =
                            gAudioContext.sampleDmaReuseQueue1RdPos.wrapping_add(1)
                    }
                    (*dma).ttl = 2 as libc::c_int as u8_0;
                    return (*dma).ramAddr.offset(devAddr.wrapping_sub((*dma).devAddr)
                                                     as isize) as
                               *mut libc::c_void
                }
                let fresh0 = i;
                i = i.wrapping_add(1);
                dma = gAudioContext.sampleDmas.offset(fresh0 as isize);
                if !(i <= gAudioContext.sampleDmaListSize1) { break ; }
            }
        }
        _ => { }
    }
    if hasDma == 0 {
        if gAudioContext.sampleDmaReuseQueue1RdPos as libc::c_int ==
               gAudioContext.sampleDmaReuseQueue1WrPos as libc::c_int {
            return 0 as *mut libc::c_void
        }
        // Allocate a DMA from reuse queue 1.
        let fresh1 = gAudioContext.sampleDmaReuseQueue1RdPos;
        gAudioContext.sampleDmaReuseQueue1RdPos =
            gAudioContext.sampleDmaReuseQueue1RdPos.wrapping_add(1);
        dmaIndex =
            gAudioContext.sampleDmaReuseQueue1[fresh1 as usize] as u32_0;
        dma = gAudioContext.sampleDmas.offset(dmaIndex as isize);
        hasDma = 1 as libc::c_int
    }
    transfer = (*dma).size as u32_0;
    dmaDevAddr = devAddr & !(0xf as libc::c_int) as libc::c_uint;
    (*dma).ttl = 3 as libc::c_int as u8_0;
    (*dma).devAddr = dmaDevAddr;
    (*dma).sizeUnused = transfer as u16_0;
    let fresh2 = gAudioContext.curAudioFrameDmaCount;
    gAudioContext.curAudioFrameDmaCount =
        gAudioContext.curAudioFrameDmaCount + 1;
    AudioLoad_Dma(&mut *gAudioContext.currAudioFrameDmaIoMesgBuf.as_mut_ptr().offset(fresh2
                                                                                         as
                                                                                         isize),
                  0 as libc::c_int as u32_0, 0 as libc::c_int, dmaDevAddr,
                  (*dma).ramAddr as *mut libc::c_void, transfer,
                  &mut gAudioContext.currAudioFrameDmaQueue, medium,
                  b"SUPERDMA\x00" as *const u8 as *const libc::c_char);
    *dmaIndexRef = dmaIndex as u8_0;
    return (*dma).ramAddr.offset(devAddr.wrapping_sub(dmaDevAddr) as isize) as
               *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_InitSampleDmaBuffers(mut arg0: s32) {
    let mut dma: *mut SampleDma = 0 as *mut SampleDma;
    let mut i: s32 = 0;
    let mut t2: s32 = 0;
    let mut j: s32 = 0;
    gAudioContext.sampleDmaBufSize = gAudioContext.sampleDmaBufSize1;
    gAudioContext.sampleDmas =
        AudioHeap_Alloc(&mut gAudioContext.notesAndBuffersPool,
                        ((4 as libc::c_int * gAudioContext.numNotes) as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<SampleDma>()
                                                            as
                                                            libc::c_ulong).wrapping_mul(gAudioContext.audioBufferParameters.specUnk4
                                                                                            as
                                                                                            libc::c_uint))
            as *mut SampleDma;
    t2 =
        3 as libc::c_int * gAudioContext.numNotes *
            gAudioContext.audioBufferParameters.specUnk4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < t2 {
        dma =
            &mut *gAudioContext.sampleDmas.offset(gAudioContext.sampleDmaCount
                                                      as isize) as
                *mut SampleDma;
        (*dma).ramAddr =
            AudioHeap_AllocAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                           gAudioContext.sampleDmaBufSize as
                                               u32_0) as *mut u8_0;
        if (*dma).ramAddr.is_null() { break ; }
        AudioHeap_WritebackDCache((*dma).ramAddr as *mut libc::c_void,
                                  gAudioContext.sampleDmaBufSize as u32_0);
        (*dma).size = gAudioContext.sampleDmaBufSize as u16_0;
        (*dma).devAddr = 0 as libc::c_int as u32_0;
        (*dma).sizeUnused = 0 as libc::c_int as u16_0;
        (*dma).unused = 0 as libc::c_int as u8_0;
        (*dma).ttl = 0 as libc::c_int as u8_0;
        gAudioContext.sampleDmaCount =
            gAudioContext.sampleDmaCount.wrapping_add(1);
        i += 1
    }
    i = 0 as libc::c_int;
    while (i as u32_0) < gAudioContext.sampleDmaCount {
        gAudioContext.sampleDmaReuseQueue1[i as usize] = i as u8_0;
        (*gAudioContext.sampleDmas.offset(i as isize)).reuseIndex = i as u8_0;
        i += 1
    }
    i = gAudioContext.sampleDmaCount as s32;
    while i < 0x100 as libc::c_int {
        gAudioContext.sampleDmaReuseQueue1[i as usize] =
            0 as libc::c_int as u8_0;
        i += 1
    }
    gAudioContext.sampleDmaReuseQueue1RdPos = 0 as libc::c_int as u8_0;
    gAudioContext.sampleDmaReuseQueue1WrPos =
        gAudioContext.sampleDmaCount as u8_0;
    gAudioContext.sampleDmaListSize1 = gAudioContext.sampleDmaCount;
    gAudioContext.sampleDmaBufSize = gAudioContext.sampleDmaBufSize2;
    j = 0 as libc::c_int;
    while j < gAudioContext.numNotes {
        dma =
            &mut *gAudioContext.sampleDmas.offset(gAudioContext.sampleDmaCount
                                                      as isize) as
                *mut SampleDma;
        (*dma).ramAddr =
            AudioHeap_AllocAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                           gAudioContext.sampleDmaBufSize as
                                               u32_0) as *mut u8_0;
        if (*dma).ramAddr.is_null() { break ; }
        AudioHeap_WritebackDCache((*dma).ramAddr as *mut libc::c_void,
                                  gAudioContext.sampleDmaBufSize as u32_0);
        (*dma).size = gAudioContext.sampleDmaBufSize as u16_0;
        (*dma).devAddr = 0 as libc::c_uint;
        (*dma).sizeUnused = 0 as libc::c_int as u16_0;
        (*dma).unused = 0 as libc::c_int as u8_0;
        (*dma).ttl = 0 as libc::c_int as u8_0;
        gAudioContext.sampleDmaCount =
            gAudioContext.sampleDmaCount.wrapping_add(1);
        j += 1
    }
    i = gAudioContext.sampleDmaListSize1 as s32;
    while (i as u32_0) < gAudioContext.sampleDmaCount {
        gAudioContext.sampleDmaReuseQueue2[(i as
                                                libc::c_uint).wrapping_sub(gAudioContext.sampleDmaListSize1)
                                               as usize] = i as u8_0;
        (*gAudioContext.sampleDmas.offset(i as isize)).reuseIndex =
            (i as libc::c_uint).wrapping_sub(gAudioContext.sampleDmaListSize1)
                as u8_0;
        i += 1
    }
    i = gAudioContext.sampleDmaCount as s32;
    while i < 0x100 as libc::c_int {
        gAudioContext.sampleDmaReuseQueue2[i as usize] =
            gAudioContext.sampleDmaListSize1 as u8_0;
        i += 1
    }
    gAudioContext.sampleDmaReuseQueue2RdPos = 0 as libc::c_int as u8_0;
    gAudioContext.sampleDmaReuseQueue2WrPos =
        gAudioContext.sampleDmaCount.wrapping_sub(gAudioContext.sampleDmaListSize1)
            as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_IsFontLoadComplete(mut fontId: s32)
 -> s32 {
    if fontId == 0xff as libc::c_int {
        return 1 as libc::c_int
    } else if gAudioContext.fontLoadStatus[fontId as usize] as libc::c_int >=
                  2 as libc::c_int {
        return 1 as libc::c_int
    } else if gAudioContext.fontLoadStatus[AudioLoad_GetRealTableIndex(FONT_TABLE
                                                                           as
                                                                           libc::c_int,
                                                                       fontId
                                                                           as
                                                                           u32_0)
                                               as usize] as libc::c_int >=
                  2 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_IsSeqLoadComplete(mut seqId: s32) -> s32 {
    if seqId == 0xff as libc::c_int {
        return 1 as libc::c_int
    } else if gAudioContext.seqLoadStatus[seqId as usize] as libc::c_int >=
                  2 as libc::c_int {
        return 1 as libc::c_int
    } else if gAudioContext.seqLoadStatus[AudioLoad_GetRealTableIndex(SEQUENCE_TABLE
                                                                          as
                                                                          libc::c_int,
                                                                      seqId as
                                                                          u32_0)
                                              as usize] as libc::c_int >=
                  2 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_IsSampleLoadComplete(mut sampleBankId: s32)
 -> s32 {
    if sampleBankId == 0xff as libc::c_int {
        return 1 as libc::c_int
    } else if gAudioContext.sampleFontLoadStatus[sampleBankId as usize] as
                  libc::c_int >= 2 as libc::c_int {
        return 1 as libc::c_int
    } else if gAudioContext.sampleFontLoadStatus[AudioLoad_GetRealTableIndex(SAMPLE_TABLE
                                                                                 as
                                                                                 libc::c_int,
                                                                             sampleBankId
                                                                                 as
                                                                                 u32_0)
                                                     as usize] as libc::c_int
                  >= 2 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SetFontLoadStatus(mut fontId: s32,
                                                     mut status: s32) {
    if fontId != 0xff as libc::c_int &&
           gAudioContext.fontLoadStatus[fontId as usize] as libc::c_int !=
               5 as libc::c_int {
        gAudioContext.fontLoadStatus[fontId as usize] = status as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SetSeqLoadStatus(mut seqId: s32,
                                                    mut status: s32) {
    if seqId != 0xff as libc::c_int &&
           gAudioContext.seqLoadStatus[seqId as usize] as libc::c_int !=
               5 as libc::c_int {
        gAudioContext.seqLoadStatus[seqId as usize] = status as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SetSampleFontLoadStatusAndApplyCaches(mut sampleBankId:
                                                                             s32,
                                                                         mut status:
                                                                             s32) {
    if sampleBankId != 0xff as libc::c_int {
        if gAudioContext.sampleFontLoadStatus[sampleBankId as usize] as
               libc::c_int != 5 as libc::c_int {
            gAudioContext.sampleFontLoadStatus[sampleBankId as usize] =
                status as u8_0
        }
        if gAudioContext.sampleFontLoadStatus[sampleBankId as usize] as
               libc::c_int == 5 as libc::c_int ||
               gAudioContext.sampleFontLoadStatus[sampleBankId as usize] as
                   libc::c_int == 2 as libc::c_int {
            AudioHeap_ApplySampleBankCache(sampleBankId);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SetSampleFontLoadStatus(mut sampleBankId:
                                                               s32,
                                                           mut status: s32) {
    if sampleBankId != 0xff as libc::c_int &&
           gAudioContext.sampleFontLoadStatus[sampleBankId as usize] as
               libc::c_int != 5 as libc::c_int {
        gAudioContext.sampleFontLoadStatus[sampleBankId as usize] =
            status as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_InitTable(mut table: *mut AudioTable,
                                             mut romAddr: u32_0,
                                             mut unkMediumParam: u16_0) {
    let mut i: s32 = 0;
    (*table).unkMediumParam = unkMediumParam as s16;
    (*table).romAddr = romAddr;
    i = 0 as libc::c_int;
    while i < (*table).numEntries as libc::c_int {
        if (*(*table).entries.as_mut_ptr().offset(i as isize)).size !=
               0 as libc::c_int as libc::c_uint &&
               (*(*table).entries.as_mut_ptr().offset(i as isize)).medium as
                   libc::c_int == MEDIUM_CART as libc::c_int {
            let ref mut fresh3 =
                (*(*table).entries.as_mut_ptr().offset(i as isize)).romAddr;
            *fresh3 =
                (*fresh3 as libc::c_uint).wrapping_add(romAddr) as u32_0 as
                    u32_0
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoadSeqFonts(mut seqId: s32,
                                                    mut outDefaultFontId:
                                                        *mut u32_0)
 -> *mut libc::c_void {
    let mut pad: [libc::c_char; 8] = [0; 8];
    let mut index: s32 = 0;
    let mut font: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut numFonts: s32 = 0;
    let mut fontId: s32 = 0;
    let mut i: s32 = 0;
    if seqId >= gAudioContext.numSequences as libc::c_int {
        return 0 as *mut libc::c_void
    }
    fontId = 0xff as libc::c_int;
    index =
        *(gAudioContext.sequenceFontTable as
              *mut u16_0).offset(seqId as isize) as s32;
    let fresh4 = index;
    index = index + 1;
    numFonts =
        *gAudioContext.sequenceFontTable.offset(fresh4 as isize) as s32;
    while numFonts > 0 as libc::c_int {
        let fresh5 = index;
        index = index + 1;
        fontId =
            *gAudioContext.sequenceFontTable.offset(fresh5 as isize) as s32;
        font = AudioLoad_SyncLoadFont(fontId as u32_0);
        numFonts -= 1
    }
    *outDefaultFontId = fontId as u32_0;
    return font;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoadSeqParts(mut seqId: s32,
                                                    mut arg1: s32) {
    let mut pad: s32 = 0;
    let mut defaultFontId: u32_0 = 0;
    if seqId < gAudioContext.numSequences as libc::c_int {
        if arg1 & 2 as libc::c_int != 0 {
            AudioLoad_SyncLoadSeqFonts(seqId, &mut defaultFontId);
        }
        if arg1 & 1 as libc::c_int != 0 { AudioLoad_SyncLoadSeq(seqId); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoadSample(mut sample:
                                                      *mut SoundFontSample,
                                                  mut fontId: s32) -> s32 {
    let mut sampleAddr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*sample).unk_bit25() as libc::c_int == 1 as libc::c_int {
        if (*sample).medium() as libc::c_int != MEDIUM_RAM as libc::c_int {
            sampleAddr =
                AudioHeap_AllocSampleCache((*sample).size(), fontId,
                                           (*sample).sampleAddr as
                                               *mut libc::c_void,
                                           (*sample).medium() as s8,
                                           CACHE_PERSISTENT as libc::c_int);
            if sampleAddr.is_null() { return -(1 as libc::c_int) }
            if (*sample).medium() as libc::c_int == MEDIUM_UNK as libc::c_int
               {
                AudioLoad_SyncDmaUnkMedium((*sample).sampleAddr as u32_0,
                                           sampleAddr as *mut u8_0,
                                           (*sample).size(),
                                           (*gAudioContext.sampleBankTable).unkMediumParam
                                               as s32);
            } else {
                AudioLoad_SyncDma((*sample).sampleAddr as u32_0,
                                  sampleAddr as *mut u8_0, (*sample).size(),
                                  (*sample).medium() as s32);
            }
            (*sample).set_medium(MEDIUM_RAM as libc::c_int as u32_0);
            (*sample).sampleAddr = sampleAddr as *mut u8_0
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoadInstrument(mut fontId: s32,
                                                      mut instId: s32,
                                                      mut drumId: s32)
 -> s32 {
    if instId < 0x7f as libc::c_int {
        let mut instrument: *mut Instrument =
            Audio_GetInstrumentInner(fontId, instId);
        if instrument.is_null() { return -(1 as libc::c_int) }
        if (*instrument).normalRangeLo as libc::c_int != 0 as libc::c_int {
            AudioLoad_SyncLoadSample((*instrument).lowNotesSound.sample,
                                     fontId);
        }
        AudioLoad_SyncLoadSample((*instrument).normalNotesSound.sample,
                                 fontId);
        if (*instrument).normalRangeHi as libc::c_int != 0x7f as libc::c_int {
            return AudioLoad_SyncLoadSample((*instrument).highNotesSound.sample,
                                            fontId)
        }
    } else if instId == 0x7f as libc::c_int {
        let mut drum: *mut Drum = Audio_GetDrum(fontId, drumId);
        if drum.is_null() { return -(1 as libc::c_int) }
        AudioLoad_SyncLoadSample((*drum).sound.sample, fontId);
        return 0 as libc::c_int
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AsyncLoad(mut tableType: s32, mut id: s32,
                                             mut nChunks: s32,
                                             mut retData: s32,
                                             mut retQueue: *mut OSMesgQueue) {
    if AudioLoad_AsyncLoadInner(tableType, id, nChunks, retData,
                                retQueue).is_null() {
        osSendMesg(retQueue, 0xffffffff as libc::c_uint as OSMesg,
                   0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AsyncLoadSeq(mut seqId: s32, mut arg1: s32,
                                                mut retData: s32,
                                                mut retQueue:
                                                    *mut OSMesgQueue) {
    AudioLoad_AsyncLoad(SEQUENCE_TABLE as libc::c_int, seqId,
                        0 as libc::c_int, retData, retQueue);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AsyncLoadSampleBank(mut sampleBankId: s32,
                                                       mut arg1: s32,
                                                       mut retData: s32,
                                                       mut retQueue:
                                                           *mut OSMesgQueue) {
    AudioLoad_AsyncLoad(SAMPLE_TABLE as libc::c_int, sampleBankId,
                        0 as libc::c_int, retData, retQueue);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AsyncLoadFont(mut fontId: s32,
                                                 mut arg1: s32,
                                                 mut retData: s32,
                                                 mut retQueue:
                                                     *mut OSMesgQueue) {
    AudioLoad_AsyncLoad(FONT_TABLE as libc::c_int, fontId, 0 as libc::c_int,
                        retData, retQueue);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_GetFontsForSequence(mut seqId: s32,
                                                       mut outNumFonts:
                                                           *mut u32_0)
 -> *mut u8_0 {
    let mut index: s32 = 0;
    index =
        *(gAudioContext.sequenceFontTable as
              *mut u16_0).offset(seqId as isize) as s32;
    let fresh6 = index;
    index = index + 1;
    *outNumFonts =
        *gAudioContext.sequenceFontTable.offset(fresh6 as isize) as u32_0;
    if *outNumFonts == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut u8_0
    }
    return &mut *gAudioContext.sequenceFontTable.offset(index as isize) as
               *mut u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_DiscardSeqFonts(mut seqId: s32) {
    let mut fontId: s32 = 0;
    let mut index: s32 = 0;
    let mut numFonts: s32 = 0;
    index =
        *(gAudioContext.sequenceFontTable as
              *mut u16_0).offset(seqId as isize) as s32;
    let fresh7 = index;
    index = index + 1;
    numFonts =
        *gAudioContext.sequenceFontTable.offset(fresh7 as isize) as s32;
    while numFonts > 0 as libc::c_int {
        numFonts -= 1;
        let fresh8 = index;
        index = index + 1;
        fontId =
            AudioLoad_GetRealTableIndex(FONT_TABLE as libc::c_int,
                                        *gAudioContext.sequenceFontTable.offset(fresh8
                                                                                    as
                                                                                    isize)
                                            as u32_0) as s32;
        if AudioHeap_SearchPermanentCache(FONT_TABLE as libc::c_int,
                                          fontId).is_null() {
            AudioLoad_DiscardFont(fontId);
            AudioLoad_SetFontLoadStatus(fontId, 0 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_DiscardFont(mut fontId: s32) {
    let mut i: u32_0 = 0;
    let mut pool: *mut AudioCache = &mut gAudioContext.fontCache;
    let mut persistent: *mut AudioPersistentCache =
        0 as *mut AudioPersistentCache;
    if fontId ==
           (*pool).temporary.entries[0 as libc::c_int as usize].id as
               libc::c_int {
        (*pool).temporary.entries[0 as libc::c_int as usize].id =
            -(1 as libc::c_int) as s16
    } else if fontId ==
                  (*pool).temporary.entries[1 as libc::c_int as usize].id as
                      libc::c_int {
        (*pool).temporary.entries[1 as libc::c_int as usize].id =
            -(1 as libc::c_int) as s16
    }
    persistent = &mut (*pool).persistent;
    i = 0 as libc::c_int as u32_0;
    while i < (*persistent).numEntries {
        if fontId == (*persistent).entries[i as usize].id as libc::c_int {
            (*persistent).entries[i as usize].id = -(1 as libc::c_int) as s16
        }
        i = i.wrapping_add(1)
    }
    AudioHeap_DiscardFont(fontId);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncInitSeqPlayer(mut playerIdx: s32,
                                                     mut seqId: s32,
                                                     mut arg2: s32) -> s32 {
    if gAudioContext.resetTimer != 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    gAudioContext.seqPlayers[playerIdx as usize].skipTicks = 0 as libc::c_int;
    AudioLoad_SyncInitSeqPlayerInternal(playerIdx, seqId, arg2);
    panic!("Reached end of non-void function without returning");
    // Intentionally missing return. Returning the result of the above function
    // call matches but is UB because it too is missing a return, and using the
    // result of a non-void function that has failed to return a value is UB.
    // The callers of this function do not use the return value, so it's fine.
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncInitSeqPlayerSkipTicks(mut playerIdx:
                                                                  s32,
                                                              mut seqId: s32,
                                                              mut skipTicks:
                                                                  s32)
 -> s32 {
    if gAudioContext.resetTimer != 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    gAudioContext.seqPlayers[playerIdx as usize].skipTicks = skipTicks;
    AudioLoad_SyncInitSeqPlayerInternal(playerIdx, seqId, 0 as libc::c_int);
    panic!("Reached end of non-void function without returning");
    // Missing return, see above.
}
/* forward declarations */
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncInitSeqPlayerInternal(mut playerIdx:
                                                                 s32,
                                                             mut seqId: s32,
                                                             mut arg2: s32)
 -> s32 {
    let mut seqPlayer: *mut SequencePlayer =
        &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(playerIdx as isize)
            as *mut SequencePlayer;
    let mut seqData: *mut u8_0 = 0 as *mut u8_0;
    let mut index: s32 = 0;
    let mut numFonts: s32 = 0;
    let mut fontId: s32 = 0;
    if seqId >= gAudioContext.numSequences as libc::c_int {
        return 0 as libc::c_int
    }
    AudioSeq_SequencePlayerDisable(seqPlayer);
    fontId = 0xff as libc::c_int;
    index =
        *(gAudioContext.sequenceFontTable as
              *mut u16_0).offset(seqId as isize) as s32;
    let fresh9 = index;
    index = index + 1;
    numFonts =
        *gAudioContext.sequenceFontTable.offset(fresh9 as isize) as s32;
    while numFonts > 0 as libc::c_int {
        let fresh10 = index;
        index = index + 1;
        fontId =
            *gAudioContext.sequenceFontTable.offset(fresh10 as isize) as s32;
        AudioLoad_SyncLoadFont(fontId as u32_0);
        numFonts -= 1
    }
    seqData = AudioLoad_SyncLoadSeq(seqId);
    if seqData.is_null() { return 0 as libc::c_int }
    AudioSeq_ResetSequencePlayer(seqPlayer);
    (*seqPlayer).seqId = seqId as u8_0;
    (*seqPlayer).defaultFont =
        AudioLoad_GetRealTableIndex(FONT_TABLE as libc::c_int,
                                    fontId as u32_0) as u8_0;
    (*seqPlayer).seqData = seqData;
    (*seqPlayer).set_enabled(1 as libc::c_int as u8_0);
    (*seqPlayer).scriptState.pc = seqData;
    (*seqPlayer).scriptState.depth = 0 as libc::c_int as u8_0;
    (*seqPlayer).delay = 0 as libc::c_int as u16_0;
    (*seqPlayer).set_finished(0 as libc::c_int as u8_0);
    (*seqPlayer).playerIdx = playerIdx as s8;
    AudioSeq_SkipForwardSequence(seqPlayer);
    panic!("Reached end of non-void function without returning");
    // ! @bug missing return (but the return value is not used so it's not UB)
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoadSeq(mut seqId: s32) -> *mut u8_0 {
    let mut pad: s32 = 0;
    let mut didAllocate: s32 = 0;
    if gAudioContext.seqLoadStatus[AudioLoad_GetRealTableIndex(SEQUENCE_TABLE
                                                                   as
                                                                   libc::c_int,
                                                               seqId as u32_0)
                                       as usize] as libc::c_int ==
           1 as libc::c_int {
        return 0 as *mut u8_0
    }
    return AudioLoad_SyncLoad(SEQUENCE_TABLE as libc::c_int as u32_0,
                              seqId as u32_0, &mut didAllocate) as *mut u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_GetSampleBank(mut sampleBankId: u32_0,
                                                 mut outMedium: *mut u32_0)
 -> u32_0 {
    return AudioLoad_TrySyncLoadSampleBank(sampleBankId, outMedium,
                                           1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_TrySyncLoadSampleBank(mut sampleBankId:
                                                             u32_0,
                                                         mut outMedium:
                                                             *mut u32_0,
                                                         mut noLoad: s32)
 -> u32_0 {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sampleBankTable: *mut AudioTable = 0 as *mut AudioTable;
    let mut realTableId: u32_0 =
        AudioLoad_GetRealTableIndex(SAMPLE_TABLE as libc::c_int,
                                    sampleBankId);
    let mut cachePolicy: s8 = 0;
    sampleBankTable = AudioLoad_GetLoadTable(SAMPLE_TABLE as libc::c_int);
    ret =
        AudioLoad_SearchCaches(SAMPLE_TABLE as libc::c_int,
                               realTableId as s32);
    if !ret.is_null() {
        if gAudioContext.sampleFontLoadStatus[realTableId as usize] as
               libc::c_int != 1 as libc::c_int {
            AudioLoad_SetSampleFontLoadStatus(realTableId as s32,
                                              2 as libc::c_int);
        }
        *outMedium = MEDIUM_RAM as libc::c_int as u32_0;
        return ret as u32_0
    }
    cachePolicy =
        (*(*sampleBankTable).entries.as_mut_ptr().offset(sampleBankId as
                                                             isize)).cachePolicy;
    if cachePolicy as libc::c_int == 4 as libc::c_int ||
           noLoad == 1 as libc::c_int {
        *outMedium =
            (*(*sampleBankTable).entries.as_mut_ptr().offset(sampleBankId as
                                                                 isize)).medium
                as u32_0;
        return (*(*sampleBankTable).entries.as_mut_ptr().offset(realTableId as
                                                                    isize)).romAddr
    }
    ret =
        AudioLoad_SyncLoad(SAMPLE_TABLE as libc::c_int as u32_0, sampleBankId,
                           &mut noLoad);
    if !ret.is_null() {
        *outMedium = MEDIUM_RAM as libc::c_int as u32_0;
        return ret as u32_0
    }
    *outMedium =
        (*(*sampleBankTable).entries.as_mut_ptr().offset(sampleBankId as
                                                             isize)).medium as
            u32_0;
    return (*(*sampleBankTable).entries.as_mut_ptr().offset(realTableId as
                                                                isize)).romAddr;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoadFont(mut fontId: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sampleBankId1: s32 = 0;
    let mut sampleBankId2: s32 = 0;
    let mut didAllocate: s32 = 0;
    let mut relocInfo: RelocInfo =
        RelocInfo{sampleBankId1: 0,
                  sampleBankId2: 0,
                  baseAddr1: 0,
                  baseAddr2: 0,
                  medium1: 0,
                  medium2: 0,};
    let mut realFontId: s32 =
        AudioLoad_GetRealTableIndex(FONT_TABLE as libc::c_int, fontId) as s32;
    if gAudioContext.fontLoadStatus[realFontId as usize] as libc::c_int ==
           1 as libc::c_int {
        return 0 as *mut libc::c_void
    }
    sampleBankId1 =
        (*gAudioContext.soundFonts.offset(realFontId as isize)).sampleBankId1
            as s32;
    sampleBankId2 =
        (*gAudioContext.soundFonts.offset(realFontId as isize)).sampleBankId2
            as s32;
    relocInfo.sampleBankId1 = sampleBankId1;
    relocInfo.sampleBankId2 = sampleBankId2;
    if sampleBankId1 != 0xff as libc::c_int {
        relocInfo.baseAddr1 =
            AudioLoad_TrySyncLoadSampleBank(sampleBankId1 as u32_0,
                                            &mut relocInfo.medium1,
                                            0 as libc::c_int) as s32
    } else { relocInfo.baseAddr1 = 0 as libc::c_int }
    if sampleBankId2 != 0xff as libc::c_int {
        relocInfo.baseAddr2 =
            AudioLoad_TrySyncLoadSampleBank(sampleBankId2 as u32_0,
                                            &mut relocInfo.medium2,
                                            0 as libc::c_int) as s32
    } else { relocInfo.baseAddr2 = 0 as libc::c_int }
    ret =
        AudioLoad_SyncLoad(FONT_TABLE as libc::c_int as u32_0, fontId,
                           &mut didAllocate);
    if ret.is_null() { return 0 as *mut libc::c_void }
    if didAllocate == 1 as libc::c_int {
        AudioLoad_RelocateFontAndPreloadSamples(realFontId, ret,
                                                &mut relocInfo,
                                                0 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoad(mut tableType: u32_0,
                                            mut id: u32_0,
                                            mut didAllocate: *mut s32)
 -> *mut libc::c_void {
    let mut size: u32_0 = 0;
    let mut table: *mut AudioTable = 0 as *mut AudioTable;
    let mut pad: s32 = 0;
    let mut medium: u32_0 = 0;
    let mut status: s32 = 0;
    let mut romAddr: u32_0 = 0;
    let mut cachePolicy: s32 = 0;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut realId: u32_0 = 0;
    realId = AudioLoad_GetRealTableIndex(tableType as s32, id);
    ret = AudioLoad_SearchCaches(tableType as s32, realId as s32);
    if !ret.is_null() {
        *didAllocate = 0 as libc::c_int;
        status = 2 as libc::c_int
    } else {
        table = AudioLoad_GetLoadTable(tableType as s32);
        size = (*(*table).entries.as_mut_ptr().offset(realId as isize)).size;
        size =
            size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
                !(0xf as libc::c_int) as libc::c_uint;
        medium =
            (*(*table).entries.as_mut_ptr().offset(id as isize)).medium as
                u32_0;
        cachePolicy =
            (*(*table).entries.as_mut_ptr().offset(id as isize)).cachePolicy
                as s32;
        romAddr =
            (*(*table).entries.as_mut_ptr().offset(realId as isize)).romAddr;
        match cachePolicy {
            0 => {
                ret =
                    AudioHeap_AllocPermanent(tableType as s32, realId as s32,
                                             size);
                if ret.is_null() { return ret }
            }
            1 => {
                ret =
                    AudioHeap_AllocCached(tableType as s32, size as s32,
                                          CACHE_PERSISTENT as libc::c_int,
                                          realId as s32);
                if ret.is_null() { return ret }
            }
            2 => {
                ret =
                    AudioHeap_AllocCached(tableType as s32, size as s32,
                                          CACHE_TEMPORARY as libc::c_int,
                                          realId as s32);
                if ret.is_null() { return ret }
            }
            3 | 4 => {
                ret =
                    AudioHeap_AllocCached(tableType as s32, size as s32,
                                          CACHE_EITHER as libc::c_int,
                                          realId as s32);
                if ret.is_null() { return ret }
            }
            _ => { }
        }
        *didAllocate = 1 as libc::c_int;
        if medium == MEDIUM_UNK as libc::c_int as libc::c_uint {
            AudioLoad_SyncDmaUnkMedium(romAddr, ret as *mut u8_0, size,
                                       (*table).unkMediumParam as s32);
        } else {
            AudioLoad_SyncDma(romAddr, ret as *mut u8_0, size, medium as s32);
        }
        status =
            if cachePolicy == 0 as libc::c_int {
                5 as libc::c_int
            } else { 2 as libc::c_int }
    }
    match tableType {
        0 => { AudioLoad_SetSeqLoadStatus(realId as s32, status); }
        1 => { AudioLoad_SetFontLoadStatus(realId as s32, status); }
        2 => {
            AudioLoad_SetSampleFontLoadStatusAndApplyCaches(realId as s32,
                                                            status);
        }
        _ => { }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_GetRealTableIndex(mut tableType: s32,
                                                     mut id: u32_0) -> u32_0 {
    let mut table: *mut AudioTable = AudioLoad_GetLoadTable(tableType);
    if (*(*table).entries.as_mut_ptr().offset(id as isize)).size ==
           0 as libc::c_int as libc::c_uint {
        id = (*(*table).entries.as_mut_ptr().offset(id as isize)).romAddr
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SearchCaches(mut tableType: s32,
                                                mut id: s32)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = AudioHeap_SearchPermanentCache(tableType, id);
    if !ret.is_null() { return ret }
    ret = AudioHeap_SearchCaches(tableType, CACHE_EITHER as libc::c_int, id);
    if !ret.is_null() { return ret }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_GetLoadTable(mut tableType: s32)
 -> *mut AudioTable {
    let mut ret: *mut AudioTable = 0 as *mut AudioTable;
    match tableType {
        0 => { ret = gAudioContext.sequenceTable }
        1 => { ret = gAudioContext.soundFontTable }
        2 => { ret = gAudioContext.sampleBankTable }
        _ => { ret = 0 as *mut AudioTable }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_RelocateFont(mut fontId: s32,
                                                mut mem: *mut libc::c_void,
                                                mut relocInfo:
                                                    *mut RelocInfo) {
    let mut reloc: u32_0 = 0;
    let mut reloc2: u32_0 = 0;
    let mut inst: *mut Instrument = 0 as *mut Instrument;
    let mut drum: *mut Drum = 0 as *mut Drum;
    let mut sfx: *mut SoundFontSound = 0 as *mut SoundFontSound;
    let mut i: s32 = 0;
    let mut numDrums: s32 =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numDrums as s32;
    let mut numInstruments: s32 =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numInstruments as
            s32;
    let mut numSfx: s32 =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numSfx as s32;
    let mut ptrs: *mut *mut libc::c_void = mem as *mut *mut libc::c_void;
    reloc2 = *ptrs.offset(0 as libc::c_int as isize) as u32_0;
    if reloc2 != 0 as libc::c_int as libc::c_uint &&
           numDrums != 0 as libc::c_int {
        let ref mut fresh11 = *ptrs.offset(0 as libc::c_int as isize);
        *fresh11 = reloc2.wrapping_add(mem as u32_0) as *mut libc::c_void;
        i = 0 as libc::c_int;
        while i < numDrums {
            reloc =
                *(*ptrs.offset(0 as libc::c_int as isize) as
                      *mut *mut Drum).offset(i as isize) as u32_0;
            if reloc != 0 as libc::c_int as libc::c_uint {
                reloc =
                    reloc.wrapping_add(mem as u32_0) as *mut libc::c_void as
                        u32_0;
                drum = reloc as *mut Drum;
                let ref mut fresh12 =
                    *(*ptrs.offset(0 as libc::c_int as isize) as
                          *mut *mut Drum).offset(i as isize);
                *fresh12 = drum;
                if (*drum).loaded == 0 {
                    AudioLoad_RelocateSample(&mut (*drum).sound, mem,
                                             relocInfo);
                    reloc = (*drum).envelope as u32_0;
                    (*drum).envelope =
                        reloc.wrapping_add(mem as u32_0) as *mut libc::c_void
                            as *mut AdsrEnvelope;
                    (*drum).loaded = 1 as libc::c_int as u8_0
                }
            }
            i += 1
        }
    }
    reloc2 = *ptrs.offset(1 as libc::c_int as isize) as u32_0;
    if reloc2 != 0 as libc::c_int as libc::c_uint &&
           numSfx != 0 as libc::c_int {
        let ref mut fresh13 = *ptrs.offset(1 as libc::c_int as isize);
        *fresh13 = reloc2.wrapping_add(mem as u32_0) as *mut libc::c_void;
        i = 0 as libc::c_int;
        while i < numSfx {
            reloc =
                (*ptrs.offset(1 as libc::c_int as isize) as
                     *mut SoundFontSound).offset(i as isize) as u32_0;
            if reloc != 0 as libc::c_int as libc::c_uint {
                sfx = reloc as *mut SoundFontSound;
                if !(*sfx).sample.is_null() {
                    AudioLoad_RelocateSample(sfx, mem, relocInfo);
                }
            }
            i += 1
        }
    }
    if numInstruments > 0x7e as libc::c_int {
        numInstruments = 0x7e as libc::c_int
    }
    i = 2 as libc::c_int;
    while i <= 2 as libc::c_int + numInstruments - 1 as libc::c_int {
        if !(*ptrs.offset(i as isize)).is_null() {
            let ref mut fresh14 = *ptrs.offset(i as isize);
            *fresh14 =
                (*ptrs.offset(i as isize) as u32_0).wrapping_add(mem as u32_0)
                    as *mut libc::c_void;
            inst = *ptrs.offset(i as isize) as *mut Instrument;
            if (*inst).loaded == 0 {
                if (*inst).normalRangeLo as libc::c_int != 0 as libc::c_int {
                    AudioLoad_RelocateSample(&mut (*inst).lowNotesSound, mem,
                                             relocInfo);
                }
                AudioLoad_RelocateSample(&mut (*inst).normalNotesSound, mem,
                                         relocInfo);
                if (*inst).normalRangeHi as libc::c_int != 0x7f as libc::c_int
                   {
                    AudioLoad_RelocateSample(&mut (*inst).highNotesSound, mem,
                                             relocInfo);
                }
                reloc = (*inst).envelope as u32_0;
                (*inst).envelope =
                    reloc.wrapping_add(mem as u32_0) as *mut libc::c_void as
                        *mut AdsrEnvelope;
                (*inst).loaded = 1 as libc::c_int as u8_0
            }
        }
        i += 1
    }
    let ref mut fresh15 =
        (*gAudioContext.soundFonts.offset(fontId as isize)).drums;
    *fresh15 = *ptrs.offset(0 as libc::c_int as isize) as *mut *mut Drum;
    let ref mut fresh16 =
        (*gAudioContext.soundFonts.offset(fontId as isize)).soundEffects;
    *fresh16 = *ptrs.offset(1 as libc::c_int as isize) as *mut SoundFontSound;
    let ref mut fresh17 =
        (*gAudioContext.soundFonts.offset(fontId as isize)).instruments;
    *fresh17 = ptrs.offset(2 as libc::c_int as isize) as *mut *mut Instrument;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncDma(mut devAddr: u32_0,
                                           mut addr: *mut u8_0,
                                           mut size: u32_0, mut medium: s32) {
    let mut msgQueue: *mut OSMesgQueue = &mut gAudioContext.syncDmaQueue;
    let mut ioMesg: *mut OSIoMesg = &mut gAudioContext.syncDmaIoMesg;
    size =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    Audio_InvalDCache(addr as *mut libc::c_void, size as s32);
    while !(size < 0x400 as libc::c_int as libc::c_uint) {
        AudioLoad_Dma(ioMesg, 1 as libc::c_int as u32_0, 0 as libc::c_int,
                      devAddr, addr as *mut libc::c_void,
                      0x400 as libc::c_int as u32_0, msgQueue, medium,
                      b"FastCopy\x00" as *const u8 as *const libc::c_char);
        osRecvMesg(msgQueue, 0 as *mut OSMesg, 1 as libc::c_int);
        size =
            (size as
                 libc::c_uint).wrapping_sub(0x400 as libc::c_int as
                                                libc::c_uint) as u32_0 as
                u32_0;
        devAddr =
            (devAddr as
                 libc::c_uint).wrapping_add(0x400 as libc::c_int as
                                                libc::c_uint) as u32_0 as
                u32_0;
        addr = addr.offset(0x400 as libc::c_int as isize)
    }
    if size != 0 as libc::c_int as libc::c_uint {
        AudioLoad_Dma(ioMesg, 1 as libc::c_int as u32_0, 0 as libc::c_int,
                      devAddr, addr as *mut libc::c_void, size, msgQueue,
                      medium,
                      b"FastCopy\x00" as *const u8 as *const libc::c_char);
        osRecvMesg(msgQueue, 0 as *mut OSMesg, 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncDmaUnkMedium(mut devAddr: u32_0,
                                                    mut addr: *mut u8_0,
                                                    mut size: u32_0,
                                                    mut unkMediumParam: s32) {
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_Dma(mut mesg: *mut OSIoMesg,
                                       mut priority: u32_0,
                                       mut direction: s32, mut devAddr: u32_0,
                                       mut ramAddr: *mut libc::c_void,
                                       mut size: u32_0,
                                       mut reqQueue: *mut OSMesgQueue,
                                       mut medium: s32,
                                       mut dmaFuncType: *const libc::c_char)
 -> s32 {
    let mut handle: *mut OSPiHandle = 0 as *mut OSPiHandle;
    if gAudioContext.resetTimer > 0x10 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    match medium {
        2 => { handle = gAudioContext.cartHandle }
        3 => {
            // driveHandle is uninitialized and corresponds to stubbed-out disk drive support.
            // SM64 Shindou called osDriveRomInit here.
            handle = gAudioContext.driveHandle
        }
        _ => { return 0 as libc::c_int }
    }
    if size.wrapping_rem(0x10 as libc::c_int as libc::c_uint) !=
           0 as libc::c_int as libc::c_uint {
        size =
            size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
                !(0xf as libc::c_int) as libc::c_uint
    }
    (*mesg).hdr.pri = priority as u8_0;
    (*mesg).hdr.retQueue = reqQueue;
    (*mesg).dramAddr = ramAddr;
    (*mesg).devAddr = devAddr;
    (*mesg).size = size as size_t;
    (*handle).transferInfo.cmdType = 2 as libc::c_int as u32_0;
    sDmaHandler.expect("non-null function pointer")(handle, mesg, direction);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_Unused1() { }
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SyncLoadSimple(mut tableType: u32_0,
                                                  mut fontId: u32_0) {
    let mut didAllocate: s32 = 0;
    AudioLoad_SyncLoad(tableType, fontId, &mut didAllocate);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AsyncLoadInner(mut tableType: s32,
                                                  mut id: s32,
                                                  mut nChunks: s32,
                                                  mut retData: s32,
                                                  mut retQueue:
                                                      *mut OSMesgQueue)
 -> *mut libc::c_void {
    let mut size: u32_0 = 0;
    let mut sp50: *mut AudioTable = 0 as *mut AudioTable;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut medium: s32 = 0;
    let mut cachePolicy: s8 = 0;
    let mut devAddr: u32_0 = 0;
    let mut status: s32 = 0;
    let mut temp_v0: u32_0 = 0;
    let mut realId: u32_0 = 0;
    realId = AudioLoad_GetRealTableIndex(tableType, id as u32_0);
    match tableType {
        0 => {
            if gAudioContext.seqLoadStatus[realId as usize] as libc::c_int ==
                   1 as libc::c_int {
                return 0 as *mut libc::c_void
            }
        }
        1 => {
            if gAudioContext.fontLoadStatus[realId as usize] as libc::c_int ==
                   1 as libc::c_int {
                return 0 as *mut libc::c_void
            }
        }
        2 => {
            if gAudioContext.sampleFontLoadStatus[realId as usize] as
                   libc::c_int == 1 as libc::c_int {
                return 0 as *mut libc::c_void
            }
        }
        _ => { }
    }
    ret = AudioLoad_SearchCaches(tableType, realId as s32);
    if !ret.is_null() {
        status = 2 as libc::c_int;
        osSendMesg(retQueue,
                   (retData << 24 as libc::c_int |
                        (0 as libc::c_int) << 16 as libc::c_int |
                        (0 as libc::c_int) << 8 as libc::c_int |
                        0 as libc::c_int) as OSMesg, 0 as libc::c_int);
    } else {
        sp50 = AudioLoad_GetLoadTable(tableType);
        size = (*(*sp50).entries.as_mut_ptr().offset(realId as isize)).size;
        size =
            size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
                !(0xf as libc::c_int) as libc::c_uint;
        medium =
            (*(*sp50).entries.as_mut_ptr().offset(id as isize)).medium as s32;
        cachePolicy =
            (*(*sp50).entries.as_mut_ptr().offset(id as isize)).cachePolicy;
        devAddr =
            (*(*sp50).entries.as_mut_ptr().offset(realId as isize)).romAddr;
        status = 2 as libc::c_int;
        match cachePolicy as libc::c_int {
            0 => {
                ret =
                    AudioHeap_AllocPermanent(tableType, realId as s32, size);
                if ret.is_null() { return ret }
                status = 5 as libc::c_int
            }
            1 => {
                ret =
                    AudioHeap_AllocCached(tableType, size as s32,
                                          CACHE_PERSISTENT as libc::c_int,
                                          realId as s32);
                if ret.is_null() { return ret }
            }
            2 => {
                ret =
                    AudioHeap_AllocCached(tableType, size as s32,
                                          CACHE_TEMPORARY as libc::c_int,
                                          realId as s32);
                if ret.is_null() { return ret }
            }
            3 | 4 => {
                ret =
                    AudioHeap_AllocCached(tableType, size as s32,
                                          CACHE_EITHER as libc::c_int,
                                          realId as s32);
                if ret.is_null() { return ret }
            }
            _ => { }
        }
        if medium == MEDIUM_UNK as libc::c_int {
            AudioLoad_StartAsyncLoadUnkMedium((*sp50).unkMediumParam as s32,
                                              devAddr, ret, size as s32,
                                              medium, nChunks, retQueue,
                                              retData << 24 as libc::c_int |
                                                  tableType <<
                                                      16 as libc::c_int |
                                                  id << 8 as libc::c_int |
                                                  status);
        } else {
            AudioLoad_StartAsyncLoad(devAddr, ret, size, medium, nChunks,
                                     retQueue,
                                     ((retData << 24 as libc::c_int |
                                           tableType << 16 as libc::c_int) as
                                          libc::c_uint |
                                          realId << 8 as libc::c_int |
                                          status as libc::c_uint) as s32);
        }
        status = 1 as libc::c_int
    }
    match tableType {
        0 => { AudioLoad_SetSeqLoadStatus(realId as s32, status); }
        1 => { AudioLoad_SetFontLoadStatus(realId as s32, status); }
        2 => {
            AudioLoad_SetSampleFontLoadStatusAndApplyCaches(realId as s32,
                                                            status);
        }
        _ => { }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ProcessLoads(mut resetStatus: s32) {
    AudioLoad_ProcessSlowLoads(resetStatus);
    AudioLoad_ProcessSamplePreloads(resetStatus);
    AudioLoad_ProcessAsyncLoads(resetStatus);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SetDmaHandler(mut callback: DmaHandler) {
    sDmaHandler = callback;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SetUnusedHandler(mut callback:
                                                        *mut libc::c_void) {
    sUnusedHandler = callback;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_InitSoundFontMeta(mut fontId: s32) {
    let mut font: *mut SoundFont =
        &mut *gAudioContext.soundFonts.offset(fontId as isize) as
            *mut SoundFont;
    let mut entry: *mut AudioTableEntry =
        &mut *(*gAudioContext.soundFontTable).entries.as_mut_ptr().offset(fontId
                                                                              as
                                                                              isize)
            as *mut AudioTableEntry;
    (*font).sampleBankId1 =
        ((*entry).shortData1 as libc::c_int >> 8 as libc::c_int &
             0xff as libc::c_int) as u8_0;
    (*font).sampleBankId2 =
        ((*entry).shortData1 as libc::c_int & 0xff as libc::c_int) as u8_0;
    (*font).numInstruments =
        ((*entry).shortData2 as libc::c_int >> 8 as libc::c_int &
             0xff as libc::c_int) as u8_0;
    (*font).numDrums =
        ((*entry).shortData2 as libc::c_int & 0xff as libc::c_int) as u8_0;
    (*font).numSfx = (*entry).shortData3 as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_Init(mut heap: *mut libc::c_void,
                                        mut heapSize: u32_0) {
    let mut pad: [libc::c_char; 72] = [0; 72];
    let mut numFonts: s32 = 0;
    let mut temp_v0_3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: s32 = 0;
    let mut heapP: *mut u64_0 = 0 as *mut u64_0;
    let mut ctxP: *mut u8_0 = 0 as *mut u8_0;
    let mut u2974p: *mut s16 = 0 as *mut s16;
    D_801755D0 = None;
    ::std::ptr::write_volatile(&mut gAudioContext.resetTimer as *mut u32_0,
                               0 as libc::c_int as u32_0);
    let mut i_0: s32 = 0;
    let mut ctxP_0: *mut u8_0 =
        &mut gAudioContext as *mut AudioContext as *mut u8_0;
    i_0 = ::std::mem::size_of::<AudioContext>() as libc::c_ulong as s32;
    while i_0 >= 0 as libc::c_int {
        let fresh18 = ctxP_0;
        ctxP_0 = ctxP_0.offset(1);
        *fresh18 = 0 as libc::c_int as u8_0;
        i_0 -= 1
    }
    match osTvType {
        0 => {
            gAudioContext.unk_2960 = 20.03042f32;
            gAudioContext.refreshRate = 50 as libc::c_int
        }
        2 => {
            gAudioContext.unk_2960 = 16.546f32;
            gAudioContext.refreshRate = 60 as libc::c_int
        }
        1 | _ => {
            gAudioContext.unk_2960 = 16.713f32;
            gAudioContext.refreshRate = 60 as libc::c_int
        }
    }
    Audio_InitMesgQueues();
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        gAudioContext.aiBufLengths[i as usize] = 0xa0 as libc::c_int as s16;
        i += 1
    }
    gAudioContext.totalTaskCnt = 0 as libc::c_int;
    gAudioContext.rspTaskIdx = 0 as libc::c_int;
    gAudioContext.curAIBufIdx = 0 as libc::c_int;
    gAudioContext.soundMode = 0 as libc::c_int as s8;
    gAudioContext.currTask = 0 as *mut AudioTask;
    gAudioContext.rspTask[0 as libc::c_int as usize].task.t.data_size =
        0 as libc::c_int as u32_0;
    gAudioContext.rspTask[1 as libc::c_int as usize].task.t.data_size =
        0 as libc::c_int as u32_0;
    osCreateMesgQueue(&mut gAudioContext.syncDmaQueue,
                      &mut gAudioContext.syncDmaMesg, 1 as libc::c_int);
    osCreateMesgQueue(&mut gAudioContext.currAudioFrameDmaQueue,
                      gAudioContext.currAudioFrameDmaMesgBuf.as_mut_ptr(),
                      0x40 as libc::c_int);
    osCreateMesgQueue(&mut gAudioContext.externalLoadQueue,
                      gAudioContext.externalLoadMesgBuf.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 16]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
    osCreateMesgQueue(&mut gAudioContext.preloadSampleQueue,
                      gAudioContext.preloadSampleMesgBuf.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 16]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
    gAudioContext.curAudioFrameDmaCount = 0 as libc::c_int;
    gAudioContext.sampleDmaCount = 0 as libc::c_int as u32_0;
    gAudioContext.cartHandle = osCartRomInit();
    if heap.is_null() {
        gAudioContext.audioHeap = gAudioHeap.as_mut_ptr();
        gAudioContext.audioHeapSize = D_8014A6C4.heapSize
    } else {
        let mut hp: *mut *mut libc::c_void = &mut heap;
        gAudioContext.audioHeap = *hp as *mut u8_0;
        gAudioContext.audioHeapSize = heapSize
    }
    i = 0 as libc::c_int;
    while i < gAudioContext.audioHeapSize as s32 / 8 as libc::c_int {
        *(gAudioContext.audioHeap as *mut u64_0).offset(i as isize) =
            0 as libc::c_int as u64_0;
        i += 1
    }
    AudioHeap_InitMainPools(D_8014A6C4.initPoolSize as s32);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        gAudioContext.aiBuffers[i as usize] =
            AudioHeap_AllocZeroed(&mut gAudioContext.audioInitPool,
                                  (0x580 as libc::c_int as
                                       libc::c_uint).wrapping_mul(::std::mem::size_of::<s16>()
                                                                      as
                                                                      libc::c_ulong))
                as *mut s16;
        i += 1
    }
    gAudioContext.sequenceTable =
        gSequenceTable.as_mut_ptr() as *mut AudioTable;
    gAudioContext.soundFontTable =
        gSoundFontTable.as_mut_ptr() as *mut AudioTable;
    gAudioContext.sampleBankTable =
        gSampleBankTable.as_mut_ptr() as *mut AudioTable;
    gAudioContext.sequenceFontTable = gSequenceFontTable.as_mut_ptr();
    gAudioContext.numSequences =
        (*gAudioContext.sequenceTable).numEntries as u16_0;
    gAudioContext.audioResetSpecIdToLoad = 0 as libc::c_int as u8_0;
    ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as *mut u8_0,
                               1 as libc::c_int as u8_0);
    AudioHeap_ResetStep();
    AudioLoad_InitTable(gAudioContext.sequenceTable,
                        _AudioseqSegmentRomStart.as_mut_ptr() as u32_0,
                        0 as libc::c_int as u16_0);
    AudioLoad_InitTable(gAudioContext.soundFontTable,
                        _AudiobankSegmentRomStart.as_mut_ptr() as u32_0,
                        0 as libc::c_int as u16_0);
    AudioLoad_InitTable(gAudioContext.sampleBankTable,
                        _AudiotableSegmentRomStart.as_mut_ptr() as u32_0,
                        0 as libc::c_int as u16_0);
    numFonts = (*gAudioContext.soundFontTable).numEntries as s32;
    gAudioContext.soundFonts =
        AudioHeap_Alloc(&mut gAudioContext.audioInitPool,
                        (numFonts as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<SoundFont>()
                                                            as libc::c_ulong))
            as *mut SoundFont;
    i = 0 as libc::c_int;
    while i < numFonts { AudioLoad_InitSoundFontMeta(i); i += 1 }
    temp_v0_3 =
        AudioHeap_Alloc(&mut gAudioContext.audioInitPool,
                        D_8014A6C4.permanentPoolSize);
    if temp_v0_3 == 0 as *mut libc::c_void {
        // cast away const from D_8014A6C4
        *(&D_8014A6C4.permanentPoolSize as *const u32_0 as *mut u32_0) =
            0 as libc::c_int as u32_0
    }
    AudioHeap_AllocPoolInit(&mut gAudioContext.permanentPool, temp_v0_3,
                            D_8014A6C4.permanentPoolSize);
    gAudioContextInitalized = 1 as libc::c_int;
    osSendMesg(gAudioContext.taskStartQueueP,
               gAudioContext.totalTaskCnt as *mut libc::c_void,
               0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_InitSlowLoads() {
    gAudioContext.slowLoads[0 as libc::c_int as usize].status =
        0 as libc::c_int;
    gAudioContext.slowLoads[1 as libc::c_int as usize].status =
        0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SlowLoadSample(mut fontId: s32,
                                                  mut instId: s32,
                                                  mut isDone: *mut s8)
 -> s32 {
    let mut sample: *mut SoundFontSample = 0 as *mut SoundFontSample;
    let mut slowLoad: *mut AudioSlowLoad = 0 as *mut AudioSlowLoad;
    sample = AudioLoad_GetFontSample(fontId, instId);
    if sample.is_null() {
        *isDone = 0 as libc::c_int as s8;
        return -(1 as libc::c_int)
    }
    if (*sample).medium() as libc::c_int == MEDIUM_RAM as libc::c_int {
        *isDone = 2 as libc::c_int as s8;
        return 0 as libc::c_int
    }
    slowLoad =
        &mut *gAudioContext.slowLoads.as_mut_ptr().offset(gAudioContext.slowLoadPos
                                                              as isize) as
            *mut AudioSlowLoad;
    if (*slowLoad).status == LOAD_STATUS_DONE as libc::c_int {
        (*slowLoad).status = LOAD_STATUS_WAITING as libc::c_int
    }
    (*slowLoad).sample = *sample;
    (*slowLoad).isDone = isDone;
    (*slowLoad).curRamAddr =
        AudioHeap_AllocSampleCache((*sample).size(), fontId,
                                   (*sample).sampleAddr as *mut libc::c_void,
                                   (*sample).medium() as s8,
                                   CACHE_TEMPORARY as libc::c_int) as
            *mut u8_0;
    if (*slowLoad).curRamAddr.is_null() {
        if (*sample).medium() as libc::c_int == MEDIUM_UNK as libc::c_int ||
               (*sample).codec() as libc::c_int ==
                   CODEC_S16_INMEMORY as libc::c_int {
            *isDone = 0 as libc::c_int as s8;
            return -(1 as libc::c_int)
        } else {
            *isDone = 3 as libc::c_int as s8;
            return -(1 as libc::c_int)
        }
    }
    (*slowLoad).status = LOAD_STATUS_START as libc::c_int;
    (*slowLoad).bytesRemaining =
        (*sample).size() as libc::c_int + 0xf as libc::c_int &
            !(0xf as libc::c_int);
    (*slowLoad).ramAddr = (*slowLoad).curRamAddr;
    (*slowLoad).curDevAddr = (*sample).sampleAddr as s32;
    (*slowLoad).medium = (*sample).medium() as u8_0;
    (*slowLoad).seqOrFontId = fontId as u8_0;
    (*slowLoad).instId = instId as u16_0;
    if (*slowLoad).medium as libc::c_int == MEDIUM_UNK as libc::c_int {
        (*slowLoad).unkMediumParam =
            (*gAudioContext.sampleBankTable).unkMediumParam as s32
    }
    gAudioContext.slowLoadPos ^= 1 as libc::c_int as libc::c_uint;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_GetFontSample(mut fontId: s32,
                                                 mut instId: s32)
 -> *mut SoundFontSample {
    let mut ret: *mut SoundFontSample = 0 as *mut SoundFontSample;
    if instId < 0x80 as libc::c_int {
        let mut instrument: *mut Instrument =
            Audio_GetInstrumentInner(fontId, instId);
        if instrument.is_null() { return 0 as *mut SoundFontSample }
        ret = (*instrument).normalNotesSound.sample
    } else if instId < 0x100 as libc::c_int {
        let mut drum: *mut Drum =
            Audio_GetDrum(fontId, instId - 0x80 as libc::c_int);
        if drum.is_null() { return 0 as *mut SoundFontSample }
        ret = (*drum).sound.sample
    } else {
        let mut sound: *mut SoundFontSound =
            Audio_GetSfx(fontId, instId - 0x100 as libc::c_int);
        if sound.is_null() { return 0 as *mut SoundFontSample }
        ret = (*sound).sample
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_Unused2() { }
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_FinishSlowLoad(mut slowLoad:
                                                      *mut AudioSlowLoad) {
    let mut sample: *mut SoundFontSample = 0 as *mut SoundFontSample;
    if (*slowLoad).sample.sampleAddr.is_null() { return }
    sample =
        AudioLoad_GetFontSample((*slowLoad).seqOrFontId as s32,
                                (*slowLoad).instId as s32);
    if sample.is_null() { return }
    (*slowLoad).sample = *sample;
    (*sample).sampleAddr = (*slowLoad).ramAddr;
    (*sample).set_medium(MEDIUM_RAM as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ProcessSlowLoads(mut resetStatus: s32) {
    let mut slowLoad: *mut AudioSlowLoad = 0 as *mut AudioSlowLoad;
    let mut i: s32 = 0;
    let mut current_block_28: u64;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[AudioSlowLoad; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<AudioSlowLoad>()
                                                   as libc::c_ulong) as s32 {
        slowLoad =
            &mut *gAudioContext.slowLoads.as_mut_ptr().offset(i as isize) as
                *mut AudioSlowLoad;
        match gAudioContext.slowLoads[i as usize].status {
            2 => {
                if (*slowLoad).medium as libc::c_int !=
                       MEDIUM_UNK as libc::c_int {
                    osRecvMesg(&mut (*slowLoad).msgqueue, 0 as *mut OSMesg,
                               1 as libc::c_int);
                }
                if resetStatus != 0 as libc::c_int {
                    (*slowLoad).status = LOAD_STATUS_DONE as libc::c_int;
                    current_block_28 = 16559507199688588974;
                } else { current_block_28 = 169503734211517357; }
            }
            1 => { current_block_28 = 169503734211517357; }
            _ => { current_block_28 = 16559507199688588974; }
        }
        match current_block_28 {
            169503734211517357 => {
                (*slowLoad).status = LOAD_STATUS_LOADING as libc::c_int;
                if (*slowLoad).bytesRemaining == 0 as libc::c_int {
                    AudioLoad_FinishSlowLoad(slowLoad);
                    (*slowLoad).status = LOAD_STATUS_DONE as libc::c_int;
                    *(*slowLoad).isDone = 1 as libc::c_int as s8
                } else if (*slowLoad).bytesRemaining < 0x400 as libc::c_int {
                    if (*slowLoad).medium as libc::c_int ==
                           MEDIUM_UNK as libc::c_int {
                        let mut size: u32_0 =
                            (*slowLoad).bytesRemaining as u32_0;
                        AudioLoad_DmaSlowCopyUnkMedium((*slowLoad).curDevAddr,
                                                       (*slowLoad).curRamAddr,
                                                       size as s32,
                                                       (*slowLoad).unkMediumParam);
                    } else {
                        AudioLoad_DmaSlowCopy(slowLoad,
                                              (*slowLoad).bytesRemaining);
                    }
                    (*slowLoad).bytesRemaining = 0 as libc::c_int
                } else {
                    if (*slowLoad).medium as libc::c_int ==
                           MEDIUM_UNK as libc::c_int {
                        AudioLoad_DmaSlowCopyUnkMedium((*slowLoad).curDevAddr,
                                                       (*slowLoad).curRamAddr,
                                                       0x400 as libc::c_int,
                                                       (*slowLoad).unkMediumParam);
                    } else {
                        AudioLoad_DmaSlowCopy(slowLoad, 0x400 as libc::c_int);
                    }
                    (*slowLoad).bytesRemaining -= 0x400 as libc::c_int;
                    (*slowLoad).curRamAddr =
                        (*slowLoad).curRamAddr.offset(0x400 as libc::c_int as
                                                          isize);
                    (*slowLoad).curDevAddr += 0x400 as libc::c_int
                }
            }
            _ => { }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_DmaSlowCopy(mut slowLoad:
                                                   *mut AudioSlowLoad,
                                               mut size: s32) {
    Audio_InvalDCache((*slowLoad).curRamAddr as *mut libc::c_void, size);
    osCreateMesgQueue(&mut (*slowLoad).msgqueue, &mut (*slowLoad).msg,
                      1 as libc::c_int);
    AudioLoad_Dma(&mut (*slowLoad).ioMesg, 0 as libc::c_int as u32_0,
                  0 as libc::c_int, (*slowLoad).curDevAddr as u32_0,
                  (*slowLoad).curRamAddr as *mut libc::c_void, size as u32_0,
                  &mut (*slowLoad).msgqueue, (*slowLoad).medium as s32,
                  b"SLOWCOPY\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_DmaSlowCopyUnkMedium(mut devAddr: s32,
                                                        mut ramAddr:
                                                            *mut u8_0,
                                                        mut size: s32,
                                                        mut arg3: s32) {
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_SlowLoadSeq(mut seqId: s32,
                                               mut ramAddr: *mut u8_0,
                                               mut isDone: *mut s8) -> s32 {
    let mut slowLoad: *mut AudioSlowLoad = 0 as *mut AudioSlowLoad;
    let mut seqTable: *mut AudioTable = 0 as *mut AudioTable;
    let mut size: u32_0 = 0;
    if seqId >= gAudioContext.numSequences as libc::c_int {
        *isDone = 0 as libc::c_int as s8;
        return -(1 as libc::c_int)
    }
    seqId =
        AudioLoad_GetRealTableIndex(SEQUENCE_TABLE as libc::c_int,
                                    seqId as u32_0) as s32;
    seqTable = AudioLoad_GetLoadTable(SEQUENCE_TABLE as libc::c_int);
    slowLoad =
        &mut *gAudioContext.slowLoads.as_mut_ptr().offset(gAudioContext.slowLoadPos
                                                              as isize) as
            *mut AudioSlowLoad;
    if (*slowLoad).status == LOAD_STATUS_DONE as libc::c_int {
        (*slowLoad).status = LOAD_STATUS_WAITING as libc::c_int
    }
    (*slowLoad).sample.sampleAddr = 0 as *mut u8_0;
    (*slowLoad).isDone = isDone;
    size = (*(*seqTable).entries.as_mut_ptr().offset(seqId as isize)).size;
    size =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    (*slowLoad).curRamAddr = ramAddr;
    (*slowLoad).status = LOAD_STATUS_START as libc::c_int;
    (*slowLoad).bytesRemaining = size as s32;
    (*slowLoad).ramAddr = ramAddr;
    (*slowLoad).curDevAddr =
        (*(*seqTable).entries.as_mut_ptr().offset(seqId as isize)).romAddr as
            s32;
    (*slowLoad).medium =
        (*(*seqTable).entries.as_mut_ptr().offset(seqId as isize)).medium as
            u8_0;
    (*slowLoad).seqOrFontId = seqId as u8_0;
    if (*slowLoad).medium as libc::c_int == MEDIUM_UNK as libc::c_int {
        (*slowLoad).unkMediumParam = (*seqTable).unkMediumParam as s32
    }
    gAudioContext.slowLoadPos ^= 1 as libc::c_int as libc::c_uint;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_InitAsyncLoads() {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[AudioAsyncLoad; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<AudioAsyncLoad>()
                                                   as libc::c_ulong) as s32 {
        gAudioContext.asyncLoads[i as usize].status = 0 as libc::c_int as s8;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_StartAsyncLoadUnkMedium(mut unkMediumParam:
                                                               s32,
                                                           mut devAddr: u32_0,
                                                           mut ramAddr:
                                                               *mut libc::c_void,
                                                           mut size: s32,
                                                           mut medium: s32,
                                                           mut nChunks: s32,
                                                           mut retQueue:
                                                               *mut OSMesgQueue,
                                                           mut retMsg: s32)
 -> *mut AudioAsyncLoad {
    let mut asyncLoad: *mut AudioAsyncLoad = 0 as *mut AudioAsyncLoad;
    asyncLoad =
        AudioLoad_StartAsyncLoad(devAddr, ramAddr, size as u32_0, medium,
                                 nChunks, retQueue, retMsg);
    if asyncLoad.is_null() { return 0 as *mut AudioAsyncLoad }
    osSendMesg(&mut gAudioContext.asyncLoadUnkMediumQueue,
               asyncLoad as OSMesg, 0 as libc::c_int);
    (*asyncLoad).unkMediumParam = unkMediumParam;
    return asyncLoad;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_StartAsyncLoad(mut devAddr: u32_0,
                                                  mut ramAddr:
                                                      *mut libc::c_void,
                                                  mut size: u32_0,
                                                  mut medium: s32,
                                                  mut nChunks: s32,
                                                  mut retQueue:
                                                      *mut OSMesgQueue,
                                                  mut retMsg: s32)
 -> *mut AudioAsyncLoad {
    let mut asyncLoad: *mut AudioAsyncLoad = 0 as *mut AudioAsyncLoad;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[AudioAsyncLoad; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<AudioAsyncLoad>()
                                                   as libc::c_ulong) as s32 {
        if gAudioContext.asyncLoads[i as usize].status as libc::c_int ==
               0 as libc::c_int {
            asyncLoad =
                &mut *gAudioContext.asyncLoads.as_mut_ptr().offset(i as isize)
                    as *mut AudioAsyncLoad;
            break ;
        } else { i += 1 }
    }
    // no more available async loads
    if i ==
           (::std::mem::size_of::<[AudioAsyncLoad; 16]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<AudioAsyncLoad>()
                                                as libc::c_ulong) as s32 {
        return 0 as *mut AudioAsyncLoad
    }
    (*asyncLoad).status = LOAD_STATUS_START as libc::c_int as s8;
    (*asyncLoad).curDevAddr = devAddr;
    (*asyncLoad).ramAddr = ramAddr as *mut u8_0;
    (*asyncLoad).curRamAddr = ramAddr as *mut u8_0;
    (*asyncLoad).bytesRemaining = size;
    if nChunks == 0 as libc::c_int {
        (*asyncLoad).chunkSize = 0x1000 as libc::c_int as u32_0
    } else if nChunks == 1 as libc::c_int {
        (*asyncLoad).chunkSize = size
    } else {
        (*asyncLoad).chunkSize =
            (size as s32 / nChunks + 0xff as libc::c_int &
                 !(0xff as libc::c_int)) as u32_0;
        if (*asyncLoad).chunkSize < 0x100 as libc::c_int as libc::c_uint {
            (*asyncLoad).chunkSize = 0x100 as libc::c_int as u32_0
        }
    }
    (*asyncLoad).retQueue = retQueue;
    (*asyncLoad).delay = 3 as libc::c_int as s8;
    (*asyncLoad).medium = medium as s8;
    (*asyncLoad).retMsg = retMsg as u32_0;
    osCreateMesgQueue(&mut (*asyncLoad).msgQueue, &mut (*asyncLoad).msg,
                      1 as libc::c_int);
    return asyncLoad;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ProcessAsyncLoads(mut resetStatus: s32) {
    let mut asyncLoad: *mut AudioAsyncLoad = 0 as *mut AudioAsyncLoad;
    let mut i: s32 = 0;
    if gAudioContext.resetTimer == 1 as libc::c_int as libc::c_uint { return }
    if gAudioContext.curUnkMediumLoad.is_null() {
        if resetStatus != 0 as libc::c_int {
            // Clear and ignore queue if resetting.
            while osRecvMesg(&mut gAudioContext.asyncLoadUnkMediumQueue,
                             &mut asyncLoad as *mut *mut AudioAsyncLoad as
                                 *mut OSMesg, 0 as libc::c_int) !=
                      -(1 as libc::c_int) {
            }
        } else if osRecvMesg(&mut gAudioContext.asyncLoadUnkMediumQueue,
                             &mut asyncLoad as *mut *mut AudioAsyncLoad as
                                 *mut OSMesg, 0 as libc::c_int) ==
                      -(1 as libc::c_int) {
            gAudioContext.curUnkMediumLoad = 0 as *mut AudioAsyncLoad
        } else { gAudioContext.curUnkMediumLoad = asyncLoad }
    }
    if !gAudioContext.curUnkMediumLoad.is_null() {
        AudioLoad_ProcessAsyncLoadUnkMedium(gAudioContext.curUnkMediumLoad,
                                            resetStatus);
    }
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[AudioAsyncLoad; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<AudioAsyncLoad>()
                                                   as libc::c_ulong) as s32 {
        if gAudioContext.asyncLoads[i as usize].status as libc::c_int ==
               1 as libc::c_int {
            asyncLoad =
                &mut *gAudioContext.asyncLoads.as_mut_ptr().offset(i as isize)
                    as *mut AudioAsyncLoad;
            if (*asyncLoad).medium as libc::c_int != MEDIUM_UNK as libc::c_int
               {
                AudioLoad_ProcessAsyncLoad(asyncLoad, resetStatus);
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ProcessAsyncLoadUnkMedium(mut asyncLoad:
                                                                 *mut AudioAsyncLoad,
                                                             mut resetStatus:
                                                                 s32) {
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_FinishAsyncLoad(mut asyncLoad:
                                                       *mut AudioAsyncLoad) {
    let mut retMsg: u32_0 = (*asyncLoad).retMsg;
    let mut fontId: u32_0 = 0;
    let mut pad: u32_0 = 0;
    let mut doneMsg: OSMesg = 0 as *mut libc::c_void;
    let mut sampleBankId1: u32_0 = 0;
    let mut sampleBankId2: u32_0 = 0;
    let mut relocInfo: RelocInfo =
        RelocInfo{sampleBankId1: 0,
                  sampleBankId2: 0,
                  baseAddr1: 0,
                  baseAddr2: 0,
                  medium1: 0,
                  medium2: 0,};
    match (retMsg >> 16 as libc::c_int) as u8_0 as libc::c_int {
        0 => {
            AudioLoad_SetSeqLoadStatus((retMsg >> 8 as libc::c_int) as u8_0 as
                                           s32,
                                       (retMsg >> 0 as libc::c_int) as u8_0 as
                                           s32);
        }
        2 => {
            AudioLoad_SetSampleFontLoadStatusAndApplyCaches((retMsg >>
                                                                 8 as
                                                                     libc::c_int)
                                                                as u8_0 as
                                                                s32,
                                                            (retMsg >>
                                                                 0 as
                                                                     libc::c_int)
                                                                as u8_0 as
                                                                s32);
        }
        1 => {
            fontId = (retMsg >> 8 as libc::c_int) as u8_0 as u32_0;
            sampleBankId1 =
                (*gAudioContext.soundFonts.offset(fontId as
                                                      isize)).sampleBankId1 as
                    u32_0;
            sampleBankId2 =
                (*gAudioContext.soundFonts.offset(fontId as
                                                      isize)).sampleBankId2 as
                    u32_0;
            relocInfo.sampleBankId1 = sampleBankId1 as s32;
            relocInfo.sampleBankId2 = sampleBankId2 as s32;
            relocInfo.baseAddr1 =
                if sampleBankId1 != 0xff as libc::c_int as libc::c_uint {
                    AudioLoad_GetSampleBank(sampleBankId1,
                                            &mut relocInfo.medium1)
                } else { 0 as libc::c_int as libc::c_uint } as s32;
            relocInfo.baseAddr2 =
                if sampleBankId2 != 0xff as libc::c_int as libc::c_uint {
                    AudioLoad_GetSampleBank(sampleBankId2,
                                            &mut relocInfo.medium2)
                } else { 0 as libc::c_int as libc::c_uint } as s32;
            AudioLoad_SetFontLoadStatus(fontId as s32,
                                        (retMsg >> 0 as libc::c_int) as u8_0
                                            as s32);
            AudioLoad_RelocateFontAndPreloadSamples(fontId as s32,
                                                    (*asyncLoad).ramAddr as
                                                        *mut libc::c_void,
                                                    &mut relocInfo,
                                                    1 as libc::c_int);
        }
        _ => { }
    }
    doneMsg = (*asyncLoad).retMsg as OSMesg;
    (*asyncLoad).status = LOAD_STATUS_WAITING as libc::c_int as s8;
    osSendMesg((*asyncLoad).retQueue, doneMsg, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ProcessAsyncLoad(mut asyncLoad:
                                                        *mut AudioAsyncLoad,
                                                    mut resetStatus: s32) {
    let mut sampleBankTable: *mut AudioTable = gAudioContext.sampleBankTable;
    if (*asyncLoad).delay as libc::c_int >= 2 as libc::c_int {
        (*asyncLoad).delay -= 1;
        return
    }
    if (*asyncLoad).delay as libc::c_int == 1 as libc::c_int {
        (*asyncLoad).delay = 0 as libc::c_int as s8
    } else if resetStatus != 0 as libc::c_int {
        // Await the previous DMA response synchronously, then return.
        osRecvMesg(&mut (*asyncLoad).msgQueue, 0 as *mut OSMesg,
                   1 as libc::c_int);
        (*asyncLoad).status = LOAD_STATUS_WAITING as libc::c_int as s8;
        return
    } else {
        if osRecvMesg(&mut (*asyncLoad).msgQueue, 0 as *mut OSMesg,
                      0 as libc::c_int) == -(1 as libc::c_int) {
            // If the previous DMA step isn't done, return.
            return
        }
    }
    if (*asyncLoad).bytesRemaining == 0 as libc::c_int as libc::c_uint {
        AudioLoad_FinishAsyncLoad(asyncLoad);
        return
    }
    if (*asyncLoad).bytesRemaining < (*asyncLoad).chunkSize {
        if (*asyncLoad).medium as libc::c_int == MEDIUM_UNK as libc::c_int {
            AudioLoad_AsyncDmaUnkMedium((*asyncLoad).curDevAddr,
                                        (*asyncLoad).curRamAddr as
                                            *mut libc::c_void,
                                        (*asyncLoad).bytesRemaining,
                                        (*sampleBankTable).unkMediumParam);
        } else { AudioLoad_AsyncDma(asyncLoad, (*asyncLoad).bytesRemaining); }
        (*asyncLoad).bytesRemaining = 0 as libc::c_int as u32_0;
        return
    }
    if (*asyncLoad).medium as libc::c_int == MEDIUM_UNK as libc::c_int {
        AudioLoad_AsyncDmaUnkMedium((*asyncLoad).curDevAddr,
                                    (*asyncLoad).curRamAddr as
                                        *mut libc::c_void,
                                    (*asyncLoad).chunkSize,
                                    (*sampleBankTable).unkMediumParam);
    } else { AudioLoad_AsyncDma(asyncLoad, (*asyncLoad).chunkSize); }
    (*asyncLoad).bytesRemaining =
        ((*asyncLoad).bytesRemaining as
             libc::c_uint).wrapping_sub((*asyncLoad).chunkSize) as u32_0 as
            u32_0;
    (*asyncLoad).curDevAddr =
        ((*asyncLoad).curDevAddr as
             libc::c_uint).wrapping_add((*asyncLoad).chunkSize) as u32_0 as
            u32_0;
    (*asyncLoad).curRamAddr =
        (*asyncLoad).curRamAddr.offset((*asyncLoad).chunkSize as isize);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AsyncDma(mut asyncLoad:
                                                *mut AudioAsyncLoad,
                                            mut size: u32_0) {
    size =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    Audio_InvalDCache((*asyncLoad).curRamAddr as *mut libc::c_void,
                      size as s32);
    osCreateMesgQueue(&mut (*asyncLoad).msgQueue, &mut (*asyncLoad).msg,
                      1 as libc::c_int);
    AudioLoad_Dma(&mut (*asyncLoad).ioMesg, 0 as libc::c_int as u32_0,
                  0 as libc::c_int, (*asyncLoad).curDevAddr,
                  (*asyncLoad).curRamAddr as *mut libc::c_void, size,
                  &mut (*asyncLoad).msgQueue, (*asyncLoad).medium as s32,
                  b"BGCOPY\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AsyncDmaUnkMedium(mut devAddr: u32_0,
                                                     mut ramAddr:
                                                         *mut libc::c_void,
                                                     mut size: u32_0,
                                                     mut arg3: s16) {
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_RelocateSample(mut sound:
                                                      *mut SoundFontSound,
                                                  mut mem: *mut libc::c_void,
                                                  mut relocInfo:
                                                      *mut RelocInfo) {
    let mut sample: *mut SoundFontSample = 0 as *mut SoundFontSample;
    let mut reloc: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*sound).sample as u32_0 <= 0x80000000 as libc::c_uint {
        reloc =
            ((*sound).sample as u32_0).wrapping_add(mem as u32_0) as
                *mut libc::c_void;
        (*sound).sample = reloc as *mut SoundFontSample;
        sample = (*sound).sample;
        if (*sample).size() as libc::c_int != 0 as libc::c_int &&
               (*sample).unk_bit25() as libc::c_int != 1 as libc::c_int {
            reloc =
                ((*sample).loop_0 as u32_0).wrapping_add(mem as u32_0) as
                    *mut libc::c_void;
            (*sample).loop_0 = reloc as *mut AdpcmLoop;
            reloc =
                ((*sample).book as u32_0).wrapping_add(mem as u32_0) as
                    *mut libc::c_void;
            (*sample).book = reloc as *mut AdpcmBook;
            // Resolve the sample medium 2-bit bitfield into a real value based on relocInfo.
            match (*sample).medium() as libc::c_int {
                0 => {
                    reloc =
                        ((*sample).sampleAddr as
                             u32_0).wrapping_add((*relocInfo).baseAddr1 as
                                                     u32_0) as
                            *mut libc::c_void;
                    (*sample).sampleAddr = reloc as *mut u8_0;
                    (*sample).set_medium((*relocInfo).medium1)
                }
                1 => {
                    reloc =
                        ((*sample).sampleAddr as
                             u32_0).wrapping_add((*relocInfo).baseAddr2 as
                                                     u32_0) as
                            *mut libc::c_void;
                    (*sample).sampleAddr = reloc as *mut u8_0;
                    (*sample).set_medium((*relocInfo).medium2)
                }
                2 | 3 | _ => { }
            }
            (*sample).set_unk_bit25(1 as libc::c_int as u32_0);
            if (*sample).unk_bit26() as libc::c_int != 0 &&
                   (*sample).medium() as libc::c_int !=
                       MEDIUM_RAM as libc::c_int {
                let fresh19 = gAudioContext.numUsedSamples;
                gAudioContext.numUsedSamples =
                    gAudioContext.numUsedSamples + 1;
                gAudioContext.usedSamples[fresh19 as usize] = sample
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_RelocateFontAndPreloadSamples(mut fontId:
                                                                     s32,
                                                                 mut mem:
                                                                     *mut libc::c_void,
                                                                 mut relocInfo:
                                                                     *mut RelocInfo,
                                                                 mut async_0:
                                                                     s32) {
    let mut preload: *mut AudioPreloadReq = 0 as *mut AudioPreloadReq;
    let mut topPreload: *mut AudioPreloadReq = 0 as *mut AudioPreloadReq;
    let mut sample: *mut SoundFontSample = 0 as *mut SoundFontSample;
    let mut size: s32 = 0;
    let mut nChunks: s32 = 0;
    let mut addr: *mut u8_0 = 0 as *mut u8_0;
    let mut preloadInProgress: s32 = 0;
    let mut i: s32 = 0;
    preloadInProgress = 0 as libc::c_int;
    if gAudioContext.preloadSampleStackTop != 0 as libc::c_int {
        preloadInProgress = 1 as libc::c_int
    } else { D_8016B780 = 0 as libc::c_int }
    gAudioContext.numUsedSamples = 0 as libc::c_int;
    AudioLoad_RelocateFont(fontId, mem, relocInfo);
    size = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < gAudioContext.numUsedSamples {
        size +=
            (*gAudioContext.usedSamples[i as usize]).size() as libc::c_int +
                0xf as libc::c_int & !(0xf as libc::c_int);
        i += 1
    }
    (size != 0) && size != 0;
    i = 0 as libc::c_int;
    while i < gAudioContext.numUsedSamples {
        if gAudioContext.preloadSampleStackTop == 120 as libc::c_int {
            break ;
        }
        sample = gAudioContext.usedSamples[i as usize];
        addr = 0 as *mut u8_0;
        match async_0 {
            0 => {
                if (*sample).medium() == (*relocInfo).medium1 {
                    addr =
                        AudioHeap_AllocSampleCache((*sample).size(),
                                                   (*relocInfo).sampleBankId1,
                                                   (*sample).sampleAddr as
                                                       *mut libc::c_void,
                                                   (*sample).medium() as s8,
                                                   CACHE_PERSISTENT as
                                                       libc::c_int) as
                            *mut u8_0
                } else if (*sample).medium() == (*relocInfo).medium2 {
                    addr =
                        AudioHeap_AllocSampleCache((*sample).size(),
                                                   (*relocInfo).sampleBankId2,
                                                   (*sample).sampleAddr as
                                                       *mut libc::c_void,
                                                   (*sample).medium() as s8,
                                                   CACHE_PERSISTENT as
                                                       libc::c_int) as
                            *mut u8_0
                } else if (*sample).medium() as libc::c_int ==
                              MEDIUM_DISK_DRIVE as libc::c_int {
                    addr =
                        AudioHeap_AllocSampleCache((*sample).size(),
                                                   0xfe as libc::c_int,
                                                   (*sample).sampleAddr as
                                                       *mut libc::c_void,
                                                   (*sample).medium() as s8,
                                                   CACHE_PERSISTENT as
                                                       libc::c_int) as
                            *mut u8_0
                }
            }
            1 => {
                if (*sample).medium() == (*relocInfo).medium1 {
                    addr =
                        AudioHeap_AllocSampleCache((*sample).size(),
                                                   (*relocInfo).sampleBankId1,
                                                   (*sample).sampleAddr as
                                                       *mut libc::c_void,
                                                   (*sample).medium() as s8,
                                                   CACHE_TEMPORARY as
                                                       libc::c_int) as
                            *mut u8_0
                } else if (*sample).medium() == (*relocInfo).medium2 {
                    addr =
                        AudioHeap_AllocSampleCache((*sample).size(),
                                                   (*relocInfo).sampleBankId2,
                                                   (*sample).sampleAddr as
                                                       *mut libc::c_void,
                                                   (*sample).medium() as s8,
                                                   CACHE_TEMPORARY as
                                                       libc::c_int) as
                            *mut u8_0
                } else if (*sample).medium() as libc::c_int ==
                              MEDIUM_DISK_DRIVE as libc::c_int {
                    addr =
                        AudioHeap_AllocSampleCache((*sample).size(),
                                                   0xfe as libc::c_int,
                                                   (*sample).sampleAddr as
                                                       *mut libc::c_void,
                                                   (*sample).medium() as s8,
                                                   CACHE_TEMPORARY as
                                                       libc::c_int) as
                            *mut u8_0
                }
            }
            _ => { }
        }
        if !addr.is_null() {
            match async_0 {
                0 => {
                    if (*sample).medium() as libc::c_int ==
                           MEDIUM_UNK as libc::c_int {
                        AudioLoad_SyncDmaUnkMedium((*sample).sampleAddr as
                                                       u32_0, addr,
                                                   (*sample).size(),
                                                   (*gAudioContext.sampleBankTable).unkMediumParam
                                                       as s32);
                        (*sample).sampleAddr = addr;
                        (*sample).set_medium(MEDIUM_RAM as libc::c_int as
                                                 u32_0)
                    } else {
                        AudioLoad_SyncDma((*sample).sampleAddr as u32_0, addr,
                                          (*sample).size(),
                                          (*sample).medium() as s32);
                        (*sample).sampleAddr = addr;
                        (*sample).set_medium(MEDIUM_RAM as libc::c_int as
                                                 u32_0)
                    }
                    ((*sample).medium() as libc::c_int) ==
                        MEDIUM_DISK_DRIVE as libc::c_int;
                }
                1 => {
                    preload =
                        &mut *gAudioContext.preloadSampleStack.as_mut_ptr().offset(gAudioContext.preloadSampleStackTop
                                                                                       as
                                                                                       isize)
                            as *mut AudioPreloadReq;
                    (*preload).sample = sample;
                    (*preload).ramAddr = addr;
                    (*preload).encodedInfo =
                        (gAudioContext.preloadSampleStackTop <<
                             24 as libc::c_int | 0xffffff as libc::c_int) as
                            u32_0;
                    (*preload).isFree = 0 as libc::c_int;
                    (*preload).endAndMediumKey =
                        ((*sample).sampleAddr as
                             u32_0).wrapping_add((*sample).size()).wrapping_add((*sample).medium());
                    gAudioContext.preloadSampleStackTop += 1
                }
                _ => { }
            }
        }
        i += 1
    }
    gAudioContext.numUsedSamples = 0 as libc::c_int;
    if gAudioContext.preloadSampleStackTop != 0 as libc::c_int &&
           preloadInProgress == 0 {
        topPreload =
            &mut *gAudioContext.preloadSampleStack.as_mut_ptr().offset((gAudioContext.preloadSampleStackTop
                                                                            -
                                                                            1
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           isize)
                as *mut AudioPreloadReq;
        sample = (*topPreload).sample;
        nChunks =
            ((*sample).size() as libc::c_int >> 12 as libc::c_int) +
                1 as libc::c_int;
        AudioLoad_StartAsyncLoad((*sample).sampleAddr as u32_0,
                                 (*topPreload).ramAddr as *mut libc::c_void,
                                 (*sample).size(), (*sample).medium() as s32,
                                 nChunks,
                                 &mut gAudioContext.preloadSampleQueue,
                                 (*topPreload).encodedInfo as s32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ProcessSamplePreloads(mut resetStatus: s32)
 -> s32 {
    let mut sample: *mut SoundFontSample = 0 as *mut SoundFontSample;
    let mut preload: *mut AudioPreloadReq = 0 as *mut AudioPreloadReq;
    let mut preloadIndex: u32_0 = 0;
    let mut key: u32_0 = 0;
    let mut nChunks: u32_0 = 0;
    let mut pad: s32 = 0;
    if gAudioContext.preloadSampleStackTop > 0 as libc::c_int {
        if resetStatus != 0 as libc::c_int {
            // Clear result queue and preload stack and return.
            osRecvMesg(&mut gAudioContext.preloadSampleQueue,
                       &mut preloadIndex as *mut u32_0 as *mut OSMesg,
                       0 as libc::c_int);
            gAudioContext.preloadSampleStackTop = 0 as libc::c_int;
            return 0 as libc::c_int
        }
        if osRecvMesg(&mut gAudioContext.preloadSampleQueue,
                      &mut preloadIndex as *mut u32_0 as *mut OSMesg,
                      0 as libc::c_int) == -(1 as libc::c_int) {
            // Previous preload is not done yet.
            return 0 as libc::c_int
        }
        preloadIndex >>= 24 as libc::c_int;
        preload =
            &mut *gAudioContext.preloadSampleStack.as_mut_ptr().offset(preloadIndex
                                                                           as
                                                                           isize)
                as *mut AudioPreloadReq;
        if (*preload).isFree == 0 as libc::c_int {
            sample = (*preload).sample;
            key =
                ((*sample).sampleAddr as
                     u32_0).wrapping_add((*sample).size()).wrapping_add((*sample).medium());
            if key == (*preload).endAndMediumKey {
                // Change storage for sample to the preloaded version.
                (*sample).sampleAddr = (*preload).ramAddr;
                (*sample).set_medium(MEDIUM_RAM as libc::c_int as u32_0)
            }
            (*preload).isFree = 1 as libc::c_int
        }
        // Pop requests with isFree = true off the stack, as far as possible,
        // and dispatch the next DMA.
        while !(gAudioContext.preloadSampleStackTop <= 0 as libc::c_int) {
            preload =
                &mut *gAudioContext.preloadSampleStack.as_mut_ptr().offset((gAudioContext.preloadSampleStackTop
                                                                                -
                                                                                1
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               isize)
                    as *mut AudioPreloadReq;
            if (*preload).isFree == 1 as libc::c_int {
                gAudioContext.preloadSampleStackTop -= 1
            } else {
                sample = (*preload).sample;
                nChunks =
                    (((*sample).size() as libc::c_int >> 12 as libc::c_int) +
                         1 as libc::c_int) as u32_0;
                key =
                    ((*sample).sampleAddr as
                         u32_0).wrapping_add((*sample).size()).wrapping_add((*sample).medium());
                if key != (*preload).endAndMediumKey {
                    (*preload).isFree = 1 as libc::c_int;
                    gAudioContext.preloadSampleStackTop -= 1
                } else {
                    AudioLoad_StartAsyncLoad((*sample).sampleAddr as u32_0,
                                             (*preload).ramAddr as
                                                 *mut libc::c_void,
                                             (*sample).size(),
                                             (*sample).medium() as s32,
                                             nChunks as s32,
                                             &mut gAudioContext.preloadSampleQueue,
                                             (*preload).encodedInfo as s32);
                    break ;
                }
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AddToSampleSet(mut sample:
                                                      *mut SoundFontSample,
                                                  mut numSamples: s32,
                                                  mut sampleSet:
                                                      *mut *mut SoundFontSample)
 -> s32 {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < numSamples {
        if (*sample).sampleAddr == (**sampleSet.offset(i as isize)).sampleAddr
           {
            break ;
        }
        i += 1
    }
    if i == numSamples {
        let ref mut fresh20 = *sampleSet.offset(numSamples as isize);
        *fresh20 = sample;
        numSamples += 1
    }
    return numSamples;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_GetSamplesForFont(mut fontId: s32,
                                                     mut sampleSet:
                                                         *mut *mut SoundFontSample)
 -> s32 {
    let mut i: s32 = 0;
    let mut numDrums: s32 = 0;
    let mut numInstruments: s32 = 0;
    let mut numSamples: s32 = 0 as libc::c_int;
    numDrums =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numDrums as s32;
    numInstruments =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numInstruments as
            s32;
    i = 0 as libc::c_int;
    while i < numDrums {
        let mut drum: *mut Drum = Audio_GetDrum(fontId, i);
        if !drum.is_null() {
            numSamples =
                AudioLoad_AddToSampleSet((*drum).sound.sample, numSamples,
                                         sampleSet)
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < numInstruments {
        let mut instrument: *mut Instrument =
            Audio_GetInstrumentInner(fontId, i);
        if !instrument.is_null() {
            if (*instrument).normalRangeLo as libc::c_int != 0 as libc::c_int
               {
                numSamples =
                    AudioLoad_AddToSampleSet((*instrument).lowNotesSound.sample,
                                             numSamples, sampleSet)
            }
            if (*instrument).normalRangeHi as libc::c_int !=
                   0x7f as libc::c_int {
                numSamples =
                    AudioLoad_AddToSampleSet((*instrument).highNotesSound.sample,
                                             numSamples, sampleSet)
            }
            numSamples =
                AudioLoad_AddToSampleSet((*instrument).normalNotesSound.sample,
                                         numSamples, sampleSet)
        }
        i += 1
    }
    // Should really also process sfx, but this method is never called, so whatever.
    return numSamples;
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_AddUsedSample(mut sound:
                                                     *mut SoundFontSound) {
    let mut sample: *mut SoundFontSample = (*sound).sample;
    if (*sample).size() as libc::c_int != 0 as libc::c_int &&
           (*sample).unk_bit26() as libc::c_int != 0 &&
           (*sample).medium() as libc::c_int != MEDIUM_RAM as libc::c_int {
        let fresh21 = gAudioContext.numUsedSamples;
        gAudioContext.numUsedSamples = gAudioContext.numUsedSamples + 1;
        gAudioContext.usedSamples[fresh21 as usize] = sample
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_PreloadSamplesForFont(mut fontId: s32,
                                                         mut async_0: s32,
                                                         mut relocInfo:
                                                             *mut RelocInfo) {
    let mut numDrums: s32 = 0;
    let mut numInstruments: s32 = 0;
    let mut numSfx: s32 = 0;
    let mut drum: *mut Drum = 0 as *mut Drum;
    let mut instrument: *mut Instrument = 0 as *mut Instrument;
    let mut sound: *mut SoundFontSound = 0 as *mut SoundFontSound;
    let mut preload: *mut AudioPreloadReq = 0 as *mut AudioPreloadReq;
    let mut topPreload: *mut AudioPreloadReq = 0 as *mut AudioPreloadReq;
    let mut addr: *mut u8_0 = 0 as *mut u8_0;
    let mut size: s32 = 0;
    let mut i: s32 = 0;
    let mut sample: *mut SoundFontSample = 0 as *mut SoundFontSample;
    let mut preloadInProgress: s32 = 0;
    let mut nChunks: s32 = 0;
    preloadInProgress = 0 as libc::c_int;
    if gAudioContext.preloadSampleStackTop != 0 as libc::c_int {
        preloadInProgress = 1 as libc::c_int
    }
    gAudioContext.numUsedSamples = 0 as libc::c_int;
    numDrums =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numDrums as s32;
    numInstruments =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numInstruments as
            s32;
    numSfx =
        (*gAudioContext.soundFonts.offset(fontId as isize)).numSfx as s32;
    i = 0 as libc::c_int;
    while i < numInstruments {
        instrument = Audio_GetInstrumentInner(fontId, i);
        if !instrument.is_null() {
            if (*instrument).normalRangeLo as libc::c_int != 0 as libc::c_int
               {
                AudioLoad_AddUsedSample(&mut (*instrument).lowNotesSound);
            }
            if (*instrument).normalRangeHi as libc::c_int !=
                   0x7f as libc::c_int {
                AudioLoad_AddUsedSample(&mut (*instrument).highNotesSound);
            }
            AudioLoad_AddUsedSample(&mut (*instrument).normalNotesSound);
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < numDrums {
        drum = Audio_GetDrum(fontId, i);
        if !drum.is_null() { AudioLoad_AddUsedSample(&mut (*drum).sound); }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < numSfx {
        sound = Audio_GetSfx(fontId, i);
        if !sound.is_null() { AudioLoad_AddUsedSample(sound); }
        i += 1
    }
    if gAudioContext.numUsedSamples == 0 as libc::c_int { return }
    size = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < gAudioContext.numUsedSamples {
        size +=
            (*gAudioContext.usedSamples[i as usize]).size() as libc::c_int +
                0xf as libc::c_int & !(0xf as libc::c_int);
        i += 1
    }
    (size) != 0;
    i = 0 as libc::c_int;
    while i < gAudioContext.numUsedSamples {
        if gAudioContext.preloadSampleStackTop == 120 as libc::c_int {
            break ;
        }
        sample = gAudioContext.usedSamples[i as usize];
        if !((*sample).medium() as libc::c_int == MEDIUM_RAM as libc::c_int) {
            match async_0 {
                0 => {
                    if (*sample).medium() == (*relocInfo).medium1 {
                        addr =
                            AudioHeap_AllocSampleCache((*sample).size(),
                                                       (*relocInfo).sampleBankId1,
                                                       (*sample).sampleAddr as
                                                           *mut libc::c_void,
                                                       (*sample).medium() as
                                                           s8,
                                                       CACHE_PERSISTENT as
                                                           libc::c_int) as
                                *mut u8_0
                    } else if (*sample).medium() == (*relocInfo).medium2 {
                        addr =
                            AudioHeap_AllocSampleCache((*sample).size(),
                                                       (*relocInfo).sampleBankId2,
                                                       (*sample).sampleAddr as
                                                           *mut libc::c_void,
                                                       (*sample).medium() as
                                                           s8,
                                                       CACHE_PERSISTENT as
                                                           libc::c_int) as
                                *mut u8_0
                    }
                }
                1 => {
                    if (*sample).medium() == (*relocInfo).medium1 {
                        addr =
                            AudioHeap_AllocSampleCache((*sample).size(),
                                                       (*relocInfo).sampleBankId1,
                                                       (*sample).sampleAddr as
                                                           *mut libc::c_void,
                                                       (*sample).medium() as
                                                           s8,
                                                       CACHE_TEMPORARY as
                                                           libc::c_int) as
                                *mut u8_0
                    } else if (*sample).medium() == (*relocInfo).medium2 {
                        addr =
                            AudioHeap_AllocSampleCache((*sample).size(),
                                                       (*relocInfo).sampleBankId2,
                                                       (*sample).sampleAddr as
                                                           *mut libc::c_void,
                                                       (*sample).medium() as
                                                           s8,
                                                       CACHE_TEMPORARY as
                                                           libc::c_int) as
                                *mut u8_0
                    }
                }
                _ => { }
            }
            if !addr.is_null() {
                match async_0 {
                    0 => {
                        if (*sample).medium() as libc::c_int ==
                               MEDIUM_UNK as libc::c_int {
                            AudioLoad_SyncDmaUnkMedium((*sample).sampleAddr as
                                                           u32_0, addr,
                                                       (*sample).size(),
                                                       (*gAudioContext.sampleBankTable).unkMediumParam
                                                           as s32);
                            (*sample).sampleAddr = addr;
                            (*sample).set_medium(MEDIUM_RAM as libc::c_int as
                                                     u32_0)
                        } else {
                            AudioLoad_SyncDma((*sample).sampleAddr as u32_0,
                                              addr, (*sample).size(),
                                              (*sample).medium() as s32);
                            (*sample).sampleAddr = addr;
                            (*sample).set_medium(MEDIUM_RAM as libc::c_int as
                                                     u32_0)
                        }
                    }
                    1 => {
                        preload =
                            &mut *gAudioContext.preloadSampleStack.as_mut_ptr().offset(gAudioContext.preloadSampleStackTop
                                                                                           as
                                                                                           isize)
                                as *mut AudioPreloadReq;
                        (*preload).sample = sample;
                        (*preload).ramAddr = addr;
                        (*preload).encodedInfo =
                            (gAudioContext.preloadSampleStackTop <<
                                 24 as libc::c_int | 0xffffff as libc::c_int)
                                as u32_0;
                        (*preload).isFree = 0 as libc::c_int;
                        (*preload).endAndMediumKey =
                            ((*sample).sampleAddr as
                                 u32_0).wrapping_add((*sample).size()).wrapping_add((*sample).medium());
                        gAudioContext.preloadSampleStackTop += 1
                    }
                    _ => { }
                }
            }
        }
        i += 1
    }
    gAudioContext.numUsedSamples = 0 as libc::c_int;
    if gAudioContext.preloadSampleStackTop != 0 as libc::c_int &&
           preloadInProgress == 0 {
        topPreload =
            &mut *gAudioContext.preloadSampleStack.as_mut_ptr().offset((gAudioContext.preloadSampleStackTop
                                                                            -
                                                                            1
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           isize)
                as *mut AudioPreloadReq;
        sample = (*topPreload).sample;
        nChunks =
            ((*sample).size() as libc::c_int >> 12 as libc::c_int) +
                1 as libc::c_int;
        AudioLoad_StartAsyncLoad((*sample).sampleAddr as u32_0,
                                 (*topPreload).ramAddr as *mut libc::c_void,
                                 (*sample).size(), (*sample).medium() as s32,
                                 nChunks,
                                 &mut gAudioContext.preloadSampleQueue,
                                 (*topPreload).encodedInfo as s32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_LoadPermanentSamples() {
    let mut pad: s32 = 0;
    let mut fontId: u32_0 = 0;
    let mut sampleBankTable: *mut AudioTable = 0 as *mut AudioTable;
    let mut pad2: s32 = 0;
    let mut i: s32 = 0;
    sampleBankTable = AudioLoad_GetLoadTable(SAMPLE_TABLE as libc::c_int);
    i = 0 as libc::c_int;
    while i < gAudioContext.permanentPool.count {
        let mut relocInfo: RelocInfo =
            RelocInfo{sampleBankId1: 0,
                      sampleBankId2: 0,
                      baseAddr1: 0,
                      baseAddr2: 0,
                      medium1: 0,
                      medium2: 0,};
        if gAudioContext.permanentCache[i as usize].tableType as libc::c_int
               == FONT_TABLE as libc::c_int {
            fontId =
                AudioLoad_GetRealTableIndex(FONT_TABLE as libc::c_int,
                                            gAudioContext.permanentCache[i as
                                                                             usize].id
                                                as u32_0);
            relocInfo.sampleBankId1 =
                (*gAudioContext.soundFonts.offset(fontId as
                                                      isize)).sampleBankId1 as
                    s32;
            relocInfo.sampleBankId2 =
                (*gAudioContext.soundFonts.offset(fontId as
                                                      isize)).sampleBankId2 as
                    s32;
            if relocInfo.sampleBankId1 != 0xff as libc::c_int {
                relocInfo.sampleBankId1 =
                    AudioLoad_GetRealTableIndex(SAMPLE_TABLE as libc::c_int,
                                                relocInfo.sampleBankId1 as
                                                    u32_0) as s32;
                relocInfo.medium1 =
                    (*(*sampleBankTable).entries.as_mut_ptr().offset(relocInfo.sampleBankId1
                                                                         as
                                                                         isize)).medium
                        as u32_0
            }
            if relocInfo.sampleBankId2 != 0xff as libc::c_int {
                relocInfo.sampleBankId2 =
                    AudioLoad_GetRealTableIndex(SAMPLE_TABLE as libc::c_int,
                                                relocInfo.sampleBankId2 as
                                                    u32_0) as s32;
                relocInfo.medium2 =
                    (*(*sampleBankTable).entries.as_mut_ptr().offset(relocInfo.sampleBankId2
                                                                         as
                                                                         isize)).medium
                        as u32_0
            }
            AudioLoad_PreloadSamplesForFont(fontId as s32, 0 as libc::c_int,
                                            &mut relocInfo);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_Unused3() { }
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_Unused4() { }
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_Unused5() { }
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ScriptLoad(mut tableType: s32, mut id: s32,
                                              mut isDone: *mut s8) {
    static mut sLoadIndex: u32_0 = 0 as libc::c_int as u32_0;
    sScriptLoadDonePointers[sLoadIndex as usize] = isDone;
    AudioLoad_AsyncLoad(tableType, id, 0 as libc::c_int, sLoadIndex as s32,
                        &mut sScriptLoadQueue);
    sLoadIndex = sLoadIndex.wrapping_add(1);
    if sLoadIndex == 0x10 as libc::c_int as libc::c_uint {
        sLoadIndex = 0 as libc::c_int as u32_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_ProcessScriptLoads() {
    let mut temp: u32_0 = 0;
    let mut sp20: u32_0 = 0;
    let mut isDone: *mut s8 = 0 as *mut s8;
    if osRecvMesg(&mut sScriptLoadQueue,
                  &mut sp20 as *mut u32_0 as *mut OSMesg, 0 as libc::c_int) !=
           -(1 as libc::c_int) {
        temp = sp20 >> 24 as libc::c_int;
        isDone = sScriptLoadDonePointers[temp as usize];
        if !isDone.is_null() { *isDone = 0 as libc::c_int as s8 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioLoad_InitScriptLoads() {
    osCreateMesgQueue(&mut sScriptLoadQueue, sScriptLoadMesgBuf.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 16]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
}
