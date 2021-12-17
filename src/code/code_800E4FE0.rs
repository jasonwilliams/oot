#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osGetCount() -> u32_0;
    #[no_mangle]
    fn AudioSynth_Update(cmdStart: *mut Acmd, cmdCnt: *mut s32,
                         aiStart: *mut s16, aiBufLen: s32) -> *mut Acmd;
    #[no_mangle]
    fn AudioHeap_AllocPoolInit(pool: *mut AudioAllocPool,
                               mem: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn AudioHeap_PopCache(tableType: s32);
    #[no_mangle]
    fn AudioHeap_ResetStep() -> s32;
    #[no_mangle]
    fn AudioLoad_DecreaseSampleDmaTtls();
    #[no_mangle]
    fn AudioLoad_SyncLoadSeqParts(seqId: s32, arg1: s32);
    #[no_mangle]
    fn AudioLoad_SyncLoadInstrument(fontId: s32, instId: s32, drumId: s32)
     -> s32;
    #[no_mangle]
    fn AudioLoad_AsyncLoadSeq(seqId: s32, arg1: s32, retData: s32,
                              retQueue: *mut OSMesgQueue);
    #[no_mangle]
    fn AudioLoad_AsyncLoadSampleBank(sampleBankId: s32, arg1: s32,
                                     retData: s32,
                                     retQueue: *mut OSMesgQueue);
    #[no_mangle]
    fn AudioLoad_AsyncLoadFont(fontId: s32, arg1: s32, retData: s32,
                               retQueue: *mut OSMesgQueue);
    #[no_mangle]
    fn AudioLoad_GetFontsForSequence(seqId: s32, arg1: *mut u32_0)
     -> *mut u8_0;
    #[no_mangle]
    fn AudioLoad_DiscardSeqFonts(seqId: s32);
    #[no_mangle]
    fn AudioLoad_SyncInitSeqPlayer(playerIdx: s32, seqId: s32, arg2: s32)
     -> s32;
    #[no_mangle]
    fn AudioLoad_SyncInitSeqPlayerSkipTicks(playerIdx: s32, seqId: s32,
                                            arg2: s32) -> s32;
    #[no_mangle]
    fn AudioLoad_ProcessLoads(resetStatus: s32);
    #[no_mangle]
    fn AudioLoad_ProcessScriptLoads();
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    #[no_mangle]
    static mut D_801120C0: [u64_0; 0];
    #[no_mangle]
    static mut gWaveSamples: [*mut s16; 9];
    #[no_mangle]
    fn AudioSeq_SequencePlayerDisableAsFinished(seqPlayer:
                                                    *mut SequencePlayer);
    #[no_mangle]
    fn Audio_SetFontInstrument(instrumentType: s32, fontId: s32, index: s32,
                               value: *mut libc::c_void) -> s32;
    #[no_mangle]
    static mut D_801755D0: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    fn osAiSetNextBuffer(_: *mut libc::c_void, _: u32_0) -> s32;
    #[no_mangle]
    fn osAiGetLength() -> u32_0;
    #[no_mangle]
    static mut gAudioContextInitalized: s32;
    #[no_mangle]
    static mut rspAspMainDataStart: [u64_0; 0];
    #[no_mangle]
    static mut rspAspMainDataEnd: [u64_0; 0];
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
pub const CHAN_UPD_STEREO: C2RustUnnamed_11 = 14;
pub const CHAN_UPD_UNK_20: C2RustUnnamed_11 = 13;
pub const CHAN_UPD_UNK_0F: C2RustUnnamed_11 = 12;
pub const CHAN_UPD_VIBE_X32: C2RustUnnamed_11 = 11;
pub const CHAN_UPD_VIBE_X8: C2RustUnnamed_11 = 10;
pub const CHAN_UPD_MUTE_BEHAVE: C2RustUnnamed_11 = 9;
pub const CHAN_UPD_STOP_SOMETHING2: C2RustUnnamed_11 = 8;
pub const CHAN_UPD_SCRIPT_IO: C2RustUnnamed_11 = 6;
pub const CHAN_UPD_REVERB: C2RustUnnamed_11 = 5;
pub const CHAN_UPD_FREQ_SCALE: C2RustUnnamed_11 = 4;
pub const CHAN_UPD_PAN_UNSIGNED: C2RustUnnamed_11 = 7;
pub const CHAN_UPD_PAN_SIGNED: C2RustUnnamed_11 = 3;
pub const CHAN_UPD_VOL: C2RustUnnamed_11 = 2;
pub const CHAN_UPD_VOL_SCALE: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const CHAN_UPD_UNK_0: C2RustUnnamed_11 = 0;
// AudioMgr_Retrace
#[no_mangle]
pub unsafe extern "C" fn func_800E4FE0() -> *mut AudioTask {
    return func_800E5000();
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5000() -> *mut AudioTask {
    static mut sMaxAbiCmdCnt: s32 = 0x80 as libc::c_int;
    static mut sWaitingAudioTask: *mut AudioTask =
        0 as *const AudioTask as *mut AudioTask;
    let mut samplesRemainingInAi: u32_0 = 0;
    let mut abiCmdCnt: s32 = 0;
    let mut pad: s32 = 0;
    let mut j: s32 = 0;
    let mut sp5C: s32 = 0;
    let mut currAiBuffer: *mut s16 = 0 as *mut s16;
    let mut task: *mut OSTask_t = 0 as *mut OSTask_t;
    let mut index: s32 = 0;
    let mut sp4C: u32_0 = 0;
    let mut sp48: s32 = 0;
    let mut i: s32 = 0;
    gAudioContext.totalTaskCnt += 1;
    if gAudioContext.totalTaskCnt %
           gAudioContext.audioBufferParameters.specUnk4 as libc::c_int !=
           0 as libc::c_int {
        if D_801755D0.is_some() {
            D_801755D0.expect("non-null function pointer")();
        }
        if gAudioContext.totalTaskCnt %
               gAudioContext.audioBufferParameters.specUnk4 as libc::c_int +
               1 as libc::c_int ==
               gAudioContext.audioBufferParameters.specUnk4 as libc::c_int {
            return sWaitingAudioTask
        } else { return 0 as *mut AudioTask }
    }
    osSendMesg(gAudioContext.taskStartQueueP,
               gAudioContext.totalTaskCnt as OSMesg, 0 as libc::c_int);
    gAudioContext.rspTaskIdx ^= 1 as libc::c_int;
    gAudioContext.curAIBufIdx += 1;
    gAudioContext.curAIBufIdx %= 3 as libc::c_int;
    index =
        (gAudioContext.curAIBufIdx - 2 as libc::c_int + 3 as libc::c_int) %
            3 as libc::c_int;
    samplesRemainingInAi =
        osAiGetLength().wrapping_div(4 as libc::c_int as libc::c_uint);
    if gAudioContext.resetTimer < 16 as libc::c_int as libc::c_uint {
        if gAudioContext.aiBufLengths[index as usize] as libc::c_int !=
               0 as libc::c_int {
            osAiSetNextBuffer(gAudioContext.aiBuffers[index as usize] as
                                  *mut libc::c_void,
                              (gAudioContext.aiBufLengths[index as usize] as
                                   libc::c_int * 4 as libc::c_int) as u32_0);
            !gAudioContext.aiBuffers[index as usize].is_null();
            (gAudioContext.aiBufLengths[index as usize]) != 0;
        }
    }
    if D_801755D0.is_some() {
        D_801755D0.expect("non-null function pointer")();
    }
    sp5C = gAudioContext.curAudioFrameDmaCount;
    i = 0 as libc::c_int;
    while i < gAudioContext.curAudioFrameDmaCount {
        if osRecvMesg(&mut gAudioContext.currAudioFrameDmaQueue,
                      0 as *mut OSMesg, 0 as libc::c_int) == 0 as libc::c_int
           {
            sp5C -= 1
        }
        i += 1
    }
    if sp5C != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < sp5C {
            osRecvMesg(&mut gAudioContext.currAudioFrameDmaQueue,
                       0 as *mut OSMesg, 1 as libc::c_int);
            i += 1
        }
    }
    sp48 = gAudioContext.currAudioFrameDmaQueue.validCount;
    if sp48 != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < sp48 {
            osRecvMesg(&mut gAudioContext.currAudioFrameDmaQueue,
                       0 as *mut OSMesg, 0 as libc::c_int);
            i += 1
        }
    }
    gAudioContext.curAudioFrameDmaCount = 0 as libc::c_int;
    AudioLoad_DecreaseSampleDmaTtls();
    AudioLoad_ProcessLoads(gAudioContext.resetStatus as s32);
    AudioLoad_ProcessScriptLoads();
    if gAudioContext.resetStatus as libc::c_int != 0 as libc::c_int {
        if AudioHeap_ResetStep() == 0 as libc::c_int {
            if gAudioContext.resetStatus as libc::c_int == 0 as libc::c_int {
                osSendMesg(gAudioContext.audioResetQueueP,
                           gAudioContext.audioResetSpecIdToLoad as OSMesg,
                           0 as libc::c_int);
            }
            sWaitingAudioTask = 0 as *mut AudioTask;
            return 0 as *mut AudioTask
        }
    }
    if gAudioContext.resetTimer > 16 as libc::c_int as libc::c_uint {
        return 0 as *mut AudioTask
    }
    if gAudioContext.resetTimer != 0 as libc::c_int as libc::c_uint {
        ::std::ptr::write_volatile(&mut gAudioContext.resetTimer as
                                       *mut u32_0,
                                   ::std::ptr::read_volatile::<u32_0>(&gAudioContext.resetTimer
                                                                          as
                                                                          *const u32_0).wrapping_add(1))
    }
    gAudioContext.currTask =
        &mut *gAudioContext.rspTask.as_mut_ptr().offset(gAudioContext.rspTaskIdx
                                                            as isize) as
            *mut AudioTask;
    gAudioContext.curAbiCmdBuf =
        gAudioContext.abiCmdBufs[gAudioContext.rspTaskIdx as usize];
    index = gAudioContext.curAIBufIdx;
    currAiBuffer = gAudioContext.aiBuffers[index as usize];
    gAudioContext.aiBufLengths[index as usize] =
        ((gAudioContext.audioBufferParameters.samplesPerFrameTarget as
              libc::c_uint).wrapping_sub(samplesRemainingInAi).wrapping_add(0x80
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
             &
             !(0xf as libc::c_int) as
                 libc::c_uint).wrapping_add(0x10 as libc::c_int as
                                                libc::c_uint) as s16;
    if (gAudioContext.aiBufLengths[index as usize] as libc::c_int) <
           gAudioContext.audioBufferParameters.minAiBufferLength as
               libc::c_int {
        gAudioContext.aiBufLengths[index as usize] =
            gAudioContext.audioBufferParameters.minAiBufferLength
    }
    if gAudioContext.aiBufLengths[index as usize] as libc::c_int >
           gAudioContext.audioBufferParameters.maxAiBufferLength as
               libc::c_int {
        gAudioContext.aiBufLengths[index as usize] =
            gAudioContext.audioBufferParameters.maxAiBufferLength
    }
    j = 0 as libc::c_int;
    if gAudioContext.resetStatus as libc::c_int == 0 as libc::c_int {
        // msg = 0000RREE R = read pos, E = End Pos
        while osRecvMesg(gAudioContext.cmdProcQueueP,
                         &mut sp4C as *mut u32_0 as *mut OSMesg,
                         0 as libc::c_int) != -(1 as libc::c_int) {
            Audio_ProcessCmds(sp4C);
            j += 1
        }
        if j == 0 as libc::c_int &&
               gAudioContext.cmdQueueFinished as libc::c_int != 0 {
            Audio_ScheduleProcessCmds();
        }
    }
    gAudioContext.curAbiCmdBuf =
        AudioSynth_Update(gAudioContext.curAbiCmdBuf, &mut abiCmdCnt,
                          currAiBuffer,
                          gAudioContext.aiBufLengths[index as usize] as s32);
    gAudioContext.audioRandom =
        gAudioContext.audioRandom.wrapping_add(gAudioContext.totalTaskCnt as
                                                   libc::c_uint).wrapping_mul(osGetCount());
    gAudioContext.audioRandom =
        (*gAudioContext.aiBuffers[index as
                                      usize].offset((gAudioContext.totalTaskCnt
                                                         &
                                                         0xff as libc::c_int)
                                                        as isize) as
             libc::c_uint).wrapping_add(gAudioContext.audioRandom);
    gWaveSamples[8 as libc::c_int as usize] =
        ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                           -> *mut AudioTask>,
                                *mut u8_0>(Some(func_800E4FE0 as
                                                    unsafe extern "C" fn()
                                                        ->
                                                            *mut AudioTask)).offset((gAudioContext.audioRandom
                                                                                         &
                                                                                         0xfff0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint)
                                                                                        as
                                                                                        isize)
            as *mut s16;
    index = gAudioContext.rspTaskIdx;
    (*gAudioContext.currTask).taskQueue = 0 as *mut OSMesgQueue;
    (*gAudioContext.currTask).unk_44 = 0 as *mut libc::c_void;
    task = &mut (*gAudioContext.currTask).task.t;
    (*task).type_0 = 2 as libc::c_int as u32_0;
    (*task).flags = 0 as libc::c_int as u32_0;
    (*task).ucode_boot = D_801120C0.as_mut_ptr();
    (*task).ucode_boot_size = 0x1000 as libc::c_int as u32_0;
    (*task).ucode_data_size =
        (rspAspMainDataEnd.as_mut_ptr().wrapping_offset_from(rspAspMainDataStart.as_mut_ptr())
             as libc::c_int as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<u64_0>() as
                                            libc::c_ulong).wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint);
    (*task).ucode = D_801120C0.as_mut_ptr();
    (*task).ucode_data = rspAspMainDataStart.as_mut_ptr();
    (*task).ucode_size = 0x1000 as libc::c_int as u32_0;
    (*task).dram_stack = 0 as *mut u64_0;
    (*task).dram_stack_size = 0 as libc::c_int as u32_0;
    (*task).output_buff = 0 as *mut u64_0;
    (*task).output_buff_size = 0 as *mut u64_0;
    (*task).data_ptr = gAudioContext.abiCmdBufs[index as usize] as *mut u64_0;
    (*task).data_size =
        (abiCmdCnt as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<Acmd>() as
                                            libc::c_ulong);
    (*task).yield_data_ptr = 0 as *mut u64_0;
    (*task).yield_data_size = 0 as libc::c_int as u32_0;
    if sMaxAbiCmdCnt < abiCmdCnt { sMaxAbiCmdCnt = abiCmdCnt }
    if gAudioContext.audioBufferParameters.specUnk4 as libc::c_int ==
           1 as libc::c_int {
        return gAudioContext.currTask
    } else {
        sWaitingAudioTask = gAudioContext.currTask;
        return 0 as *mut AudioTask
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5584(mut cmd: *mut AudioCmd) {
    let mut i: s32 = 0;
    let mut pad: s32 = 0;
    let mut pad2: s32 = 0;
    let mut temp_a1_5: u32_0 = 0;
    let mut temp_t7: u32_0 = 0;
    match (*cmd).c2rust_unnamed.c2rust_unnamed.op as libc::c_int {
        129 => {
            AudioLoad_SyncLoadSeqParts((*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                           as s32,
                                       (*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                           as s32);
            return
        }
        130 => {
            AudioLoad_SyncInitSeqPlayer((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                            as s32,
                                        (*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                            as s32,
                                        (*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                            as s32);
            func_800E59AC((*cmd).c2rust_unnamed.c2rust_unnamed.arg0 as s32,
                          (*cmd).c2rust_unnamed_0.data as s32);
            return
        }
        133 => {
            AudioLoad_SyncInitSeqPlayerSkipTicks((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                                     as s32,
                                                 (*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                                     as s32,
                                                 (*cmd).c2rust_unnamed_0.data
                                                     as s32);
            return
        }
        131 => {
            if gAudioContext.seqPlayers[(*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                            as usize].enabled() != 0 {
                if (*cmd).c2rust_unnamed_0.asInt == 0 as libc::c_int {
                    AudioSeq_SequencePlayerDisableAsFinished(&mut *gAudioContext.seqPlayers.as_mut_ptr().offset((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                                                                                                    as
                                                                                                                    isize));
                } else {
                    func_800E5958((*cmd).c2rust_unnamed.c2rust_unnamed.arg0 as
                                      s32, (*cmd).c2rust_unnamed_0.asInt);
                }
            }
            return
        }
        240 => {
            gAudioContext.soundMode = (*cmd).c2rust_unnamed_0.asUInt as s8;
            return
        }
        241 => {
            i = 0 as libc::c_int;
            while i <
                      gAudioContext.audioBufferParameters.numSequencePlayers
                          as libc::c_int {
                let mut seqPlayer: *mut SequencePlayer =
                    &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(i as
                                                                           isize)
                        as *mut SequencePlayer;
                (*seqPlayer).set_muted(1 as libc::c_int as u8_0);
                (*seqPlayer).set_recalculateVolume(1 as libc::c_int as u8_0);
                i += 1
            }
            return
        }
        242 => {
            if (*cmd).c2rust_unnamed_0.asUInt ==
                   1 as libc::c_int as libc::c_uint {
                i = 0 as libc::c_int;
                while i < gAudioContext.numNotes {
                    let mut note: *mut Note =
                        &mut *gAudioContext.notes.offset(i as isize) as
                            *mut Note;
                    let mut subEu: *mut NoteSubEu = &mut (*note).noteSubEu;
                    if (*subEu).bitField0.enabled() as libc::c_int != 0 &&
                           (*note).playbackState.unk_04 as libc::c_int ==
                               0 as libc::c_int {
                        if (*(*(*note).playbackState.parentLayer).channel).muteBehavior
                               as libc::c_int & 8 as libc::c_int != 0 {
                            (*subEu).bitField0.set_finished(1 as libc::c_int
                                                                as u8_0)
                        }
                    }
                    i += 1
                }
            }
            i = 0 as libc::c_int;
            while i <
                      gAudioContext.audioBufferParameters.numSequencePlayers
                          as libc::c_int {
                let mut seqPlayer_0: *mut SequencePlayer =
                    &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(i as
                                                                           isize)
                        as *mut SequencePlayer;
                (*seqPlayer_0).set_muted(0 as libc::c_int as u8_0);
                (*seqPlayer_0).set_recalculateVolume(1 as libc::c_int as
                                                         u8_0);
                i += 1
            }
            return
        }
        243 => {
            AudioLoad_SyncLoadInstrument((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                             as s32,
                                         (*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                             as s32,
                                         (*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                             as s32);
            return
        }
        244 => {
            AudioLoad_AsyncLoadSampleBank((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                              as s32,
                                          (*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                              as s32,
                                          (*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                              as s32,
                                          &mut gAudioContext.externalLoadQueue);
            return
        }
        245 => {
            AudioLoad_AsyncLoadFont((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                        as s32,
                                    (*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                        as s32,
                                    (*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                        as s32,
                                    &mut gAudioContext.externalLoadQueue);
            return
        }
        252 => {
            AudioLoad_AsyncLoadSeq((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                       as s32,
                                   (*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                       as s32,
                                   (*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                       as s32,
                                   &mut gAudioContext.externalLoadQueue);
            return
        }
        246 => {
            AudioLoad_DiscardSeqFonts((*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                          as s32);
            return
        }
        144 => {
            gAudioContext.unk_5BDC[(*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                       as usize] =
                (*cmd).c2rust_unnamed_0.asUShort;
            return
        }
        249 => {
            ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as
                                           *mut u8_0,
                                       5 as libc::c_int as u8_0);
            gAudioContext.audioResetSpecIdToLoad =
                (*cmd).c2rust_unnamed_0.asUInt as u8_0;
            return
        }
        251 => {
            D_801755D0 =
                ::std::mem::transmute::<libc::intptr_t,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       ()>>((*cmd).c2rust_unnamed_0.asUInt
                                                                as
                                                                libc::intptr_t);
            return
        }
        224 | 225 | 226 => {
            Audio_SetFontInstrument((*cmd).c2rust_unnamed.c2rust_unnamed.op as
                                        libc::c_int - 0xe0 as libc::c_int,
                                    (*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                        as s32,
                                    (*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                        as s32, (*cmd).c2rust_unnamed_0.data);
            return
        }
        254 => {
            temp_t7 = (*cmd).c2rust_unnamed_0.asUInt;
            if temp_t7 == 1 as libc::c_int as libc::c_uint {
                i = 0 as libc::c_int;
                while i <
                          gAudioContext.audioBufferParameters.numSequencePlayers
                              as libc::c_int {
                    let mut seqPlayer_1: *mut SequencePlayer =
                        &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)
                            as *mut SequencePlayer;
                    if (*seqPlayer_1).enabled() != 0 {
                        AudioSeq_SequencePlayerDisableAsFinished(seqPlayer_1);
                    }
                    i += 1
                }
            }
            func_800E66C0(temp_t7 as s32);
            return
        }
        227 => { AudioHeap_PopCache((*cmd).c2rust_unnamed_0.asInt); return }
        _ => { return }
    };
}
// SetFadeOutTimer
#[no_mangle]
pub unsafe extern "C" fn func_800E5958(mut playerIdx: s32,
                                       mut fadeTimer: s32) {
    let mut seqPlayer: *mut SequencePlayer =
        &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(playerIdx as isize)
            as *mut SequencePlayer;
    if fadeTimer == 0 as libc::c_int { fadeTimer = 1 as libc::c_int }
    (*seqPlayer).fadeVelocity =
        -((*seqPlayer).fadeVolume / fadeTimer as libc::c_float);
    (*seqPlayer).state = 2 as libc::c_int as u8_0;
    (*seqPlayer).fadeTimer = fadeTimer as u16_0;
}
// SetFadeInTimer
#[no_mangle]
pub unsafe extern "C" fn func_800E59AC(mut playerIdx: s32,
                                       mut fadeTimer: s32) {
    let mut seqPlayer: *mut SequencePlayer = 0 as *mut SequencePlayer;
    if fadeTimer != 0 as libc::c_int {
        seqPlayer =
            &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(playerIdx as
                                                                   isize) as
                *mut SequencePlayer;
        (*seqPlayer).state = 1 as libc::c_int as u8_0;
        (*seqPlayer).fadeTimerUnkEu = fadeTimer as u16_0;
        (*seqPlayer).fadeTimer = fadeTimer as u16_0;
        (*seqPlayer).fadeVolume = 0.0f32;
        (*seqPlayer).fadeVelocity = 0.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitMesgQueuesInternal() {
    gAudioContext.cmdWrPos = 0 as libc::c_int as u8_0;
    gAudioContext.cmdRdPos = 0 as libc::c_int as u8_0;
    gAudioContext.cmdQueueFinished = 0 as libc::c_int as u8_0;
    gAudioContext.taskStartQueueP = &mut gAudioContext.taskStartQueue;
    gAudioContext.cmdProcQueueP = &mut gAudioContext.cmdProcQueue;
    gAudioContext.audioResetQueueP = &mut gAudioContext.audioResetQueue;
    osCreateMesgQueue(gAudioContext.taskStartQueueP,
                      gAudioContext.taskStartMsgs.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 1]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
    osCreateMesgQueue(gAudioContext.cmdProcQueueP,
                      gAudioContext.cmdProcMsgs.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 4]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
    osCreateMesgQueue(gAudioContext.audioResetQueueP,
                      gAudioContext.audioResetMesgs.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 1]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_QueueCmd(mut opArgs: u32_0,
                                        mut data: *mut *mut libc::c_void) {
    let mut cmd: *mut AudioCmd =
        &mut *gAudioContext.cmdBuf.as_mut_ptr().offset((gAudioContext.cmdWrPos
                                                            as libc::c_int &
                                                            0xff as
                                                                libc::c_int)
                                                           as isize) as
            *mut AudioCmd;
    (*cmd).c2rust_unnamed.opArgs = opArgs;
    (*cmd).c2rust_unnamed_0.data = *data;
    gAudioContext.cmdWrPos = gAudioContext.cmdWrPos.wrapping_add(1);
    if gAudioContext.cmdWrPos as libc::c_int ==
           gAudioContext.cmdRdPos as libc::c_int {
        gAudioContext.cmdWrPos = gAudioContext.cmdWrPos.wrapping_sub(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_QueueCmdF32(mut opArgs: u32_0,
                                           mut data: f32_0) {
    Audio_QueueCmd(opArgs, &mut data as *mut f32_0 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_QueueCmdS32(mut opArgs: u32_0, mut data: s32) {
    Audio_QueueCmd(opArgs, &mut data as *mut s32 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_QueueCmdS8(mut opArgs: u32_0, mut data: s8) {
    let mut uData: u32_0 =
        ((data as libc::c_int) << 0x18 as libc::c_int) as u32_0;
    Audio_QueueCmd(opArgs,
                   &mut uData as *mut u32_0 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_QueueCmdU16(mut opArgs: u32_0,
                                           mut data: u16_0) {
    let mut uData: u32_0 =
        ((data as libc::c_int) << 0x10 as libc::c_int) as u32_0;
    Audio_QueueCmd(opArgs,
                   &mut uData as *mut u32_0 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ScheduleProcessCmds() -> s32 {
    static mut D_801304E8: s32 = 0 as libc::c_int;
    let mut ret: s32 = 0;
    if D_801304E8 <
           (gAudioContext.cmdWrPos as libc::c_int -
                gAudioContext.cmdRdPos as libc::c_int + 0x100 as libc::c_int)
               as u8_0 as libc::c_int {
        D_801304E8 =
            (gAudioContext.cmdWrPos as libc::c_int -
                 gAudioContext.cmdRdPos as libc::c_int + 0x100 as libc::c_int)
                as u8_0 as s32
    }
    ret =
        osSendMesg(gAudioContext.cmdProcQueueP,
                   ((gAudioContext.cmdRdPos as libc::c_int &
                         0xff as libc::c_int) << 8 as libc::c_int |
                        gAudioContext.cmdWrPos as libc::c_int &
                            0xff as libc::c_int) as *mut libc::c_void,
                   0 as libc::c_int);
    if ret != -(1 as libc::c_int) {
        gAudioContext.cmdRdPos = gAudioContext.cmdWrPos;
        ret = 0 as libc::c_int
    } else { return -(1 as libc::c_int) }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ResetCmdQueue() {
    gAudioContext.cmdQueueFinished = 0 as libc::c_int as u8_0;
    gAudioContext.cmdRdPos = gAudioContext.cmdWrPos;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ProcessCmd(mut cmd: *mut AudioCmd) {
    let mut seqPlayer: *mut SequencePlayer = 0 as *mut SequencePlayer;
    let mut phi_v0: u16_0 = 0;
    let mut i: s32 = 0;
    if (*cmd).c2rust_unnamed.c2rust_unnamed.op as libc::c_int &
           0xf0 as libc::c_int == 0xf0 as libc::c_int {
        func_800E5584(cmd);
        return
    }
    if ((*cmd).c2rust_unnamed.c2rust_unnamed.arg0 as libc::c_int) <
           gAudioContext.audioBufferParameters.numSequencePlayers as
               libc::c_int {
        seqPlayer =
            &mut *gAudioContext.seqPlayers.as_mut_ptr().offset((*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                                                   as isize)
                as *mut SequencePlayer;
        if (*cmd).c2rust_unnamed.c2rust_unnamed.op as libc::c_int &
               0x80 as libc::c_int != 0 {
            func_800E5584(cmd);
            return
        }
        if (*cmd).c2rust_unnamed.c2rust_unnamed.op as libc::c_int &
               0x40 as libc::c_int != 0 {
            func_800E6128(seqPlayer, cmd);
            return
        }
        if ((*cmd).c2rust_unnamed.c2rust_unnamed.arg1 as libc::c_int) <
               0x10 as libc::c_int {
            func_800E6300((*seqPlayer).channels[(*cmd).c2rust_unnamed.c2rust_unnamed.arg1
                                                    as usize], cmd);
            return
        }
        if (*cmd).c2rust_unnamed.c2rust_unnamed.arg1 as libc::c_int ==
               0xff as libc::c_int {
            phi_v0 =
                gAudioContext.unk_5BDC[(*cmd).c2rust_unnamed.c2rust_unnamed.arg0
                                           as usize];
            i = 0 as libc::c_int;
            while i < 0x10 as libc::c_int {
                if phi_v0 as libc::c_int & 1 as libc::c_int != 0 {
                    func_800E6300((*seqPlayer).channels[i as usize], cmd);
                }
                phi_v0 = (phi_v0 as libc::c_int >> 1 as libc::c_int) as u16_0;
                i += 1
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ProcessCmds(mut msg: u32_0) {
    static mut curCmdRdPos: u8_0 = 0 as libc::c_int as u8_0;
    let mut cmd: *mut AudioCmd = 0 as *mut AudioCmd;
    let mut endPos: u8_0 = 0;
    if gAudioContext.cmdQueueFinished == 0 {
        curCmdRdPos = (msg >> 8 as libc::c_int) as u8_0
    }
    loop  {
        endPos = (msg & 0xff as libc::c_int as libc::c_uint) as u8_0;
        if curCmdRdPos as libc::c_int == endPos as libc::c_int {
            gAudioContext.cmdQueueFinished = 0 as libc::c_int as u8_0;
            return
        }
        let fresh0 = curCmdRdPos;
        curCmdRdPos = curCmdRdPos.wrapping_add(1);
        cmd =
            &mut *gAudioContext.cmdBuf.as_mut_ptr().offset((fresh0 as
                                                                libc::c_int &
                                                                0xff as
                                                                    libc::c_int)
                                                               as isize) as
                *mut AudioCmd;
        if (*cmd).c2rust_unnamed.c2rust_unnamed.op as libc::c_int ==
               0xf8 as libc::c_int {
            gAudioContext.cmdQueueFinished = 1 as libc::c_int as u8_0;
            return
        }
        Audio_ProcessCmd(cmd);
        (*cmd).c2rust_unnamed.c2rust_unnamed.op = 0 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5E20(mut out: *mut u32_0) -> u32_0 {
    let mut sp1C: u32_0 = 0;
    if osRecvMesg(&mut gAudioContext.externalLoadQueue,
                  &mut sp1C as *mut u32_0 as *mut OSMesg, 0 as libc::c_int) ==
           -(1 as libc::c_int) {
        *out = 0 as libc::c_int as u32_0;
        return 0 as libc::c_int as u32_0
    }
    *out = sp1C & 0xffffff as libc::c_int as libc::c_uint;
    return sp1C >> 0x18 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5E84(mut arg0: s32, mut arg1: *mut u32_0)
 -> *mut u8_0 {
    return AudioLoad_GetFontsForSequence(arg0, arg1);
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5EA4(mut arg0: s32, mut arg1: *mut u32_0,
                                       mut arg2: *mut u32_0) {
    *arg1 =
        (*gAudioContext.soundFonts.offset(arg0 as isize)).sampleBankId1 as
            u32_0;
    *arg2 =
        (*gAudioContext.soundFonts.offset(arg0 as isize)).sampleBankId2 as
            u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5EDC() -> s32 {
    let mut pad: s32 = 0;
    let mut sp18: s32 = 0;
    if osRecvMesg(gAudioContext.audioResetQueueP,
                  &mut sp18 as *mut s32 as *mut OSMesg, 0 as libc::c_int) ==
           -(1 as libc::c_int) {
        return 0 as libc::c_int
    } else if gAudioContext.audioResetSpecIdToLoad as libc::c_int != sp18 {
        return -(1 as libc::c_int)
    } else { return 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5F34() {
    // macro?
    // clang-format off
    let mut chk: s32 = -(1 as libc::c_int);
    let mut sp28: s32 = 0;
    while osRecvMesg(gAudioContext.audioResetQueueP,
                     &mut sp28 as *mut s32 as *mut OSMesg, 0 as libc::c_int)
              != chk {
    };
    // clang-format on
}
#[no_mangle]
pub unsafe extern "C" fn func_800E5F88(mut resetPreloadID: s32) -> s32 {
    let mut resetStatus: s32 = 0;
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut pad: s32 = 0;
    func_800E5F34();
    resetStatus = gAudioContext.resetStatus as s32;
    if resetStatus != 0 as libc::c_int {
        Audio_ResetCmdQueue();
        if gAudioContext.audioResetSpecIdToLoad as libc::c_int ==
               resetPreloadID {
            return -(2 as libc::c_int)
        } else {
            if resetStatus > 2 as libc::c_int {
                gAudioContext.audioResetSpecIdToLoad = resetPreloadID as u8_0;
                return -(3 as libc::c_int)
            } else {
                osRecvMesg(gAudioContext.audioResetQueueP, &mut msg,
                           1 as libc::c_int);
            }
        }
    }
    func_800E5F34();
    Audio_QueueCmdS32(0xf9000000 as libc::c_uint, resetPreloadID);
    return Audio_ScheduleProcessCmds();
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PreNMIInternal() {
    ::std::ptr::write_volatile(&mut gAudioContext.resetTimer as *mut u32_0,
                               1 as libc::c_int as u32_0);
    if gAudioContextInitalized != 0 {
        func_800E5F88(0 as libc::c_int);
        ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as
                                       *mut u8_0, 0 as libc::c_int as u8_0)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800E6070(mut playerIdx: s32,
                                       mut channelIdx: s32,
                                       mut scriptIdx: s32) -> s8 {
    let mut seqPlayer: *mut SequencePlayer =
        &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(playerIdx as isize)
            as *mut SequencePlayer;
    let mut channel: *mut SequenceChannel = 0 as *mut SequenceChannel;
    if (*seqPlayer).enabled() != 0 {
        channel = (*seqPlayer).channels[channelIdx as usize];
        return (*channel).soundScriptIO[scriptIdx as usize]
    } else { return -(1 as libc::c_int) as s8 };
}
#[no_mangle]
pub unsafe extern "C" fn func_800E60C4(mut playerIdx: s32, mut arg1: s32)
 -> s8 {
    return gAudioContext.seqPlayers[playerIdx as
                                        usize].soundScriptIO[arg1 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitExternalPool(mut mem: *mut libc::c_void,
                                                mut size: u32_0) {
    AudioHeap_AllocPoolInit(&mut gAudioContext.externalPool, mem, size);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_DestroyExternalPool() {
    gAudioContext.externalPool.start = 0 as *mut u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800E6128(mut seqPlayer: *mut SequencePlayer,
                                       mut cmd: *mut AudioCmd) {
    let mut fadeVolume: f32_0 = 0.;
    let mut current_block_43: u64;
    match (*cmd).c2rust_unnamed.c2rust_unnamed.op as libc::c_int {
        65 => {
            if (*seqPlayer).fadeVolumeScale != (*cmd).c2rust_unnamed_0.asFloat
               {
                (*seqPlayer).fadeVolumeScale =
                    (*cmd).c2rust_unnamed_0.asFloat;
                (*seqPlayer).set_recalculateVolume(1 as libc::c_int as u8_0)
            }
            return
        }
        71 => {
            (*seqPlayer).tempo =
                ((*cmd).c2rust_unnamed_0.asInt * 0x30 as libc::c_int) as
                    u16_0;
            return
        }
        73 => {
            (*seqPlayer).unk_0C =
                ((*cmd).c2rust_unnamed_0.asInt * 0x30 as libc::c_int) as
                    u16_0;
            return
        }
        78 => {
            (*seqPlayer).unk_0C = (*cmd).c2rust_unnamed_0.asInt as u16_0;
            return
        }
        72 => {
            (*seqPlayer).transposition =
                (*cmd).c2rust_unnamed_0.asSbyte as s16;
            return
        }
        70 => {
            (*seqPlayer).soundScriptIO[(*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                           as usize] =
                (*cmd).c2rust_unnamed_0.asSbyte;
            return
        }
        74 => {
            fadeVolume =
                (*cmd).c2rust_unnamed.c2rust_unnamed.arg1 as s32 as
                    libc::c_float / 127.0f32;
            current_block_43 = 14324732909660258251;
        }
        75 => {
            fadeVolume =
                (*cmd).c2rust_unnamed.c2rust_unnamed.arg1 as s32 as
                    libc::c_float / 100.0f32 * (*seqPlayer).fadeVolume;
            current_block_43 = 14324732909660258251;
        }
        76 => {
            if (*seqPlayer).state as libc::c_int != 2 as libc::c_int {
                if (*cmd).c2rust_unnamed_0.asInt == 0 as libc::c_int {
                    (*seqPlayer).fadeVolume = (*seqPlayer).volume
                } else {
                    let mut tmp_0: s32 = (*cmd).c2rust_unnamed_0.asInt;
                    (*seqPlayer).state = 0 as libc::c_int as u8_0;
                    (*seqPlayer).fadeTimer = tmp_0 as u16_0;
                    (*seqPlayer).fadeVelocity =
                        ((*seqPlayer).volume - (*seqPlayer).fadeVolume) /
                            tmp_0 as libc::c_float
                }
            }
            return
        }
        77 => {
            (*seqPlayer).unk_34 = (*cmd).c2rust_unnamed_0.asFloat;
            if (*seqPlayer).unk_34 == 1.0f32 {
                (*seqPlayer).set_unk_0b1(0 as libc::c_int as u8_0)
            } else { (*seqPlayer).set_unk_0b1(1 as libc::c_int as u8_0) }
            current_block_43 = 1356832168064818221;
        }
        _ => { current_block_43 = 1356832168064818221; }
    }
    match current_block_43 {
        1356832168064818221 => { }
        _ => {
            if (*seqPlayer).state as libc::c_int != 2 as libc::c_int {
                (*seqPlayer).volume = (*seqPlayer).fadeVolume;
                if (*cmd).c2rust_unnamed_0.asInt == 0 as libc::c_int {
                    (*seqPlayer).fadeVolume = fadeVolume
                } else {
                    let mut tmp: s32 = (*cmd).c2rust_unnamed_0.asInt;
                    (*seqPlayer).state = 0 as libc::c_int as u8_0;
                    (*seqPlayer).fadeTimer = tmp as u16_0;
                    (*seqPlayer).fadeVelocity =
                        (fadeVolume - (*seqPlayer).fadeVolume) /
                            tmp as libc::c_float
                }
            }
            return
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800E6300(mut channel: *mut SequenceChannel,
                                       mut cmd: *mut AudioCmd) {
    match (*cmd).c2rust_unnamed.c2rust_unnamed.op as libc::c_int {
        1 => {
            if (*channel).volumeScale != (*cmd).c2rust_unnamed_0.asFloat {
                (*channel).volumeScale = (*cmd).c2rust_unnamed_0.asFloat;
                (*channel).changes.s.set_volume(1 as libc::c_int as u8_0)
            }
            return
        }
        2 => {
            if (*channel).volume != (*cmd).c2rust_unnamed_0.asFloat {
                (*channel).volume = (*cmd).c2rust_unnamed_0.asFloat;
                (*channel).changes.s.set_volume(1 as libc::c_int as u8_0)
            }
            return
        }
        3 => {
            if (*channel).newPan as libc::c_int !=
                   (*cmd).c2rust_unnamed_0.asSbyte as libc::c_int {
                (*channel).newPan = (*cmd).c2rust_unnamed_0.asSbyte as u8_0;
                (*channel).changes.s.set_pan(1 as libc::c_int as u8_0)
            }
            return
        }
        7 => {
            if (*channel).newPan as libc::c_int !=
                   (*cmd).c2rust_unnamed_0.asSbyte as libc::c_int {
                (*channel).panChannelWeight =
                    (*cmd).c2rust_unnamed_0.asSbyte as u8_0;
                (*channel).changes.s.set_pan(1 as libc::c_int as u8_0)
            }
            return
        }
        4 => {
            if (*channel).freqScale != (*cmd).c2rust_unnamed_0.asFloat {
                (*channel).freqScale = (*cmd).c2rust_unnamed_0.asFloat;
                (*channel).changes.s.set_freqScale(1 as libc::c_int as u8_0)
            }
            return
        }
        5 => {
            if (*channel).reverb as libc::c_int !=
                   (*cmd).c2rust_unnamed_0.asSbyte as libc::c_int {
                (*channel).reverb = (*cmd).c2rust_unnamed_0.asSbyte as u8_0
            }
            return
        }
        6 => {
            if ((*cmd).c2rust_unnamed.c2rust_unnamed.arg2 as libc::c_int) <
                   8 as libc::c_int {
                (*channel).soundScriptIO[(*cmd).c2rust_unnamed.c2rust_unnamed.arg2
                                             as usize] =
                    (*cmd).c2rust_unnamed_0.asSbyte
            }
            return
        }
        8 => {
            (*channel).set_stopSomething2((*cmd).c2rust_unnamed_0.asSbyte as
                                              u8_0);
            return
        }
        9 => {
            (*channel).muteBehavior = (*cmd).c2rust_unnamed_0.asSbyte as u8_0;
            return
        }
        10 => {
            (*channel).vibratoExtentTarget =
                ((*cmd).c2rust_unnamed_0.asUbyte as libc::c_int *
                     8 as libc::c_int) as u16_0;
            (*channel).vibratoExtentChangeDelay = 1 as libc::c_int as u16_0;
            return
        }
        11 => {
            (*channel).vibratoRateTarget =
                ((*cmd).c2rust_unnamed_0.asUbyte as libc::c_int *
                     32 as libc::c_int) as u16_0;
            (*channel).vibratoRateChangeDelay = 1 as libc::c_int as u16_0;
            return
        }
        12 => { (*channel).unk_0F = (*cmd).c2rust_unnamed_0.asUbyte; return }
        13 => { (*channel).unk_20 = (*cmd).c2rust_unnamed_0.asUShort; return }
        14 => {
            (*channel).stereo.asByte = (*cmd).c2rust_unnamed_0.asUbyte;
            return
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800E64B0(mut arg0: s32, mut arg1: s32,
                                       mut arg2: s32) {
    Audio_QueueCmdS32(((arg0 & 0xff as libc::c_int) << 0x10 as libc::c_int) as
                          libc::c_uint | 0xfa000000 as libc::c_uint |
                          ((arg1 & 0xff as libc::c_int) << 8 as libc::c_int)
                              as libc::c_uint |
                          (arg2 & 0xff as libc::c_int) as libc::c_uint,
                      1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800E64F8() {
    Audio_QueueCmdS32(0xfa000000 as libc::c_uint, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800E651C(mut arg0: u32_0, mut arg1: s32) {
    Audio_QueueCmdS32((arg1 & 0xff as libc::c_int) as libc::c_uint |
                          0xfd000000 as libc::c_uint, arg0 as s32);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_WaitForAudioTask() {
    osRecvMesg(gAudioContext.taskStartQueueP, 0 as *mut OSMesg,
               0 as libc::c_int);
    osRecvMesg(gAudioContext.taskStartQueueP, 0 as *mut OSMesg,
               1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800E6590(mut playerIdx: s32, mut arg1: s32,
                                       mut arg2: s32) -> s32 {
    let mut seqPlayer: *mut SequencePlayer = 0 as *mut SequencePlayer;
    let mut layer: *mut SequenceLayer = 0 as *mut SequenceLayer;
    let mut note: *mut Note = 0 as *mut Note;
    let mut sound: *mut SoundFontSound = 0 as *mut SoundFontSound;
    let mut loopEnd: s32 = 0;
    let mut samplePos: s32 = 0;
    seqPlayer =
        &mut *gAudioContext.seqPlayers.as_mut_ptr().offset(playerIdx as isize)
            as *mut SequencePlayer;
    if (*seqPlayer).enabled() as libc::c_int != 0 &&
           (*(*seqPlayer).channels[arg1 as usize]).enabled() as libc::c_int !=
               0 {
        layer = (*(*seqPlayer).channels[arg1 as usize]).layers[arg2 as usize];
        if layer.is_null() { return 0 as libc::c_int }
        if (*layer).enabled() != 0 {
            if (*layer).note.is_null() { return 0 as libc::c_int }
            if (*layer).bit3() == 0 { return 0 as libc::c_int }
            note = (*layer).note;
            if layer == (*note).playbackState.parentLayer {
                sound = (*note).noteSubEu.sound.soundFontSound;
                if sound.is_null() { return 0 as libc::c_int }
                loopEnd = (*(*(*sound).sample).loop_0).end as s32;
                samplePos = (*note).synthesisState.samplePosInt;
                return loopEnd - samplePos
            }
            return 0 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800E6680() -> s32 {
    return func_800E66C0(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800E66A0() { func_800E66C0(2 as libc::c_int); }
#[no_mangle]
pub unsafe extern "C" fn func_800E66C0(mut arg0: s32) -> s32 {
    let mut phi_v1: s32 = 0;
    let mut temp_a2: *mut NotePlaybackState = 0 as *mut NotePlaybackState;
    let mut temp_a3: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut i: s32 = 0;
    let mut note: *mut Note = 0 as *mut Note;
    let mut sound: *mut SoundFontSound = 0 as *mut SoundFontSound;
    phi_v1 = 0 as libc::c_int;
    let mut current_block_10: u64;
    i = 0 as libc::c_int;
    while i < gAudioContext.numNotes {
        note = &mut *gAudioContext.notes.offset(i as isize) as *mut Note;
        temp_a2 = &mut (*note).playbackState;
        if (*note).noteSubEu.bitField0.enabled() != 0 {
            temp_a3 = &mut (*note).noteSubEu;
            if (*temp_a2).adsr.action.s.state() as libc::c_int !=
                   0 as libc::c_int {
                if arg0 >= 2 as libc::c_int {
                    sound = (*temp_a3).sound.soundFontSound;
                    if sound.is_null() ||
                           (*temp_a3).bitField1.isSyntheticWave() as
                               libc::c_int != 0 {
                        current_block_10 = 7095457783677275021;
                    } else if (*(*sound).sample).medium() as libc::c_int ==
                                  MEDIUM_RAM as libc::c_int {
                        current_block_10 = 7095457783677275021;
                    } else { current_block_10 = 8831408221741692167; }
                } else { current_block_10 = 8831408221741692167; }
                match current_block_10 {
                    7095457783677275021 => { }
                    _ => {
                        phi_v1 += 1;
                        if arg0 & 1 as libc::c_int == 1 as libc::c_int {
                            (*temp_a2).adsr.fadeOutVel =
                                gAudioContext.audioBufferParameters.updatesPerFrameInv;
                            (*temp_a2).adsr.action.s.set_release(1 as
                                                                     libc::c_int
                                                                     as u8_0)
                        }
                    }
                }
            }
        }
        i += 1
    }
    return phi_v1;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NextRandom() -> u32_0 {
    static mut audRand: u32_0 = 0x12345678 as libc::c_int as u32_0;
    audRand =
        osGetCount().wrapping_add(0x1234567 as libc::c_int as
                                      libc::c_uint).wrapping_mul(audRand.wrapping_add(gAudioContext.totalTaskCnt
                                                                                          as
                                                                                          libc::c_uint));
    audRand =
        (audRand as libc::c_uint).wrapping_add(gAudioContext.audioRandom) as
            u32_0 as u32_0;
    return audRand;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitMesgQueues() {
    Audio_InitMesgQueuesInternal();
}
