#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn Yaz0_Decompress(romStart: u32_0, dst: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osDriveRomInit() -> *mut OSPiHandle;
    #[no_mangle]
    fn StackCheck_Init(entry: *mut StackEntry, stackTop: *mut libc::c_void,
                       stackBottom: *mut libc::c_void, initValue: u32_0,
                       minSpace: s32, name: *const libc::c_char);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn sprintf(dst: *mut libc::c_char, fmt: *const libc::c_char, _: ...)
     -> s32;
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn osEPiStartDma(handle: *mut OSPiHandle, mb: *mut OSIoMesg,
                     direction: s32) -> s32;
    #[no_mangle]
    fn osInvalICache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osInvalDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osSetThreadPri(thread: *mut OSThread, pri: OSPri);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn Fault_AddHungupAndCrashImpl(_: *const libc::c_char,
                                   _: *const libc::c_char);
    #[no_mangle]
    fn Fault_AddHungupAndCrash(_: *const libc::c_char, _: u32_0);
    #[no_mangle]
    static mut _dmadataSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _dmadataSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _dmadataSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _bootSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut osMemSize: u32_0;
    #[no_mangle]
    static mut gDmaDataTable: [DmaEntry; 1548];
    #[no_mangle]
    static mut gCartHandle: *mut OSPiHandle;
    #[no_mangle]
    static mut gPiMgrCmdQ: OSMesgQueue;
}
pub type u8_0 = libc::c_uchar;
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
pub type OSTime = u64_0;
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
pub struct DmaRequest {
    pub vromAddr: u32_0,
    pub dramAddr: *mut libc::c_void,
    pub size: u32_0,
    pub filename: *const libc::c_char,
    pub line: s32,
    pub unk_14: s32,
    pub notifyQueue: *mut OSMesgQueue,
    pub notifyMsg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DmaEntry {
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
    pub romStart: u32_0,
    pub romEnd: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackEntry {
    pub next: *mut StackEntry,
    pub prev: *mut StackEntry,
    pub head: u32_0,
    pub tail: u32_0,
    pub initValue: u32_0,
    pub minSpace: s32,
    pub name: *const libc::c_char,
}
#[no_mangle]
pub static mut sDmaMgrStackInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sDmaMgrMsgQueue: OSMesgQueue =
    OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                fullqueue: 0 as *const OSThread as *mut OSThread,
                validCount: 0,
                first: 0,
                msgCount: 0,
                msg: 0 as *const OSMesg as *mut OSMesg,};
#[no_mangle]
pub static mut sDmaMgrMsgs: [OSMesg; 32] =
    [0 as *const libc::c_void as *mut libc::c_void; 32];
#[no_mangle]
pub static mut sDmaMgrThread: OSThread =
    OSThread{next: 0 as *const OSThread as *mut OSThread,
             priority: 0,
             queue: 0 as *const *mut OSThread as *mut *mut OSThread,
             tlnext: 0 as *const OSThread as *mut OSThread,
             state: 0,
             flags: 0,
             id: 0,
             fp: 0,
             thprof: 0 as *const __OSThreadprofile as *mut __OSThreadprofile,
             context:
                 __OSThreadContext{at: 0,
                                   v0: 0,
                                   v1: 0,
                                   a0: 0,
                                   a1: 0,
                                   a2: 0,
                                   a3: 0,
                                   t0: 0,
                                   t1: 0,
                                   t2: 0,
                                   t3: 0,
                                   t4: 0,
                                   t5: 0,
                                   t6: 0,
                                   t7: 0,
                                   s0: 0,
                                   s1: 0,
                                   s2: 0,
                                   s3: 0,
                                   s4: 0,
                                   s5: 0,
                                   s6: 0,
                                   s7: 0,
                                   t8: 0,
                                   t9: 0,
                                   gp: 0,
                                   sp: 0,
                                   s8: 0,
                                   ra: 0,
                                   lo: 0,
                                   hi: 0,
                                   sr: 0,
                                   pc: 0,
                                   cause: 0,
                                   badvaddr: 0,
                                   rcp: 0,
                                   fpcsr: 0,
                                   fp0:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp2:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp4:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp6:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp8:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp10:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp12:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp14:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp16:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp18:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp20:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp22:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp24:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp26:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp28:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp30:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},},};
#[no_mangle]
pub static mut sDmaMgrStack: [u8_0; 1280] = [0; 1280];
#[no_mangle]
pub static mut sDmaMgrCurFileName: *const libc::c_char =
    0 as *const libc::c_char;
#[no_mangle]
pub static mut sDmaMgrCurFileLine: s32 = 0;
#[no_mangle]
pub static mut D_80009460: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut gDmaMgrDmaBuffSize: u32_0 = 0x2000 as libc::c_int as u32_0;
#[no_mangle]
pub static mut sDmaMgrDataExistError: u32_0 = 0 as libc::c_int as u32_0;
// dmadata filenames
#[no_mangle]
pub static mut sDmaMgrFileNames: [*const libc::c_char; 1533] =
    [b"makerom\x00" as *const u8 as *const libc::c_char,
     b"boot\x00" as *const u8 as *const libc::c_char,
     b"dmadata\x00" as *const u8 as *const libc::c_char,
     b"Audiobank\x00" as *const u8 as *const libc::c_char,
     b"Audioseq\x00" as *const u8 as *const libc::c_char,
     b"Audiotable\x00" as *const u8 as *const libc::c_char,
     b"link_animetion\x00" as *const u8 as *const libc::c_char,
     b"icon_item_static\x00" as *const u8 as *const libc::c_char,
     b"icon_item_24_static\x00" as *const u8 as *const libc::c_char,
     b"icon_item_field_static\x00" as *const u8 as *const libc::c_char,
     b"icon_item_dungeon_static\x00" as *const u8 as *const libc::c_char,
     b"icon_item_gameover_static\x00" as *const u8 as *const libc::c_char,
     b"icon_item_nes_static\x00" as *const u8 as *const libc::c_char,
     b"icon_item_ger_static\x00" as *const u8 as *const libc::c_char,
     b"icon_item_fra_static\x00" as *const u8 as *const libc::c_char,
     b"item_name_static\x00" as *const u8 as *const libc::c_char,
     b"map_name_static\x00" as *const u8 as *const libc::c_char,
     b"do_action_static\x00" as *const u8 as *const libc::c_char,
     b"message_static\x00" as *const u8 as *const libc::c_char,
     b"message_texture_static\x00" as *const u8 as *const libc::c_char,
     b"nes_font_static\x00" as *const u8 as *const libc::c_char,
     b"nes_message_data_static\x00" as *const u8 as *const libc::c_char,
     b"ger_message_data_static\x00" as *const u8 as *const libc::c_char,
     b"fra_message_data_static\x00" as *const u8 as *const libc::c_char,
     b"staff_message_data_static\x00" as *const u8 as *const libc::c_char,
     b"map_grand_static\x00" as *const u8 as *const libc::c_char,
     b"map_48x85_static\x00" as *const u8 as *const libc::c_char,
     b"map_i_static\x00" as *const u8 as *const libc::c_char,
     b"code\x00" as *const u8 as *const libc::c_char,
     b"buffers\x00" as *const u8 as *const libc::c_char,
     b"ovl_title\x00" as *const u8 as *const libc::c_char,
     b"ovl_select\x00" as *const u8 as *const libc::c_char,
     b"ovl_opening\x00" as *const u8 as *const libc::c_char,
     b"ovl_file_choose\x00" as *const u8 as *const libc::c_char,
     b"ovl_kaleido_scope\x00" as *const u8 as *const libc::c_char,
     b"ovl_player_actor\x00" as *const u8 as *const libc::c_char,
     b"ovl_map_mark_data\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Test\x00" as *const u8 as *const libc::c_char,
     b"ovl_Arms_Hook\x00" as *const u8 as *const libc::c_char,
     b"ovl_Arrow_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_Arrow_Ice\x00" as *const u8 as *const libc::c_char,
     b"ovl_Arrow_Light\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Bdan_Objects\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Bdan_Switch\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Bom_Guard\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Bombwall\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Bowl_Wall\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Breakwall\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ddan_Jd\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ddan_Kd\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Dodoago\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Dy_Yoseizo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ganon_Otyuka\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Gate_Shutter\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Gjyo_Bridge\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Gnd_Darkmeiro\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Gnd_Firemeiro\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Gnd_Iceblock\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Gnd_Nisekabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Gnd_Soulmeiro\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Gate\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Huta\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Megane\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_MeganeBG\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Sgami\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Ship\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Trap\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Tubo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Water\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Haka_Zou\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Heavy_Block\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Curtain\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Dalm\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Firewall\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Fslift\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Fwbig\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Hamstep\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Hrock\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Kousi\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Kowarerukabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Rock\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Rsekizou\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Sekizou\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Sima\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Hidan_Syoku\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ice_Objects\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ice_Shelter\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ice_Shutter\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ice_Turara\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ingate\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_1flift\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Amishutter\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Bigmirror\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Block\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Bombchuiwa\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Bombiwa\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Cobra\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Goroiwa\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Haheniron\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Ironobj\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Kanaami\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Lift\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Megami\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Jya_Zurerukabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Menkuri_Eye\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Menkuri_Kaiten\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Menkuri_Nisekabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mizu_Bwall\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mizu_Movebg\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mizu_Shutter\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mizu_Uzu\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mizu_Water\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mjin\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Bigst\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Elevator\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Hashigo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Hashira4\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Hineri\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Idomizu\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Kaitenkabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Mori_Rakkatenjo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Po_Event\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Po_Syokudai\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Pushbox\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Relay_Objects\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot00_Break\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot00_Hanebasi\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot01_Fusya\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot01_Idohashira\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot01_Idomizu\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot01_Idosoko\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot01_Objects2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot02_Objects\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot03_Taki\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot05_Soko\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot06_Objects\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot07_Taki\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot08_Bakudankabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot08_Iceblock\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot09_Obj\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot11_Bakudankabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot11_Oasis\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot12_Gate\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot12_Saku\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot15_Rrbox\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot15_Saku\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot16_Bombstone\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot16_Doughnut\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot17_Bakudankabe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot17_Funen\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot18_Basket\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot18_Futa\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot18_Obj\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Spot18_Shutter\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Sst_Floor\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Toki_Hikari\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Toki_Swd\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Treemouth\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Umajump\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Vb_Sima\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ydan_Hasi\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ydan_Maruta\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Ydan_Sp\x00" as *const u8 as *const libc::c_char,
     b"ovl_Bg_Zg\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Dodongo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Fd\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Fd2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Ganon\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Ganon2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Ganondrof\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Goma\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Mo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Sst\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Tw\x00" as *const u8 as *const libc::c_char,
     b"ovl_Boss_Va\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_6K\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Du\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Ec\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Effect\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Ext\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Geff\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Gj\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Go\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Gt\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Ik\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Im\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Kankyo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Kekkai\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Sa\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Shd\x00" as *const u8 as *const libc::c_char,
     b"ovl_Demo_Tre_Lgt\x00" as *const u8 as *const libc::c_char,
     b"ovl_Door_Ana\x00" as *const u8 as *const libc::c_char,
     b"ovl_Door_Gerudo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Door_Killer\x00" as *const u8 as *const libc::c_char,
     b"ovl_Door_Shutter\x00" as *const u8 as *const libc::c_char,
     b"ovl_Door_Toki\x00" as *const u8 as *const libc::c_char,
     b"ovl_Door_Warp1\x00" as *const u8 as *const libc::c_char,
     b"ovl_Efc_Erupc\x00" as *const u8 as *const libc::c_char,
     b"ovl_Eff_Dust\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Blast\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Bomb\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Bomb2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Bubble\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_D_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Dead_Db\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Dead_Dd\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Dead_Ds\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Dead_Sound\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Dt_Bubble\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Dust\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_En_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_En_Ice\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Extra\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Fcircle\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Fhg_Flash\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Fire_Tail\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_G_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_G_Magma\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_G_Magma2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_G_Ripple\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_G_Spk\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_G_Splash\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Hahen\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_HitMark\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Ice_Piece\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Ice_Smoke\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_K_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Kakera\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_KiraKira\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Lightning\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Sibuki\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Sibuki2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Solder_Srch_Ball\x00" as *const u8 as
         *const libc::c_char,
     b"ovl_Effect_Ss_Stick\x00" as *const u8 as *const libc::c_char,
     b"ovl_Effect_Ss_Stone1\x00" as *const u8 as *const libc::c_char,
     b"ovl_Elf_Msg\x00" as *const u8 as *const libc::c_char,
     b"ovl_Elf_Msg2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Am\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ani\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Anubice\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Anubice_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Anubice_Tag\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Arow_Trap\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Arrow\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Attack_Niw\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ba\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bb\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bdfire\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bigokuta\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bili\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bird\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Blkobj\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bom\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bom_Bowl_Man\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bom_Bowl_Pit\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bom_Chu\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bombf\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Boom\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Box\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Brob\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bubble\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Butte\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bw\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Bx\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Changer\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Clear_Tag\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Cow\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Crow\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Cs\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Daiku\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Daiku_Kakariko\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dekubaba\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dekunuts\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dh\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dha\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Diving_Game\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dns\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dnt_Demo\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dnt_Jiji\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dnt_Nomal\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dodojr\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dodongo\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dog\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Door\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ds\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Du\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Dy_Extra\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Eg\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Eiyer\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Elf\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Encount1\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Encount2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ex_Item\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ex_Ruppy\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fd\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fd_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fhg_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fire_Rock\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Firefly\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fish\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Floormas\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fr\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fu\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fw\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Fz\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_G_Switch\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ganon_Mant\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ganon_Organ\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Gb\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ge1\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ge2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ge3\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_GeldB\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_GirlA\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Gm\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Go\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Go2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Goma\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Goroiwa\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Gs\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Guest\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Hata\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Heishi1\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Heishi2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Heishi3\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Heishi4\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Hintnuts\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Holl\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Honotrap\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Horse\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Horse_Game_Check\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Horse_Ganon\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Horse_Link_Child\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Horse_Normal\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Horse_Zelda\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Hs\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Hs2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Hy\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ice_Hono\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ik\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_In\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Insect\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ishi\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_It\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Jj\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Js\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Jsjutan\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Kakasi\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Kakasi2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Kakasi3\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Kanban\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Karebaba\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ko\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Kusa\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Kz\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Light\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Lightbox\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_M_Fire1\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_M_Thunder\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ma1\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ma2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ma3\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Mag\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Mb\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Md\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Mk\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Mm\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Mm2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ms\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Mu\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Nb\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Niw\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Niw_Girl\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Niw_Lady\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Nutsball\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Nwc\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ny\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_OE2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Okarina_Effect\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Okarina_Tag\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Okuta\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ossan\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Owl\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Part\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Peehat\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Po_Desert\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Po_Field\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Po_Relay\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Po_Sisters\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Poh\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Pu_box\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Rd\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Reeba\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_River_Sound\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Rl\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Rr\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ru1\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ru2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Sa\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Sb\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Scene_Change\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Sda\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Shopnuts\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Si\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Siofuki\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Skb\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Skj\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Skjneedle\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ssh\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_St\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Sth\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Stream\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Sw\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Syateki_Itm\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Syateki_Man\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Syateki_Niw\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Ta\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Takara_Man\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Tana\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Tg\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Tite\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Tk\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Torch\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Torch2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Toryo\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Tp\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Tr\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Trap\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Tubo_Trap\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Vali\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Vase\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Vb_Ball\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Viewer\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Vm\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Wall_Tubo\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Wallmas\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Weather_Tag\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Weiyer\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Wf\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Wonder_Item\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Wonder_Talk\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Wonder_Talk2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Wood02\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Xc\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Yabusame_Mark\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Yukabyun\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Zf\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Zl1\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Zl2\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Zl3\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Zl4\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_Zo\x00" as *const u8 as *const libc::c_char,
     b"ovl_En_fHG\x00" as *const u8 as *const libc::c_char,
     b"ovl_End_Title\x00" as *const u8 as *const libc::c_char,
     b"ovl_Fishing\x00" as *const u8 as *const libc::c_char,
     b"ovl_Item_B_Heart\x00" as *const u8 as *const libc::c_char,
     b"ovl_Item_Etcetera\x00" as *const u8 as *const libc::c_char,
     b"ovl_Item_Inbox\x00" as *const u8 as *const libc::c_char,
     b"ovl_Item_Ocarina\x00" as *const u8 as *const libc::c_char,
     b"ovl_Item_Shield\x00" as *const u8 as *const libc::c_char,
     b"ovl_Magic_Dark\x00" as *const u8 as *const libc::c_char,
     b"ovl_Magic_Fire\x00" as *const u8 as *const libc::c_char,
     b"ovl_Magic_Wind\x00" as *const u8 as *const libc::c_char,
     b"ovl_Mir_Ray\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Bean\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Blockstop\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Bombiwa\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Comb\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Dekujr\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Elevator\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Hamishi\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Hana\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Hsblock\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Ice_Poly\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Kibako\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Kibako2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Lift\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Lightswitch\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Makekinsuta\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Makeoshihiki\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Mure\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Mure2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Mure3\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Oshihiki\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Roomtimer\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Switch\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Syokudai\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Timeblock\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Tsubo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Obj_Warp2block\x00" as *const u8 as *const libc::c_char,
     b"ovl_Object_Kankyo\x00" as *const u8 as *const libc::c_char,
     b"ovl_Oceff_Spot\x00" as *const u8 as *const libc::c_char,
     b"ovl_Oceff_Storm\x00" as *const u8 as *const libc::c_char,
     b"ovl_Oceff_Wipe\x00" as *const u8 as *const libc::c_char,
     b"ovl_Oceff_Wipe2\x00" as *const u8 as *const libc::c_char,
     b"ovl_Oceff_Wipe3\x00" as *const u8 as *const libc::c_char,
     b"ovl_Oceff_Wipe4\x00" as *const u8 as *const libc::c_char,
     b"ovl_Shot_Sun\x00" as *const u8 as *const libc::c_char,
     b"gameplay_keep\x00" as *const u8 as *const libc::c_char,
     b"gameplay_field_keep\x00" as *const u8 as *const libc::c_char,
     b"gameplay_dangeon_keep\x00" as *const u8 as *const libc::c_char,
     b"gameplay_object_exchange_static\x00" as *const u8 as
         *const libc::c_char,
     b"object_link_boy\x00" as *const u8 as *const libc::c_char,
     b"object_link_child\x00" as *const u8 as *const libc::c_char,
     b"object_box\x00" as *const u8 as *const libc::c_char,
     b"object_human\x00" as *const u8 as *const libc::c_char,
     b"object_okuta\x00" as *const u8 as *const libc::c_char,
     b"object_poh\x00" as *const u8 as *const libc::c_char,
     b"object_wallmaster\x00" as *const u8 as *const libc::c_char,
     b"object_dy_obj\x00" as *const u8 as *const libc::c_char,
     b"object_firefly\x00" as *const u8 as *const libc::c_char,
     b"object_dodongo\x00" as *const u8 as *const libc::c_char,
     b"object_fire\x00" as *const u8 as *const libc::c_char,
     b"object_niw\x00" as *const u8 as *const libc::c_char,
     b"object_tite\x00" as *const u8 as *const libc::c_char,
     b"object_reeba\x00" as *const u8 as *const libc::c_char,
     b"object_peehat\x00" as *const u8 as *const libc::c_char,
     b"object_kingdodongo\x00" as *const u8 as *const libc::c_char,
     b"object_horse\x00" as *const u8 as *const libc::c_char,
     b"object_zf\x00" as *const u8 as *const libc::c_char,
     b"object_goma\x00" as *const u8 as *const libc::c_char,
     b"object_zl1\x00" as *const u8 as *const libc::c_char,
     b"object_gol\x00" as *const u8 as *const libc::c_char,
     b"object_bubble\x00" as *const u8 as *const libc::c_char,
     b"object_dodojr\x00" as *const u8 as *const libc::c_char,
     b"object_torch2\x00" as *const u8 as *const libc::c_char,
     b"object_bl\x00" as *const u8 as *const libc::c_char,
     b"object_tp\x00" as *const u8 as *const libc::c_char,
     b"object_oA1\x00" as *const u8 as *const libc::c_char,
     b"object_st\x00" as *const u8 as *const libc::c_char,
     b"object_bw\x00" as *const u8 as *const libc::c_char,
     b"object_ei\x00" as *const u8 as *const libc::c_char,
     b"object_horse_normal\x00" as *const u8 as *const libc::c_char,
     b"object_oB1\x00" as *const u8 as *const libc::c_char,
     b"object_o_anime\x00" as *const u8 as *const libc::c_char,
     b"object_spot04_objects\x00" as *const u8 as *const libc::c_char,
     b"object_ddan_objects\x00" as *const u8 as *const libc::c_char,
     b"object_hidan_objects\x00" as *const u8 as *const libc::c_char,
     b"object_horse_ganon\x00" as *const u8 as *const libc::c_char,
     b"object_oA2\x00" as *const u8 as *const libc::c_char,
     b"object_spot00_objects\x00" as *const u8 as *const libc::c_char,
     b"object_mb\x00" as *const u8 as *const libc::c_char,
     b"object_bombf\x00" as *const u8 as *const libc::c_char,
     b"object_sk2\x00" as *const u8 as *const libc::c_char,
     b"object_oE1\x00" as *const u8 as *const libc::c_char,
     b"object_oE_anime\x00" as *const u8 as *const libc::c_char,
     b"object_oE2\x00" as *const u8 as *const libc::c_char,
     b"object_ydan_objects\x00" as *const u8 as *const libc::c_char,
     b"object_gnd\x00" as *const u8 as *const libc::c_char,
     b"object_am\x00" as *const u8 as *const libc::c_char,
     b"object_dekubaba\x00" as *const u8 as *const libc::c_char,
     b"object_oA3\x00" as *const u8 as *const libc::c_char,
     b"object_oA4\x00" as *const u8 as *const libc::c_char,
     b"object_oA5\x00" as *const u8 as *const libc::c_char,
     b"object_oA6\x00" as *const u8 as *const libc::c_char,
     b"object_oA7\x00" as *const u8 as *const libc::c_char,
     b"object_jj\x00" as *const u8 as *const libc::c_char,
     b"object_oA8\x00" as *const u8 as *const libc::c_char,
     b"object_oA9\x00" as *const u8 as *const libc::c_char,
     b"object_oB2\x00" as *const u8 as *const libc::c_char,
     b"object_oB3\x00" as *const u8 as *const libc::c_char,
     b"object_oB4\x00" as *const u8 as *const libc::c_char,
     b"object_horse_zelda\x00" as *const u8 as *const libc::c_char,
     b"object_opening_demo1\x00" as *const u8 as *const libc::c_char,
     b"object_warp1\x00" as *const u8 as *const libc::c_char,
     b"object_b_heart\x00" as *const u8 as *const libc::c_char,
     b"object_dekunuts\x00" as *const u8 as *const libc::c_char,
     b"object_oE3\x00" as *const u8 as *const libc::c_char,
     b"object_oE4\x00" as *const u8 as *const libc::c_char,
     b"object_menkuri_objects\x00" as *const u8 as *const libc::c_char,
     b"object_oE5\x00" as *const u8 as *const libc::c_char,
     b"object_oE6\x00" as *const u8 as *const libc::c_char,
     b"object_oE7\x00" as *const u8 as *const libc::c_char,
     b"object_oE8\x00" as *const u8 as *const libc::c_char,
     b"object_oE9\x00" as *const u8 as *const libc::c_char,
     b"object_oE10\x00" as *const u8 as *const libc::c_char,
     b"object_oE11\x00" as *const u8 as *const libc::c_char,
     b"object_oE12\x00" as *const u8 as *const libc::c_char,
     b"object_vali\x00" as *const u8 as *const libc::c_char,
     b"object_oA10\x00" as *const u8 as *const libc::c_char,
     b"object_oA11\x00" as *const u8 as *const libc::c_char,
     b"object_mizu_objects\x00" as *const u8 as *const libc::c_char,
     b"object_fhg\x00" as *const u8 as *const libc::c_char,
     b"object_ossan\x00" as *const u8 as *const libc::c_char,
     b"object_mori_hineri1\x00" as *const u8 as *const libc::c_char,
     b"object_Bb\x00" as *const u8 as *const libc::c_char,
     b"object_toki_objects\x00" as *const u8 as *const libc::c_char,
     b"object_yukabyun\x00" as *const u8 as *const libc::c_char,
     b"object_zl2\x00" as *const u8 as *const libc::c_char,
     b"object_mjin\x00" as *const u8 as *const libc::c_char,
     b"object_mjin_flash\x00" as *const u8 as *const libc::c_char,
     b"object_mjin_dark\x00" as *const u8 as *const libc::c_char,
     b"object_mjin_flame\x00" as *const u8 as *const libc::c_char,
     b"object_mjin_ice\x00" as *const u8 as *const libc::c_char,
     b"object_mjin_soul\x00" as *const u8 as *const libc::c_char,
     b"object_mjin_wind\x00" as *const u8 as *const libc::c_char,
     b"object_mjin_oka\x00" as *const u8 as *const libc::c_char,
     b"object_haka_objects\x00" as *const u8 as *const libc::c_char,
     b"object_spot06_objects\x00" as *const u8 as *const libc::c_char,
     b"object_ice_objects\x00" as *const u8 as *const libc::c_char,
     b"object_relay_objects\x00" as *const u8 as *const libc::c_char,
     b"object_mori_hineri1a\x00" as *const u8 as *const libc::c_char,
     b"object_mori_hineri2\x00" as *const u8 as *const libc::c_char,
     b"object_mori_hineri2a\x00" as *const u8 as *const libc::c_char,
     b"object_mori_objects\x00" as *const u8 as *const libc::c_char,
     b"object_mori_tex\x00" as *const u8 as *const libc::c_char,
     b"object_spot08_obj\x00" as *const u8 as *const libc::c_char,
     b"object_warp2\x00" as *const u8 as *const libc::c_char,
     b"object_hata\x00" as *const u8 as *const libc::c_char,
     b"object_bird\x00" as *const u8 as *const libc::c_char,
     b"object_wood02\x00" as *const u8 as *const libc::c_char,
     b"object_lightbox\x00" as *const u8 as *const libc::c_char,
     b"object_pu_box\x00" as *const u8 as *const libc::c_char,
     b"object_trap\x00" as *const u8 as *const libc::c_char,
     b"object_vase\x00" as *const u8 as *const libc::c_char,
     b"object_im\x00" as *const u8 as *const libc::c_char,
     b"object_ta\x00" as *const u8 as *const libc::c_char,
     b"object_tk\x00" as *const u8 as *const libc::c_char,
     b"object_xc\x00" as *const u8 as *const libc::c_char,
     b"object_vm\x00" as *const u8 as *const libc::c_char,
     b"object_bv\x00" as *const u8 as *const libc::c_char,
     b"object_hakach_objects\x00" as *const u8 as *const libc::c_char,
     b"object_efc_crystal_light\x00" as *const u8 as *const libc::c_char,
     b"object_efc_fire_ball\x00" as *const u8 as *const libc::c_char,
     b"object_efc_flash\x00" as *const u8 as *const libc::c_char,
     b"object_efc_lgt_shower\x00" as *const u8 as *const libc::c_char,
     b"object_efc_star_field\x00" as *const u8 as *const libc::c_char,
     b"object_god_lgt\x00" as *const u8 as *const libc::c_char,
     b"object_light_ring\x00" as *const u8 as *const libc::c_char,
     b"object_triforce_spot\x00" as *const u8 as *const libc::c_char,
     b"object_medal\x00" as *const u8 as *const libc::c_char,
     b"object_bdan_objects\x00" as *const u8 as *const libc::c_char,
     b"object_sd\x00" as *const u8 as *const libc::c_char,
     b"object_rd\x00" as *const u8 as *const libc::c_char,
     b"object_po_sisters\x00" as *const u8 as *const libc::c_char,
     b"object_heavy_object\x00" as *const u8 as *const libc::c_char,
     b"object_gndd\x00" as *const u8 as *const libc::c_char,
     b"object_fd\x00" as *const u8 as *const libc::c_char,
     b"object_du\x00" as *const u8 as *const libc::c_char,
     b"object_fw\x00" as *const u8 as *const libc::c_char,
     b"object_horse_link_child\x00" as *const u8 as *const libc::c_char,
     b"object_spot02_objects\x00" as *const u8 as *const libc::c_char,
     b"object_haka\x00" as *const u8 as *const libc::c_char,
     b"object_ru1\x00" as *const u8 as *const libc::c_char,
     b"object_syokudai\x00" as *const u8 as *const libc::c_char,
     b"object_fd2\x00" as *const u8 as *const libc::c_char,
     b"object_dh\x00" as *const u8 as *const libc::c_char,
     b"object_rl\x00" as *const u8 as *const libc::c_char,
     b"object_efc_tw\x00" as *const u8 as *const libc::c_char,
     b"object_demo_tre_lgt\x00" as *const u8 as *const libc::c_char,
     b"object_gi_key\x00" as *const u8 as *const libc::c_char,
     b"object_mir_ray\x00" as *const u8 as *const libc::c_char,
     b"object_brob\x00" as *const u8 as *const libc::c_char,
     b"object_gi_jewel\x00" as *const u8 as *const libc::c_char,
     b"object_spot09_obj\x00" as *const u8 as *const libc::c_char,
     b"object_spot18_obj\x00" as *const u8 as *const libc::c_char,
     b"object_bdoor\x00" as *const u8 as *const libc::c_char,
     b"object_spot17_obj\x00" as *const u8 as *const libc::c_char,
     b"object_shop_dungen\x00" as *const u8 as *const libc::c_char,
     b"object_nb\x00" as *const u8 as *const libc::c_char,
     b"object_mo\x00" as *const u8 as *const libc::c_char,
     b"object_sb\x00" as *const u8 as *const libc::c_char,
     b"object_gi_melody\x00" as *const u8 as *const libc::c_char,
     b"object_gi_heart\x00" as *const u8 as *const libc::c_char,
     b"object_gi_compass\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bosskey\x00" as *const u8 as *const libc::c_char,
     b"object_gi_medal\x00" as *const u8 as *const libc::c_char,
     b"object_gi_nuts\x00" as *const u8 as *const libc::c_char,
     b"object_sa\x00" as *const u8 as *const libc::c_char,
     b"object_gi_hearts\x00" as *const u8 as *const libc::c_char,
     b"object_gi_arrowcase\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bombpouch\x00" as *const u8 as *const libc::c_char,
     b"object_in\x00" as *const u8 as *const libc::c_char,
     b"object_tr\x00" as *const u8 as *const libc::c_char,
     b"object_spot16_obj\x00" as *const u8 as *const libc::c_char,
     b"object_oE1s\x00" as *const u8 as *const libc::c_char,
     b"object_oE4s\x00" as *const u8 as *const libc::c_char,
     b"object_os_anime\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bottle\x00" as *const u8 as *const libc::c_char,
     b"object_gi_stick\x00" as *const u8 as *const libc::c_char,
     b"object_gi_map\x00" as *const u8 as *const libc::c_char,
     b"object_oF1d_map\x00" as *const u8 as *const libc::c_char,
     b"object_ru2\x00" as *const u8 as *const libc::c_char,
     b"object_gi_shield_1\x00" as *const u8 as *const libc::c_char,
     b"object_dekujr\x00" as *const u8 as *const libc::c_char,
     b"object_gi_magicpot\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bomb_1\x00" as *const u8 as *const libc::c_char,
     b"object_oF1s\x00" as *const u8 as *const libc::c_char,
     b"object_ma2\x00" as *const u8 as *const libc::c_char,
     b"object_gi_purse\x00" as *const u8 as *const libc::c_char,
     b"object_hni\x00" as *const u8 as *const libc::c_char,
     b"object_tw\x00" as *const u8 as *const libc::c_char,
     b"object_rr\x00" as *const u8 as *const libc::c_char,
     b"object_bxa\x00" as *const u8 as *const libc::c_char,
     b"object_anubice\x00" as *const u8 as *const libc::c_char,
     b"object_gi_gerudo\x00" as *const u8 as *const libc::c_char,
     b"object_gi_arrow\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bomb_2\x00" as *const u8 as *const libc::c_char,
     b"object_gi_egg\x00" as *const u8 as *const libc::c_char,
     b"object_gi_scale\x00" as *const u8 as *const libc::c_char,
     b"object_gi_shield_2\x00" as *const u8 as *const libc::c_char,
     b"object_gi_hookshot\x00" as *const u8 as *const libc::c_char,
     b"object_gi_ocarina\x00" as *const u8 as *const libc::c_char,
     b"object_gi_milk\x00" as *const u8 as *const libc::c_char,
     b"object_ma1\x00" as *const u8 as *const libc::c_char,
     b"object_ganon\x00" as *const u8 as *const libc::c_char,
     b"object_sst\x00" as *const u8 as *const libc::c_char,
     b"object_ny\x00" as *const u8 as *const libc::c_char,
     b"object_fr\x00" as *const u8 as *const libc::c_char,
     b"object_gi_pachinko\x00" as *const u8 as *const libc::c_char,
     b"object_gi_boomerang\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bow\x00" as *const u8 as *const libc::c_char,
     b"object_gi_glasses\x00" as *const u8 as *const libc::c_char,
     b"object_gi_liquid\x00" as *const u8 as *const libc::c_char,
     b"object_ani\x00" as *const u8 as *const libc::c_char,
     b"object_demo_6k\x00" as *const u8 as *const libc::c_char,
     b"object_gi_shield_3\x00" as *const u8 as *const libc::c_char,
     b"object_gi_letter\x00" as *const u8 as *const libc::c_char,
     b"object_spot15_obj\x00" as *const u8 as *const libc::c_char,
     b"object_jya_obj\x00" as *const u8 as *const libc::c_char,
     b"object_gi_clothes\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bean\x00" as *const u8 as *const libc::c_char,
     b"object_gi_fish\x00" as *const u8 as *const libc::c_char,
     b"object_gi_saw\x00" as *const u8 as *const libc::c_char,
     b"object_gi_hammer\x00" as *const u8 as *const libc::c_char,
     b"object_gi_grass\x00" as *const u8 as *const libc::c_char,
     b"object_gi_longsword\x00" as *const u8 as *const libc::c_char,
     b"object_spot01_objects\x00" as *const u8 as *const libc::c_char,
     b"object_md\x00" as *const u8 as *const libc::c_char,
     b"object_km1\x00" as *const u8 as *const libc::c_char,
     b"object_kw1\x00" as *const u8 as *const libc::c_char,
     b"object_zo\x00" as *const u8 as *const libc::c_char,
     b"object_kz\x00" as *const u8 as *const libc::c_char,
     b"object_umajump\x00" as *const u8 as *const libc::c_char,
     b"object_masterkokiri\x00" as *const u8 as *const libc::c_char,
     b"object_masterkokirihead\x00" as *const u8 as *const libc::c_char,
     b"object_mastergolon\x00" as *const u8 as *const libc::c_char,
     b"object_masterzoora\x00" as *const u8 as *const libc::c_char,
     b"object_aob\x00" as *const u8 as *const libc::c_char,
     b"object_ik\x00" as *const u8 as *const libc::c_char,
     b"object_ahg\x00" as *const u8 as *const libc::c_char,
     b"object_cne\x00" as *const u8 as *const libc::c_char,
     b"object_gi_niwatori\x00" as *const u8 as *const libc::c_char,
     b"object_skj\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bottle_letter\x00" as *const u8 as *const libc::c_char,
     b"object_bji\x00" as *const u8 as *const libc::c_char,
     b"object_bba\x00" as *const u8 as *const libc::c_char,
     b"object_gi_ocarina_0\x00" as *const u8 as *const libc::c_char,
     b"object_ds\x00" as *const u8 as *const libc::c_char,
     b"object_ane\x00" as *const u8 as *const libc::c_char,
     b"object_boj\x00" as *const u8 as *const libc::c_char,
     b"object_spot03_object\x00" as *const u8 as *const libc::c_char,
     b"object_spot07_object\x00" as *const u8 as *const libc::c_char,
     b"object_fz\x00" as *const u8 as *const libc::c_char,
     b"object_bob\x00" as *const u8 as *const libc::c_char,
     b"object_ge1\x00" as *const u8 as *const libc::c_char,
     b"object_yabusame_point\x00" as *const u8 as *const libc::c_char,
     b"object_gi_boots_2\x00" as *const u8 as *const libc::c_char,
     b"object_gi_seed\x00" as *const u8 as *const libc::c_char,
     b"object_gnd_magic\x00" as *const u8 as *const libc::c_char,
     b"object_d_elevator\x00" as *const u8 as *const libc::c_char,
     b"object_d_hsblock\x00" as *const u8 as *const libc::c_char,
     b"object_d_lift\x00" as *const u8 as *const libc::c_char,
     b"object_mamenoki\x00" as *const u8 as *const libc::c_char,
     b"object_goroiwa\x00" as *const u8 as *const libc::c_char,
     b"object_toryo\x00" as *const u8 as *const libc::c_char,
     b"object_daiku\x00" as *const u8 as *const libc::c_char,
     b"object_nwc\x00" as *const u8 as *const libc::c_char,
     b"object_blkobj\x00" as *const u8 as *const libc::c_char,
     b"object_gm\x00" as *const u8 as *const libc::c_char,
     b"object_ms\x00" as *const u8 as *const libc::c_char,
     b"object_hs\x00" as *const u8 as *const libc::c_char,
     b"object_ingate\x00" as *const u8 as *const libc::c_char,
     b"object_lightswitch\x00" as *const u8 as *const libc::c_char,
     b"object_kusa\x00" as *const u8 as *const libc::c_char,
     b"object_tsubo\x00" as *const u8 as *const libc::c_char,
     b"object_gi_gloves\x00" as *const u8 as *const libc::c_char,
     b"object_gi_coin\x00" as *const u8 as *const libc::c_char,
     b"object_kanban\x00" as *const u8 as *const libc::c_char,
     b"object_gjyo_objects\x00" as *const u8 as *const libc::c_char,
     b"object_owl\x00" as *const u8 as *const libc::c_char,
     b"object_mk\x00" as *const u8 as *const libc::c_char,
     b"object_fu\x00" as *const u8 as *const libc::c_char,
     b"object_gi_ki_tan_mask\x00" as *const u8 as *const libc::c_char,
     b"object_gi_redead_mask\x00" as *const u8 as *const libc::c_char,
     b"object_gi_skj_mask\x00" as *const u8 as *const libc::c_char,
     b"object_gi_rabit_mask\x00" as *const u8 as *const libc::c_char,
     b"object_gi_truth_mask\x00" as *const u8 as *const libc::c_char,
     b"object_ganon_objects\x00" as *const u8 as *const libc::c_char,
     b"object_siofuki\x00" as *const u8 as *const libc::c_char,
     b"object_stream\x00" as *const u8 as *const libc::c_char,
     b"object_mm\x00" as *const u8 as *const libc::c_char,
     b"object_fa\x00" as *const u8 as *const libc::c_char,
     b"object_os\x00" as *const u8 as *const libc::c_char,
     b"object_gi_eye_lotion\x00" as *const u8 as *const libc::c_char,
     b"object_gi_powder\x00" as *const u8 as *const libc::c_char,
     b"object_gi_mushroom\x00" as *const u8 as *const libc::c_char,
     b"object_gi_ticketstone\x00" as *const u8 as *const libc::c_char,
     b"object_gi_brokensword\x00" as *const u8 as *const libc::c_char,
     b"object_js\x00" as *const u8 as *const libc::c_char,
     b"object_cs\x00" as *const u8 as *const libc::c_char,
     b"object_gi_prescription\x00" as *const u8 as *const libc::c_char,
     b"object_gi_bracelet\x00" as *const u8 as *const libc::c_char,
     b"object_gi_soldout\x00" as *const u8 as *const libc::c_char,
     b"object_gi_frog\x00" as *const u8 as *const libc::c_char,
     b"object_mag\x00" as *const u8 as *const libc::c_char,
     b"object_door_gerudo\x00" as *const u8 as *const libc::c_char,
     b"object_gt\x00" as *const u8 as *const libc::c_char,
     b"object_efc_erupc\x00" as *const u8 as *const libc::c_char,
     b"object_zl2_anime1\x00" as *const u8 as *const libc::c_char,
     b"object_zl2_anime2\x00" as *const u8 as *const libc::c_char,
     b"object_gi_golonmask\x00" as *const u8 as *const libc::c_char,
     b"object_gi_zoramask\x00" as *const u8 as *const libc::c_char,
     b"object_gi_gerudomask\x00" as *const u8 as *const libc::c_char,
     b"object_ganon2\x00" as *const u8 as *const libc::c_char,
     b"object_ka\x00" as *const u8 as *const libc::c_char,
     b"object_ts\x00" as *const u8 as *const libc::c_char,
     b"object_zg\x00" as *const u8 as *const libc::c_char,
     b"object_gi_hoverboots\x00" as *const u8 as *const libc::c_char,
     b"object_gi_m_arrow\x00" as *const u8 as *const libc::c_char,
     b"object_ds2\x00" as *const u8 as *const libc::c_char,
     b"object_ec\x00" as *const u8 as *const libc::c_char,
     b"object_fish\x00" as *const u8 as *const libc::c_char,
     b"object_gi_sutaru\x00" as *const u8 as *const libc::c_char,
     b"object_gi_goddess\x00" as *const u8 as *const libc::c_char,
     b"object_ssh\x00" as *const u8 as *const libc::c_char,
     b"object_bigokuta\x00" as *const u8 as *const libc::c_char,
     b"object_bg\x00" as *const u8 as *const libc::c_char,
     b"object_spot05_objects\x00" as *const u8 as *const libc::c_char,
     b"object_spot12_obj\x00" as *const u8 as *const libc::c_char,
     b"object_bombiwa\x00" as *const u8 as *const libc::c_char,
     b"object_hintnuts\x00" as *const u8 as *const libc::c_char,
     b"object_rs\x00" as *const u8 as *const libc::c_char,
     b"object_spot00_break\x00" as *const u8 as *const libc::c_char,
     b"object_gla\x00" as *const u8 as *const libc::c_char,
     b"object_shopnuts\x00" as *const u8 as *const libc::c_char,
     b"object_geldb\x00" as *const u8 as *const libc::c_char,
     b"object_gr\x00" as *const u8 as *const libc::c_char,
     b"object_dog\x00" as *const u8 as *const libc::c_char,
     b"object_jya_iron\x00" as *const u8 as *const libc::c_char,
     b"object_jya_door\x00" as *const u8 as *const libc::c_char,
     b"object_spot01_objects2\x00" as *const u8 as *const libc::c_char,
     b"object_spot11_obj\x00" as *const u8 as *const libc::c_char,
     b"object_kibako2\x00" as *const u8 as *const libc::c_char,
     b"object_dns\x00" as *const u8 as *const libc::c_char,
     b"object_dnk\x00" as *const u8 as *const libc::c_char,
     b"object_gi_fire\x00" as *const u8 as *const libc::c_char,
     b"object_gi_insect\x00" as *const u8 as *const libc::c_char,
     b"object_gi_butterfly\x00" as *const u8 as *const libc::c_char,
     b"object_gi_ghost\x00" as *const u8 as *const libc::c_char,
     b"object_gi_soul\x00" as *const u8 as *const libc::c_char,
     b"object_bowl\x00" as *const u8 as *const libc::c_char,
     b"object_po_field\x00" as *const u8 as *const libc::c_char,
     b"object_demo_kekkai\x00" as *const u8 as *const libc::c_char,
     b"object_efc_doughnut\x00" as *const u8 as *const libc::c_char,
     b"object_gi_dekupouch\x00" as *const u8 as *const libc::c_char,
     b"object_ganon_anime1\x00" as *const u8 as *const libc::c_char,
     b"object_ganon_anime2\x00" as *const u8 as *const libc::c_char,
     b"object_ganon_anime3\x00" as *const u8 as *const libc::c_char,
     b"object_gi_rupy\x00" as *const u8 as *const libc::c_char,
     b"object_spot01_matoya\x00" as *const u8 as *const libc::c_char,
     b"object_spot01_matoyab\x00" as *const u8 as *const libc::c_char,
     b"object_po_composer\x00" as *const u8 as *const libc::c_char,
     b"object_mu\x00" as *const u8 as *const libc::c_char,
     b"object_wf\x00" as *const u8 as *const libc::c_char,
     b"object_skb\x00" as *const u8 as *const libc::c_char,
     b"object_gj\x00" as *const u8 as *const libc::c_char,
     b"object_geff\x00" as *const u8 as *const libc::c_char,
     b"object_haka_door\x00" as *const u8 as *const libc::c_char,
     b"object_gs\x00" as *const u8 as *const libc::c_char,
     b"object_ps\x00" as *const u8 as *const libc::c_char,
     b"object_bwall\x00" as *const u8 as *const libc::c_char,
     b"object_crow\x00" as *const u8 as *const libc::c_char,
     b"object_cow\x00" as *const u8 as *const libc::c_char,
     b"object_cob\x00" as *const u8 as *const libc::c_char,
     b"object_gi_sword_1\x00" as *const u8 as *const libc::c_char,
     b"object_door_killer\x00" as *const u8 as *const libc::c_char,
     b"object_ouke_haka\x00" as *const u8 as *const libc::c_char,
     b"object_timeblock\x00" as *const u8 as *const libc::c_char,
     b"object_zl4\x00" as *const u8 as *const libc::c_char,
     b"g_pn_01\x00" as *const u8 as *const libc::c_char,
     b"g_pn_02\x00" as *const u8 as *const libc::c_char,
     b"g_pn_03\x00" as *const u8 as *const libc::c_char,
     b"g_pn_04\x00" as *const u8 as *const libc::c_char,
     b"g_pn_05\x00" as *const u8 as *const libc::c_char,
     b"g_pn_06\x00" as *const u8 as *const libc::c_char,
     b"g_pn_07\x00" as *const u8 as *const libc::c_char,
     b"g_pn_08\x00" as *const u8 as *const libc::c_char,
     b"g_pn_09\x00" as *const u8 as *const libc::c_char,
     b"g_pn_10\x00" as *const u8 as *const libc::c_char,
     b"g_pn_11\x00" as *const u8 as *const libc::c_char,
     b"g_pn_12\x00" as *const u8 as *const libc::c_char,
     b"g_pn_13\x00" as *const u8 as *const libc::c_char,
     b"g_pn_14\x00" as *const u8 as *const libc::c_char,
     b"g_pn_15\x00" as *const u8 as *const libc::c_char,
     b"g_pn_16\x00" as *const u8 as *const libc::c_char,
     b"g_pn_17\x00" as *const u8 as *const libc::c_char,
     b"g_pn_18\x00" as *const u8 as *const libc::c_char,
     b"g_pn_19\x00" as *const u8 as *const libc::c_char,
     b"g_pn_20\x00" as *const u8 as *const libc::c_char,
     b"g_pn_21\x00" as *const u8 as *const libc::c_char,
     b"g_pn_22\x00" as *const u8 as *const libc::c_char,
     b"g_pn_23\x00" as *const u8 as *const libc::c_char,
     b"g_pn_24\x00" as *const u8 as *const libc::c_char,
     b"g_pn_25\x00" as *const u8 as *const libc::c_char,
     b"g_pn_26\x00" as *const u8 as *const libc::c_char,
     b"g_pn_27\x00" as *const u8 as *const libc::c_char,
     b"g_pn_28\x00" as *const u8 as *const libc::c_char,
     b"g_pn_29\x00" as *const u8 as *const libc::c_char,
     b"g_pn_30\x00" as *const u8 as *const libc::c_char,
     b"g_pn_31\x00" as *const u8 as *const libc::c_char,
     b"g_pn_32\x00" as *const u8 as *const libc::c_char,
     b"g_pn_33\x00" as *const u8 as *const libc::c_char,
     b"g_pn_34\x00" as *const u8 as *const libc::c_char,
     b"g_pn_35\x00" as *const u8 as *const libc::c_char,
     b"g_pn_36\x00" as *const u8 as *const libc::c_char,
     b"g_pn_37\x00" as *const u8 as *const libc::c_char,
     b"g_pn_38\x00" as *const u8 as *const libc::c_char,
     b"g_pn_39\x00" as *const u8 as *const libc::c_char,
     b"g_pn_40\x00" as *const u8 as *const libc::c_char,
     b"g_pn_41\x00" as *const u8 as *const libc::c_char,
     b"g_pn_42\x00" as *const u8 as *const libc::c_char,
     b"g_pn_43\x00" as *const u8 as *const libc::c_char,
     b"g_pn_44\x00" as *const u8 as *const libc::c_char,
     b"g_pn_45\x00" as *const u8 as *const libc::c_char,
     b"g_pn_46\x00" as *const u8 as *const libc::c_char,
     b"g_pn_47\x00" as *const u8 as *const libc::c_char,
     b"g_pn_48\x00" as *const u8 as *const libc::c_char,
     b"g_pn_49\x00" as *const u8 as *const libc::c_char,
     b"g_pn_50\x00" as *const u8 as *const libc::c_char,
     b"g_pn_51\x00" as *const u8 as *const libc::c_char,
     b"g_pn_52\x00" as *const u8 as *const libc::c_char,
     b"g_pn_53\x00" as *const u8 as *const libc::c_char,
     b"g_pn_54\x00" as *const u8 as *const libc::c_char,
     b"g_pn_55\x00" as *const u8 as *const libc::c_char,
     b"g_pn_56\x00" as *const u8 as *const libc::c_char,
     b"g_pn_57\x00" as *const u8 as *const libc::c_char,
     b"z_select_static\x00" as *const u8 as *const libc::c_char,
     b"nintendo_rogo_static\x00" as *const u8 as *const libc::c_char,
     b"title_static\x00" as *const u8 as *const libc::c_char,
     b"parameter_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine0_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine0_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine1_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine1_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine2_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine2_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine3_static\x00" as *const u8 as *const libc::c_char,
     b"vr_fine3_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud0_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud0_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud1_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud1_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud2_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud2_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud3_static\x00" as *const u8 as *const libc::c_char,
     b"vr_cloud3_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_holy0_static\x00" as *const u8 as *const libc::c_char,
     b"vr_holy0_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_holy1_static\x00" as *const u8 as *const libc::c_char,
     b"vr_holy1_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_MDVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_MDVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_MNVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_MNVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_RUVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_RUVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_LHVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_LHVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KHVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KHVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_K3VR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_K3VR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_K4VR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_K4VR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_K5VR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_K5VR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_SP1a_static\x00" as *const u8 as *const libc::c_char,
     b"vr_SP1a_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_MLVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_MLVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KKRVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KKRVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KR3VR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KR3VR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_IPVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_IPVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KSVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_KSVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_GLVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_GLVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_ZRVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_ZRVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_DGVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_DGVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_ALVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_ALVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_NSVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_NSVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_LBVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_LBVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_TTVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_TTVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"vr_FCVR_static\x00" as *const u8 as *const libc::c_char,
     b"vr_FCVR_pal_static\x00" as *const u8 as *const libc::c_char,
     b"elf_message_field\x00" as *const u8 as *const libc::c_char,
     b"elf_message_ydan\x00" as *const u8 as *const libc::c_char,
     b"ydan_scene\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_0\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_1\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_2\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_3\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_4\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_5\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_6\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_7\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_8\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_9\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_10\x00" as *const u8 as *const libc::c_char,
     b"ydan_room_11\x00" as *const u8 as *const libc::c_char,
     b"ddan_scene\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_0\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_1\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_2\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_3\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_4\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_5\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_6\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_7\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_8\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_9\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_10\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_11\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_12\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_13\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_14\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_15\x00" as *const u8 as *const libc::c_char,
     b"ddan_room_16\x00" as *const u8 as *const libc::c_char,
     b"bdan_scene\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_0\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_1\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_2\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_3\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_4\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_5\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_6\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_7\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_8\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_9\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_10\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_11\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_12\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_13\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_14\x00" as *const u8 as *const libc::c_char,
     b"bdan_room_15\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_scene\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_0\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_1\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_2\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_3\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_4\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_5\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_6\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_7\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_8\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_9\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_10\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_11\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_12\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_13\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_14\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_15\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_16\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_17\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_18\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_19\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_20\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_21\x00" as *const u8 as *const libc::c_char,
     b"Bmori1_room_22\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_scene\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_0\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_1\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_2\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_3\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_4\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_5\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_6\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_7\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_8\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_9\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_10\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_11\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_12\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_13\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_14\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_15\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_16\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_17\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_18\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_19\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_20\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_21\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_22\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_23\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_24\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_25\x00" as *const u8 as *const libc::c_char,
     b"HIDAN_room_26\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_scene\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_0\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_1\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_2\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_3\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_4\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_5\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_6\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_7\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_8\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_9\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_10\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_11\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_12\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_13\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_14\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_15\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_16\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_17\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_18\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_19\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_20\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_21\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_room_22\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_scene\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_0\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_1\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_2\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_3\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_4\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_5\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_6\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_7\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_8\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_9\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_10\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_11\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_12\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_13\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_14\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_15\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_16\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_17\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_18\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_19\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_20\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_21\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_22\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_23\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_24\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_25\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_26\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_27\x00" as *const u8 as *const libc::c_char,
     b"jyasinzou_room_28\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_scene\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_0\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_1\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_2\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_3\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_4\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_5\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_6\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_7\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_8\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_9\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_10\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_11\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_12\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_13\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_14\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_15\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_16\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_17\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_18\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_19\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_20\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_21\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_room_22\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_scene\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_room_0\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_room_1\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_room_2\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_room_3\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_room_4\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_room_5\x00" as *const u8 as *const libc::c_char,
     b"HAKAdanCH_room_6\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_scene\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_0\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_1\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_2\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_3\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_4\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_5\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_6\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_7\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_8\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_9\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_10\x00" as *const u8 as *const libc::c_char,
     b"ice_doukutu_room_11\x00" as *const u8 as *const libc::c_char,
     b"men_scene\x00" as *const u8 as *const libc::c_char,
     b"men_room_0\x00" as *const u8 as *const libc::c_char,
     b"men_room_1\x00" as *const u8 as *const libc::c_char,
     b"men_room_2\x00" as *const u8 as *const libc::c_char,
     b"men_room_3\x00" as *const u8 as *const libc::c_char,
     b"men_room_4\x00" as *const u8 as *const libc::c_char,
     b"men_room_5\x00" as *const u8 as *const libc::c_char,
     b"men_room_6\x00" as *const u8 as *const libc::c_char,
     b"men_room_7\x00" as *const u8 as *const libc::c_char,
     b"men_room_8\x00" as *const u8 as *const libc::c_char,
     b"men_room_9\x00" as *const u8 as *const libc::c_char,
     b"men_room_10\x00" as *const u8 as *const libc::c_char,
     b"ganontika_scene\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_0\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_1\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_2\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_3\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_4\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_5\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_6\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_7\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_8\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_9\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_10\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_11\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_12\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_13\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_14\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_15\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_16\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_17\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_18\x00" as *const u8 as *const libc::c_char,
     b"ganontika_room_19\x00" as *const u8 as *const libc::c_char,
     b"syotes_scene\x00" as *const u8 as *const libc::c_char,
     b"syotes_room_0\x00" as *const u8 as *const libc::c_char,
     b"syotes2_scene\x00" as *const u8 as *const libc::c_char,
     b"syotes2_room_0\x00" as *const u8 as *const libc::c_char,
     b"depth_test_scene\x00" as *const u8 as *const libc::c_char,
     b"depth_test_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot00_scene\x00" as *const u8 as *const libc::c_char,
     b"spot00_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot01_scene\x00" as *const u8 as *const libc::c_char,
     b"spot01_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot02_scene\x00" as *const u8 as *const libc::c_char,
     b"spot02_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot02_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot03_scene\x00" as *const u8 as *const libc::c_char,
     b"spot03_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot03_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot04_scene\x00" as *const u8 as *const libc::c_char,
     b"spot04_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot04_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot04_room_2\x00" as *const u8 as *const libc::c_char,
     b"spot05_scene\x00" as *const u8 as *const libc::c_char,
     b"spot05_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot06_scene\x00" as *const u8 as *const libc::c_char,
     b"spot06_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot07_scene\x00" as *const u8 as *const libc::c_char,
     b"spot07_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot07_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot08_scene\x00" as *const u8 as *const libc::c_char,
     b"spot08_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot09_scene\x00" as *const u8 as *const libc::c_char,
     b"spot09_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot10_scene\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_2\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_3\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_4\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_5\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_6\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_7\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_8\x00" as *const u8 as *const libc::c_char,
     b"spot10_room_9\x00" as *const u8 as *const libc::c_char,
     b"spot11_scene\x00" as *const u8 as *const libc::c_char,
     b"spot11_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot12_scene\x00" as *const u8 as *const libc::c_char,
     b"spot12_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot12_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot13_scene\x00" as *const u8 as *const libc::c_char,
     b"spot13_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot13_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot15_scene\x00" as *const u8 as *const libc::c_char,
     b"spot15_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot16_scene\x00" as *const u8 as *const libc::c_char,
     b"spot16_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot17_scene\x00" as *const u8 as *const libc::c_char,
     b"spot17_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot17_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot18_scene\x00" as *const u8 as *const libc::c_char,
     b"spot18_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot18_room_1\x00" as *const u8 as *const libc::c_char,
     b"spot18_room_2\x00" as *const u8 as *const libc::c_char,
     b"spot18_room_3\x00" as *const u8 as *const libc::c_char,
     b"market_day_scene\x00" as *const u8 as *const libc::c_char,
     b"market_day_room_0\x00" as *const u8 as *const libc::c_char,
     b"market_night_scene\x00" as *const u8 as *const libc::c_char,
     b"market_night_room_0\x00" as *const u8 as *const libc::c_char,
     b"testroom_scene\x00" as *const u8 as *const libc::c_char,
     b"testroom_room_0\x00" as *const u8 as *const libc::c_char,
     b"testroom_room_1\x00" as *const u8 as *const libc::c_char,
     b"testroom_room_2\x00" as *const u8 as *const libc::c_char,
     b"testroom_room_3\x00" as *const u8 as *const libc::c_char,
     b"testroom_room_4\x00" as *const u8 as *const libc::c_char,
     b"kenjyanoma_scene\x00" as *const u8 as *const libc::c_char,
     b"kenjyanoma_room_0\x00" as *const u8 as *const libc::c_char,
     b"tokinoma_scene\x00" as *const u8 as *const libc::c_char,
     b"tokinoma_room_0\x00" as *const u8 as *const libc::c_char,
     b"tokinoma_room_1\x00" as *const u8 as *const libc::c_char,
     b"sutaru_scene\x00" as *const u8 as *const libc::c_char,
     b"sutaru_room_0\x00" as *const u8 as *const libc::c_char,
     b"link_home_scene\x00" as *const u8 as *const libc::c_char,
     b"link_home_room_0\x00" as *const u8 as *const libc::c_char,
     b"kokiri_shop_scene\x00" as *const u8 as *const libc::c_char,
     b"kokiri_shop_room_0\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home_scene\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home_room_0\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_scene\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_0\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_1\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_2\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_3\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_4\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_5\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_6\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_7\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_8\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_9\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_10\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_11\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_12\x00" as *const u8 as *const libc::c_char,
     b"kakusiana_room_13\x00" as *const u8 as *const libc::c_char,
     b"entra_scene\x00" as *const u8 as *const libc::c_char,
     b"entra_room_0\x00" as *const u8 as *const libc::c_char,
     b"moribossroom_scene\x00" as *const u8 as *const libc::c_char,
     b"moribossroom_room_0\x00" as *const u8 as *const libc::c_char,
     b"moribossroom_room_1\x00" as *const u8 as *const libc::c_char,
     b"syatekijyou_scene\x00" as *const u8 as *const libc::c_char,
     b"syatekijyou_room_0\x00" as *const u8 as *const libc::c_char,
     b"shop1_scene\x00" as *const u8 as *const libc::c_char,
     b"shop1_room_0\x00" as *const u8 as *const libc::c_char,
     b"hairal_niwa_scene\x00" as *const u8 as *const libc::c_char,
     b"hairal_niwa_room_0\x00" as *const u8 as *const libc::c_char,
     b"ganon_tou_scene\x00" as *const u8 as *const libc::c_char,
     b"ganon_tou_room_0\x00" as *const u8 as *const libc::c_char,
     b"sasatest_scene\x00" as *const u8 as *const libc::c_char,
     b"sasatest_room_0\x00" as *const u8 as *const libc::c_char,
     b"market_alley_scene\x00" as *const u8 as *const libc::c_char,
     b"market_alley_room_0\x00" as *const u8 as *const libc::c_char,
     b"spot20_scene\x00" as *const u8 as *const libc::c_char,
     b"spot20_room_0\x00" as *const u8 as *const libc::c_char,
     b"market_ruins_scene\x00" as *const u8 as *const libc::c_char,
     b"market_ruins_room_0\x00" as *const u8 as *const libc::c_char,
     b"entra_n_scene\x00" as *const u8 as *const libc::c_char,
     b"entra_n_room_0\x00" as *const u8 as *const libc::c_char,
     b"enrui_scene\x00" as *const u8 as *const libc::c_char,
     b"enrui_room_0\x00" as *const u8 as *const libc::c_char,
     b"market_alley_n_scene\x00" as *const u8 as *const libc::c_char,
     b"market_alley_n_room_0\x00" as *const u8 as *const libc::c_char,
     b"hiral_demo_scene\x00" as *const u8 as *const libc::c_char,
     b"hiral_demo_room_0\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home3_scene\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home3_room_0\x00" as *const u8 as *const libc::c_char,
     b"malon_stable_scene\x00" as *const u8 as *const libc::c_char,
     b"malon_stable_room_0\x00" as *const u8 as *const libc::c_char,
     b"kakariko_scene\x00" as *const u8 as *const libc::c_char,
     b"kakariko_room_0\x00" as *const u8 as *const libc::c_char,
     b"bdan_boss_scene\x00" as *const u8 as *const libc::c_char,
     b"bdan_boss_room_0\x00" as *const u8 as *const libc::c_char,
     b"bdan_boss_room_1\x00" as *const u8 as *const libc::c_char,
     b"FIRE_bs_scene\x00" as *const u8 as *const libc::c_char,
     b"FIRE_bs_room_0\x00" as *const u8 as *const libc::c_char,
     b"FIRE_bs_room_1\x00" as *const u8 as *const libc::c_char,
     b"hut_scene\x00" as *const u8 as *const libc::c_char,
     b"hut_room_0\x00" as *const u8 as *const libc::c_char,
     b"daiyousei_izumi_scene\x00" as *const u8 as *const libc::c_char,
     b"daiyousei_izumi_room_0\x00" as *const u8 as *const libc::c_char,
     b"hakaana_scene\x00" as *const u8 as *const libc::c_char,
     b"hakaana_room_0\x00" as *const u8 as *const libc::c_char,
     b"yousei_izumi_tate_scene\x00" as *const u8 as *const libc::c_char,
     b"yousei_izumi_tate_room_0\x00" as *const u8 as *const libc::c_char,
     b"yousei_izumi_yoko_scene\x00" as *const u8 as *const libc::c_char,
     b"yousei_izumi_yoko_room_0\x00" as *const u8 as *const libc::c_char,
     b"golon_scene\x00" as *const u8 as *const libc::c_char,
     b"golon_room_0\x00" as *const u8 as *const libc::c_char,
     b"zoora_scene\x00" as *const u8 as *const libc::c_char,
     b"zoora_room_0\x00" as *const u8 as *const libc::c_char,
     b"drag_scene\x00" as *const u8 as *const libc::c_char,
     b"drag_room_0\x00" as *const u8 as *const libc::c_char,
     b"alley_shop_scene\x00" as *const u8 as *const libc::c_char,
     b"alley_shop_room_0\x00" as *const u8 as *const libc::c_char,
     b"night_shop_scene\x00" as *const u8 as *const libc::c_char,
     b"night_shop_room_0\x00" as *const u8 as *const libc::c_char,
     b"impa_scene\x00" as *const u8 as *const libc::c_char,
     b"impa_room_0\x00" as *const u8 as *const libc::c_char,
     b"labo_scene\x00" as *const u8 as *const libc::c_char,
     b"labo_room_0\x00" as *const u8 as *const libc::c_char,
     b"tent_scene\x00" as *const u8 as *const libc::c_char,
     b"tent_room_0\x00" as *const u8 as *const libc::c_char,
     b"nakaniwa_scene\x00" as *const u8 as *const libc::c_char,
     b"nakaniwa_room_0\x00" as *const u8 as *const libc::c_char,
     b"ddan_boss_scene\x00" as *const u8 as *const libc::c_char,
     b"ddan_boss_room_0\x00" as *const u8 as *const libc::c_char,
     b"ddan_boss_room_1\x00" as *const u8 as *const libc::c_char,
     b"ydan_boss_scene\x00" as *const u8 as *const libc::c_char,
     b"ydan_boss_room_0\x00" as *const u8 as *const libc::c_char,
     b"ydan_boss_room_1\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_bs_scene\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_bs_room_0\x00" as *const u8 as *const libc::c_char,
     b"HAKAdan_bs_room_1\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_bs_scene\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_bs_room_0\x00" as *const u8 as *const libc::c_char,
     b"MIZUsin_bs_room_1\x00" as *const u8 as *const libc::c_char,
     b"ganon_scene\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_0\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_1\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_2\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_3\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_4\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_5\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_6\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_7\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_8\x00" as *const u8 as *const libc::c_char,
     b"ganon_room_9\x00" as *const u8 as *const libc::c_char,
     b"ganon_boss_scene\x00" as *const u8 as *const libc::c_char,
     b"ganon_boss_room_0\x00" as *const u8 as *const libc::c_char,
     b"jyasinboss_scene\x00" as *const u8 as *const libc::c_char,
     b"jyasinboss_room_0\x00" as *const u8 as *const libc::c_char,
     b"jyasinboss_room_1\x00" as *const u8 as *const libc::c_char,
     b"jyasinboss_room_2\x00" as *const u8 as *const libc::c_char,
     b"jyasinboss_room_3\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home4_scene\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home4_room_0\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home5_scene\x00" as *const u8 as *const libc::c_char,
     b"kokiri_home5_room_0\x00" as *const u8 as *const libc::c_char,
     b"ganon_final_scene\x00" as *const u8 as *const libc::c_char,
     b"ganon_final_room_0\x00" as *const u8 as *const libc::c_char,
     b"kakariko3_scene\x00" as *const u8 as *const libc::c_char,
     b"kakariko3_room_0\x00" as *const u8 as *const libc::c_char,
     b"hairal_niwa2_scene\x00" as *const u8 as *const libc::c_char,
     b"hairal_niwa2_room_0\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_scene\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_room_0\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_room_1\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_room_2\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_room_3\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_room_4\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_room_5\x00" as *const u8 as *const libc::c_char,
     b"hakasitarelay_room_6\x00" as *const u8 as *const libc::c_char,
     b"shrine_scene\x00" as *const u8 as *const libc::c_char,
     b"shrine_room_0\x00" as *const u8 as *const libc::c_char,
     b"turibori_scene\x00" as *const u8 as *const libc::c_char,
     b"turibori_room_0\x00" as *const u8 as *const libc::c_char,
     b"shrine_n_scene\x00" as *const u8 as *const libc::c_char,
     b"shrine_n_room_0\x00" as *const u8 as *const libc::c_char,
     b"shrine_r_scene\x00" as *const u8 as *const libc::c_char,
     b"shrine_r_room_0\x00" as *const u8 as *const libc::c_char,
     b"hakaana2_scene\x00" as *const u8 as *const libc::c_char,
     b"hakaana2_room_0\x00" as *const u8 as *const libc::c_char,
     b"gerudoway_scene\x00" as *const u8 as *const libc::c_char,
     b"gerudoway_room_0\x00" as *const u8 as *const libc::c_char,
     b"gerudoway_room_1\x00" as *const u8 as *const libc::c_char,
     b"gerudoway_room_2\x00" as *const u8 as *const libc::c_char,
     b"gerudoway_room_3\x00" as *const u8 as *const libc::c_char,
     b"gerudoway_room_4\x00" as *const u8 as *const libc::c_char,
     b"gerudoway_room_5\x00" as *const u8 as *const libc::c_char,
     b"hairal_niwa_n_scene\x00" as *const u8 as *const libc::c_char,
     b"hairal_niwa_n_room_0\x00" as *const u8 as *const libc::c_char,
     b"bowling_scene\x00" as *const u8 as *const libc::c_char,
     b"bowling_room_0\x00" as *const u8 as *const libc::c_char,
     b"hakaana_ouke_scene\x00" as *const u8 as *const libc::c_char,
     b"hakaana_ouke_room_0\x00" as *const u8 as *const libc::c_char,
     b"hakaana_ouke_room_1\x00" as *const u8 as *const libc::c_char,
     b"hakaana_ouke_room_2\x00" as *const u8 as *const libc::c_char,
     b"hylia_labo_scene\x00" as *const u8 as *const libc::c_char,
     b"hylia_labo_room_0\x00" as *const u8 as *const libc::c_char,
     b"souko_scene\x00" as *const u8 as *const libc::c_char,
     b"souko_room_0\x00" as *const u8 as *const libc::c_char,
     b"souko_room_1\x00" as *const u8 as *const libc::c_char,
     b"souko_room_2\x00" as *const u8 as *const libc::c_char,
     b"miharigoya_scene\x00" as *const u8 as *const libc::c_char,
     b"miharigoya_room_0\x00" as *const u8 as *const libc::c_char,
     b"mahouya_scene\x00" as *const u8 as *const libc::c_char,
     b"mahouya_room_0\x00" as *const u8 as *const libc::c_char,
     b"takaraya_scene\x00" as *const u8 as *const libc::c_char,
     b"takaraya_room_0\x00" as *const u8 as *const libc::c_char,
     b"takaraya_room_1\x00" as *const u8 as *const libc::c_char,
     b"takaraya_room_2\x00" as *const u8 as *const libc::c_char,
     b"takaraya_room_3\x00" as *const u8 as *const libc::c_char,
     b"takaraya_room_4\x00" as *const u8 as *const libc::c_char,
     b"takaraya_room_5\x00" as *const u8 as *const libc::c_char,
     b"takaraya_room_6\x00" as *const u8 as *const libc::c_char,
     b"ganon_sonogo_scene\x00" as *const u8 as *const libc::c_char,
     b"ganon_sonogo_room_0\x00" as *const u8 as *const libc::c_char,
     b"ganon_sonogo_room_1\x00" as *const u8 as *const libc::c_char,
     b"ganon_sonogo_room_2\x00" as *const u8 as *const libc::c_char,
     b"ganon_sonogo_room_3\x00" as *const u8 as *const libc::c_char,
     b"ganon_sonogo_room_4\x00" as *const u8 as *const libc::c_char,
     b"ganon_demo_scene\x00" as *const u8 as *const libc::c_char,
     b"ganon_demo_room_0\x00" as *const u8 as *const libc::c_char,
     b"besitu_scene\x00" as *const u8 as *const libc::c_char,
     b"besitu_room_0\x00" as *const u8 as *const libc::c_char,
     b"face_shop_scene\x00" as *const u8 as *const libc::c_char,
     b"face_shop_room_0\x00" as *const u8 as *const libc::c_char,
     b"kinsuta_scene\x00" as *const u8 as *const libc::c_char,
     b"kinsuta_room_0\x00" as *const u8 as *const libc::c_char,
     b"ganontikasonogo_scene\x00" as *const u8 as *const libc::c_char,
     b"ganontikasonogo_room_0\x00" as *const u8 as *const libc::c_char,
     b"ganontikasonogo_room_1\x00" as *const u8 as *const libc::c_char,
     b"test01_scene\x00" as *const u8 as *const libc::c_char,
     b"test01_room_0\x00" as *const u8 as *const libc::c_char,
     b"bump_texture_static\x00" as *const u8 as *const libc::c_char,
     b"anime_model_1_static\x00" as *const u8 as *const libc::c_char,
     b"anime_model_2_static\x00" as *const u8 as *const libc::c_char,
     b"anime_model_3_static\x00" as *const u8 as *const libc::c_char,
     b"anime_model_4_static\x00" as *const u8 as *const libc::c_char,
     b"anime_model_5_static\x00" as *const u8 as *const libc::c_char,
     b"anime_model_6_static\x00" as *const u8 as *const libc::c_char,
     b"anime_texture_1_static\x00" as *const u8 as *const libc::c_char,
     b"anime_texture_2_static\x00" as *const u8 as *const libc::c_char,
     b"anime_texture_3_static\x00" as *const u8 as *const libc::c_char,
     b"anime_texture_4_static\x00" as *const u8 as *const libc::c_char,
     b"anime_texture_5_static\x00" as *const u8 as *const libc::c_char,
     b"anime_texture_6_static\x00" as *const u8 as *const libc::c_char,
     b"softsprite_matrix_static\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_CompareName(mut name1: *const libc::c_char,
                                            mut name2: *const libc::c_char)
 -> s32 {
    while *name1 as libc::c_uint != 0 as libc::c_uint {
        if *name1 as libc::c_int > *name2 as libc::c_int {
            return 1 as libc::c_int
        }
        if (*name1 as libc::c_int) < *name2 as libc::c_int {
            return -(1 as libc::c_int)
        }
        name1 = name1.offset(1);
        name2 = name2.offset(1)
    }
    if *name2 as libc::c_int > 0 as libc::c_int { return -(1 as libc::c_int) }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_DmaRomToRam(mut rom: u32_0, mut ram: u32_0,
                                            mut size: u32_0) -> s32 {
    let mut current_block: u64;
    let mut ioMsg: OSIoMesg =
        OSIoMesg{hdr:
                     OSIoMesgHdr{type_0: 0,
                                 pri: 0,
                                 status: 0,
                                 retQueue: 0 as *mut OSMesgQueue,},
                 dramAddr: 0 as *mut libc::c_void,
                 devAddr: 0,
                 size: 0,
                 piHandle: 0 as *mut OSPiHandle,};
    let mut queue: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut ret: s32 = 0;
    let mut buffSize: u32_0 = gDmaMgrDmaBuffSize;
    let mut pad: [s32; 2] = [0; 2];
    if buffSize == 0 as libc::c_int as libc::c_uint {
        buffSize = 0x2000 as libc::c_int as u32_0
    }
    osInvalICache(ram as *mut libc::c_void, size as s32);
    osInvalDCache(ram as *mut libc::c_void, size as s32);
    osCreateMesgQueue(&mut queue, &mut msg, 1 as libc::c_int);
    loop  {
        if !(size > buffSize) {
            current_block = 18377268871191777778;
            break ;
        }
        // Necessary to match
        ioMsg.hdr.pri = 0 as libc::c_int as u8_0;
        ioMsg.hdr.retQueue = &mut queue;
        ioMsg.devAddr = rom;
        ioMsg.dramAddr = ram as *mut libc::c_void;
        ioMsg.size = buffSize as size_t;
        if D_80009460 == 10 as libc::c_int as libc::c_uint {
            osSyncPrintf(b"%10lld \xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\xab\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1 %08x %08x %08x (%d)\n\x00"
                             as *const u8 as *const libc::c_char,
                         osGetTime().wrapping_mul((1000000 as libc::c_longlong
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
                                                                                          libc::c_ulonglong),
                         ioMsg.dramAddr, ioMsg.devAddr, ioMsg.size,
                         gPiMgrCmdQ.validCount);
        }
        ret = osEPiStartDma(gCartHandle, &mut ioMsg, 0 as libc::c_int);
        if ret != 0 { current_block = 15688134543633727796; break ; }
        if D_80009460 == 10 as libc::c_int as libc::c_uint {
            osSyncPrintf(b"%10lld \xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\xab\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1 START (%d)\n\x00"
                             as *const u8 as *const libc::c_char,
                         osGetTime().wrapping_mul((1000000 as libc::c_longlong
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
                                                                                          libc::c_ulonglong),
                         gPiMgrCmdQ.validCount);
        }
        osRecvMesg(&mut queue, 0 as *mut OSMesg, 1 as libc::c_int);
        if D_80009460 == 10 as libc::c_int as libc::c_uint {
            osSyncPrintf(b"%10lld \xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\xab\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1 END (%d)\n\x00"
                             as *const u8 as *const libc::c_char,
                         osGetTime().wrapping_mul((1000000 as libc::c_longlong
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
                                                                                          libc::c_ulonglong),
                         gPiMgrCmdQ.validCount);
        }
        size =
            (size as libc::c_uint).wrapping_sub(buffSize) as u32_0 as u32_0;
        rom = (rom as libc::c_uint).wrapping_add(buffSize) as u32_0 as u32_0;
        ram = (ram as libc::c_uint).wrapping_add(buffSize) as u32_0 as u32_0
    }
    match current_block {
        18377268871191777778 =>
        // Also necessary to match
        {
            ioMsg.hdr.pri = 0 as libc::c_int as u8_0;
            ioMsg.hdr.retQueue = &mut queue;
            ioMsg.devAddr = rom;
            ioMsg.dramAddr = ram as *mut libc::c_void;
            ioMsg.size = size as size_t;
            if D_80009460 == 10 as libc::c_int as libc::c_uint {
                osSyncPrintf(b"%10lld \xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\xab\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1 %08x %08x %08x (%d)\n\x00"
                                 as *const u8 as *const libc::c_char,
                             osGetTime().wrapping_mul((1000000 as
                                                           libc::c_longlong /
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
                                                                                              libc::c_ulonglong),
                             ioMsg.dramAddr, ioMsg.devAddr, ioMsg.size,
                             gPiMgrCmdQ.validCount);
            }
            ret = osEPiStartDma(gCartHandle, &mut ioMsg, 0 as libc::c_int);
            if !(ret != 0) {
                osRecvMesg(&mut queue, 0 as *mut OSMesg, 1 as libc::c_int);
                if D_80009460 == 10 as libc::c_int as libc::c_uint {
                    osSyncPrintf(b"%10lld \xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\xab\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1 END (%d)\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 osGetTime().wrapping_mul((1000000 as
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
                                                                                                  libc::c_ulonglong),
                                 gPiMgrCmdQ.validCount);
                }
            }
        }
        _ => { }
    }
    osInvalICache(ram as *mut libc::c_void, size as s32);
    osInvalDCache(ram as *mut libc::c_void, size as s32);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_DmaHandler(mut pihandle: *mut OSPiHandle,
                                           mut mb: *mut OSIoMesg,
                                           mut direction: s32) -> s32 {
    let mut ret: s32 = 0;
    if pihandle == gCartHandle {
    } else {
        __assert(b"pihandle == carthandle\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_std_dma.c\x00" as *const u8 as *const libc::c_char,
                 530 as libc::c_int);
    };
    if direction == 0 as libc::c_int {
    } else {
        __assert(b"direction == OS_READ\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_std_dma.c\x00" as *const u8 as *const libc::c_char,
                 531 as libc::c_int);
    };
    if !mb.is_null() {
    } else {
        __assert(b"mb != NULL\x00" as *const u8 as *const libc::c_char,
                 b"../z_std_dma.c\x00" as *const u8 as *const libc::c_char,
                 532 as libc::c_int);
    };
    if D_80009460 == 10 as libc::c_int as libc::c_uint {
        osSyncPrintf(b"%10lld \xe3\x82\xb5\xe3\x82\xa6\xe3\x83\xb3\xe3\x83\x89\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1 %08x %08x %08x (%d)\n\x00"
                         as *const u8 as *const libc::c_char,
                     osGetTime().wrapping_mul((1000000 as libc::c_longlong /
                                                   15625 as libc::c_longlong)
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
                                                                                      libc::c_ulonglong),
                     (*mb).dramAddr, (*mb).devAddr, (*mb).size,
                     gPiMgrCmdQ.validCount);
    }
    ret = osEPiStartDma(pihandle, mb, direction);
    if ret != 0 {
        osSyncPrintf(b"OOPS!!\n\x00" as *const u8 as *const libc::c_char);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_DmaFromDriveRom(mut ram: u32_0,
                                                mut rom: u32_0,
                                                mut size: u32_0) {
    let mut handle: *mut OSPiHandle = osDriveRomInit();
    let mut queue: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut ioMsg: OSIoMesg =
        OSIoMesg{hdr:
                     OSIoMesgHdr{type_0: 0,
                                 pri: 0,
                                 status: 0,
                                 retQueue: 0 as *mut OSMesgQueue,},
                 dramAddr: 0 as *mut libc::c_void,
                 devAddr: 0,
                 size: 0,
                 piHandle: 0 as *mut OSPiHandle,};
    osInvalICache(ram as *mut libc::c_void, size as s32);
    osInvalDCache(ram as *mut libc::c_void, size as s32);
    osCreateMesgQueue(&mut queue, &mut msg, 1 as libc::c_int);
    ioMsg.hdr.retQueue = &mut queue;
    ioMsg.hdr.pri = 0 as libc::c_int as u8_0;
    ioMsg.devAddr = rom;
    ioMsg.dramAddr = ram as *mut libc::c_void;
    ioMsg.size = size as size_t;
    (*handle).transferInfo.cmdType = 2 as libc::c_int as u32_0;
    osEPiStartDma(handle, &mut ioMsg, 0 as libc::c_int);
    osRecvMesg(&mut queue, 0 as *mut OSMesg, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_Error(mut req: *mut DmaRequest,
                                      mut file: *const libc::c_char,
                                      mut errorName: *const libc::c_char,
                                      mut errorDesc: *const libc::c_char) {
    let mut vrom: u32_0 = (*req).vromAddr;
    let mut ram: u32_0 = (*req).dramAddr as u32_0;
    let mut size: u32_0 = (*req).size;
    let mut buff1: [libc::c_char; 80] = [0; 80];
    let mut buff2: [libc::c_char; 80] = [0; 80];
    osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                 7 as libc::c_int);
    osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"DMA\xe8\x87\xb4\xe5\x91\xbd\xe7\x9a\x84\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc(%s)\nROM:%X RAM:%X SIZE:%X %s\n\x00"
                     as *const u8 as *const libc::c_char,
                 if !errorDesc.is_null() {
                     errorDesc
                 } else if !errorName.is_null() {
                     errorName
                 } else { b"???\x00" as *const u8 as *const libc::c_char },
                 vrom, ram, size,
                 if !file.is_null() {
                     file
                 } else { b"???\x00" as *const u8 as *const libc::c_char });
    if !(*req).filename.is_null() {
        osSyncPrintf(b"DMA ERROR: %s %d\x00" as *const u8 as
                         *const libc::c_char, (*req).filename, (*req).line);
    } else if !sDmaMgrCurFileName.is_null() {
        osSyncPrintf(b"DMA ERROR: %s %d\x00" as *const u8 as
                         *const libc::c_char, sDmaMgrCurFileName,
                     sDmaMgrCurFileLine);
    }
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    if !(*req).filename.is_null() {
        sprintf(buff1.as_mut_ptr(),
                b"DMA ERROR: %s %d\x00" as *const u8 as *const libc::c_char,
                (*req).filename, (*req).line);
    } else if !sDmaMgrCurFileName.is_null() {
        sprintf(buff1.as_mut_ptr(),
                b"DMA ERROR: %s %d\x00" as *const u8 as *const libc::c_char,
                sDmaMgrCurFileName, sDmaMgrCurFileLine);
    } else {
        sprintf(buff1.as_mut_ptr(),
                b"DMA ERROR: %s\x00" as *const u8 as *const libc::c_char,
                if !errorName.is_null() {
                    errorName
                } else { b"???\x00" as *const u8 as *const libc::c_char });
    }
    sprintf(buff2.as_mut_ptr(),
            b"%07X %08X %X %s\x00" as *const u8 as *const libc::c_char, vrom,
            ram, size,
            if !file.is_null() {
                file
            } else { b"???\x00" as *const u8 as *const libc::c_char });
    Fault_AddHungupAndCrashImpl(buff1.as_mut_ptr(), buff2.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_GetFileNameImpl(mut vrom: u32_0)
 -> *const libc::c_char {
    let mut iter: *mut DmaEntry = gDmaDataTable.as_mut_ptr();
    let mut name: *mut *const libc::c_char = sDmaMgrFileNames.as_mut_ptr();
    while (*iter).vromEnd != 0 {
        if vrom >= (*iter).vromStart && vrom < (*iter).vromEnd {
            return *name
        }
        iter = iter.offset(1);
        name = name.offset(1)
    }
    panic!("Reached end of non-void function without returning");
    // ! @bug Since there is no return, in case the file isn't found, the return value will be a pointer to the end
    // of gDmaDataTable
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_GetFileName(mut vrom: u32_0)
 -> *const libc::c_char {
    let mut ret: *const libc::c_char = DmaMgr_GetFileNameImpl(vrom);
    if ret.is_null() {
        return b"(unknown)\x00" as *const u8 as *const libc::c_char
    }
    if DmaMgr_CompareName(ret,
                          b"kanji\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           DmaMgr_CompareName(ret,
                              b"link_animetion\x00" as *const u8 as
                                  *const libc::c_char) == 0 as libc::c_int {
        return 0 as *const libc::c_char
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_ProcessMsg(mut req: *mut DmaRequest) {
    let mut vrom: u32_0 = (*req).vromAddr;
    let mut ram: *mut libc::c_void = (*req).dramAddr;
    let mut size: u32_0 = (*req).size;
    let mut romStart: u32_0 = 0;
    let mut romSize: u32_0 = 0;
    let mut found: u8_0 = 0 as libc::c_int as u8_0;
    let mut iter: *mut DmaEntry = 0 as *mut DmaEntry;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    filename = DmaMgr_GetFileName(vrom);
    iter = gDmaDataTable.as_mut_ptr();
    while (*iter).vromEnd != 0 {
        if vrom >= (*iter).vromStart && vrom < (*iter).vromEnd {
            // Necessary to match
            if (*iter).romEnd == 0 as libc::c_int as libc::c_uint {
                if (*iter).vromEnd < vrom.wrapping_add(size) {
                    DmaMgr_Error(req, filename,
                                 b"Segment Alignment Error\x00" as *const u8
                                     as *const libc::c_char,
                                 b"\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88\xe5\xa2\x83\xe7\x95\x8c\xe3\x82\x92\xe3\x81\xbe\xe3\x81\x9f\xe3\x81\x8c\xe3\x81\xa3\xe3\x81\xa6\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1\xe8\xbb\xa2\xe9\x80\x81\xe3\x81\x99\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\xe3\x81\xaf\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\x00"
                                     as *const u8 as *const libc::c_char);
                }
                DmaMgr_DmaRomToRam((*iter).romStart.wrapping_add(vrom.wrapping_sub((*iter).vromStart)),
                                   ram as u32_0, size);
                found = 1 as libc::c_int as u8_0
            } else {
                romStart = (*iter).romStart;
                romSize = (*iter).romEnd.wrapping_sub((*iter).romStart);
                if vrom != (*iter).vromStart {
                    DmaMgr_Error(req, filename,
                                 b"Can\'t Transfer Segment\x00" as *const u8
                                     as *const libc::c_char,
                                 b"\xe5\x9c\xa7\xe7\xb8\xae\xe3\x81\x95\xe3\x82\x8c\xe3\x81\x9f\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae\xe9\x80\x94\xe4\xb8\xad\xe3\x81\x8b\xe3\x82\x89\xe3\x81\xaf\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1\xe8\xbb\xa2\xe9\x80\x81\xe3\x81\x99\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\xe3\x81\xaf\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\x00"
                                     as *const u8 as *const libc::c_char);
                }
                if size != (*iter).vromEnd.wrapping_sub((*iter).vromStart) {
                    DmaMgr_Error(req, filename,
                                 b"Can\'t Transfer Segment\x00" as *const u8
                                     as *const libc::c_char,
                                 b"\xe5\x9c\xa7\xe7\xb8\xae\xe3\x81\x95\xe3\x82\x8c\xe3\x81\x9f\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae\xe4\xb8\x80\xe9\x83\xa8\xe3\x81\xa0\xe3\x81\x91\xe3\x82\x92\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1\xe8\xbb\xa2\xe9\x80\x81\xe3\x81\x99\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\xe3\x81\xaf\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\x00"
                                     as *const u8 as *const libc::c_char);
                }
                osSetThreadPri(0 as *mut OSThread, 10 as libc::c_int);
                Yaz0_Decompress(romStart, ram, romSize);
                osSetThreadPri(0 as *mut OSThread, 16 as libc::c_int);
                found = 1 as libc::c_int as u8_0
            }
            break ;
        } else { iter = iter.offset(1) }
    }
    if found == 0 {
        if sDmaMgrDataExistError != 0 {
            DmaMgr_Error(req, 0 as *const libc::c_char,
                         b"DATA DON\'T EXIST\x00" as *const u8 as
                             *const libc::c_char,
                         b"\xe8\xa9\xb2\xe5\xbd\x93\xe3\x81\x99\xe3\x82\x8b\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe3\x81\x8c\xe5\xad\x98\xe5\x9c\xa8\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\x00"
                             as *const u8 as *const libc::c_char);
            return
        }
        DmaMgr_DmaRomToRam(vrom, ram as u32_0, size);
    };
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_ThreadEntry(mut arg0: *mut libc::c_void) {
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut req: *mut DmaRequest = 0 as *mut DmaRequest;
    osSyncPrintf(b"\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    loop  {
        osRecvMesg(&mut sDmaMgrMsgQueue, &mut msg, 1 as libc::c_int);
        req = msg as *mut DmaRequest;
        if req.is_null() { break ; }
        DmaMgr_ProcessMsg(req);
        if !(*req).notifyQueue.is_null() {
            osSendMesg((*req).notifyQueue, (*req).notifyMsg,
                       0 as libc::c_int);
        }
    }
    osSyncPrintf(b"\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_SendRequestImpl(mut req: *mut DmaRequest,
                                                mut ram: u32_0,
                                                mut vrom: u32_0,
                                                mut size: u32_0,
                                                mut unk: u32_0,
                                                mut queue: *mut OSMesgQueue,
                                                mut msg: OSMesg) -> s32 {
    static mut sDmaMgrQueueFullLogged: s32 = 0 as libc::c_int;
    if 1 as libc::c_int != 0 && ram == 0 as libc::c_int as libc::c_uint ||
           osMemSize <
               ram.wrapping_add(size).wrapping_add(0x80000000 as libc::c_uint)
           || vrom & 1 as libc::c_int as libc::c_uint != 0 ||
           vrom > 0x4000000 as libc::c_int as libc::c_uint ||
           size == 0 as libc::c_int as libc::c_uint ||
           size & 1 as libc::c_int as libc::c_uint != 0 {
        DmaMgr_Error(req, 0 as *const libc::c_char,
                     b"ILLIGAL DMA-FUNCTION CALL\x00" as *const u8 as
                         *const libc::c_char,
                     b"\xe3\x83\x91\xe3\x83\xa9\xe3\x83\xa1\xe3\x83\xbc\xe3\x82\xbf\xe7\x95\xb0\xe5\xb8\xb8\xe3\x81\xa7\xe3\x81\x99\x00"
                         as *const u8 as *const libc::c_char);
    }
    (*req).vromAddr = vrom;
    (*req).dramAddr = ram as *mut libc::c_void;
    (*req).size = size;
    (*req).unk_14 = 0 as libc::c_int;
    (*req).notifyQueue = queue;
    (*req).notifyMsg = msg;
    if sDmaMgrQueueFullLogged == 0 as libc::c_int &&
           sDmaMgrMsgQueue.validCount >= sDmaMgrMsgQueue.msgCount {
        sDmaMgrQueueFullLogged += 1;
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"dmaEntryMsgQ\xe3\x81\x8c\xe4\xb8\x80\xe6\x9d\xaf\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\xe3\x82\xad\xe3\x83\xa5\xe3\x83\xbc\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xae\xe5\x86\x8d\xe6\xa4\x9c\xe8\xa8\x8e\xe3\x82\x92\xe3\x81\x8a\xe3\x81\x99\xe3\x81\x99\xe3\x82\x81\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\x00"
                         as *const u8 as *const libc::c_char);
        LogUtils_LogThreadId(b"../z_std_dma.c\x00" as *const u8 as
                                 *const libc::c_char, 952 as libc::c_int);
        osSyncPrintf(b"(sizeof(dmaEntryMsgBufs) / sizeof(dmaEntryMsgBufs[0])) = %d\n\x00"
                         as *const u8 as *const libc::c_char,
                     (::std::mem::size_of::<[OSMesg; 32]>() as
                          libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                          as libc::c_ulong) as
                         s32);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    osSendMesg(&mut sDmaMgrMsgQueue, req as OSMesg, 1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_SendRequest0(mut ram: u32_0, mut vrom: u32_0,
                                             mut size: u32_0) -> s32 {
    let mut req: DmaRequest =
        DmaRequest{vromAddr: 0,
                   dramAddr: 0 as *mut libc::c_void,
                   size: 0,
                   filename: 0 as *const libc::c_char,
                   line: 0,
                   unk_14: 0,
                   notifyQueue: 0 as *mut OSMesgQueue,
                   notifyMsg: 0 as *mut libc::c_void,};
    let mut queue: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut ret: s32 = 0;
    osCreateMesgQueue(&mut queue, &mut msg, 1 as libc::c_int);
    ret =
        DmaMgr_SendRequestImpl(&mut req, ram, vrom, size,
                               0 as libc::c_int as u32_0, &mut queue,
                               0 as *mut libc::c_void);
    if ret == -(1 as libc::c_int) { return ret }
    osRecvMesg(&mut queue, 0 as *mut OSMesg, 1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_Init() {
    let mut name: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut idx: s32 = 0;
    let mut iter: *mut DmaEntry = 0 as *mut DmaEntry;
    DmaMgr_DmaRomToRam(_dmadataSegmentRomStart.as_mut_ptr() as u32_0,
                       _dmadataSegmentStart.as_mut_ptr() as u32_0,
                       _dmadataSegmentRomEnd.as_mut_ptr().wrapping_offset_from(_dmadataSegmentRomStart.as_mut_ptr())
                           as libc::c_int as u32_0);
    osSyncPrintf(b"dma_rom_ad[]\n\x00" as *const u8 as *const libc::c_char);
    sDmaMgrDataExistError = 0 as libc::c_int as u32_0;
    name = sDmaMgrFileNames.as_mut_ptr();
    iter = gDmaDataTable.as_mut_ptr();
    idx = 0 as libc::c_int;
    while (*iter).vromEnd != 0 as libc::c_int as libc::c_uint {
        if (*iter).romEnd != 0 as libc::c_int as libc::c_uint {
            sDmaMgrDataExistError = 1 as libc::c_int as u32_0
        }
        osSyncPrintf(b"%3d %08x %08x %08x %08x %08x %c %s\n\x00" as *const u8
                         as *const libc::c_char, idx, (*iter).vromStart,
                     (*iter).vromEnd, (*iter).romStart, (*iter).romEnd,
                     if (*iter).romEnd != 0 as libc::c_int as libc::c_uint {
                         (*iter).romEnd.wrapping_sub((*iter).romStart)
                     } else {
                         (*iter).vromEnd.wrapping_sub((*iter).vromStart)
                     },
                     if (if (*iter).romEnd != 0 as libc::c_int as libc::c_uint
                            {
                             (*iter).romEnd.wrapping_sub((*iter).romStart)
                         } else { 0 as libc::c_int as libc::c_uint }) >
                            0x10000 as libc::c_int as libc::c_uint {
                         '*' as i32
                     } else { ' ' as i32 },
                     if !name.is_null() {
                         *name
                     } else { b"\x00" as *const u8 as *const libc::c_char });
        idx += 1;
        iter = iter.offset(1);
        if !name.is_null() { name = name.offset(1) }
    }
    if _bootSegmentRomStart.as_mut_ptr() as u32_0 !=
           gDmaDataTable[0 as libc::c_int as usize].vromEnd {
        osSyncPrintf(b"_bootSegmentRomStart(%08x) != dma_rom_ad[0].rom_b(%08x)\n\x00"
                         as *const u8 as *const libc::c_char,
                     _bootSegmentRomStart.as_mut_ptr(),
                     gDmaDataTable[0 as libc::c_int as usize].vromEnd);
        Fault_AddHungupAndCrash(b"../z_std_dma.c\x00" as *const u8 as
                                    *const libc::c_char,
                                1055 as libc::c_int as u32_0);
    }
    osCreateMesgQueue(&mut sDmaMgrMsgQueue, sDmaMgrMsgs.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 32]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
    StackCheck_Init(&mut sDmaMgrStackInfo,
                    sDmaMgrStack.as_mut_ptr() as *mut libc::c_void,
                    sDmaMgrStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1280]>()
                                                         as libc::c_ulong as
                                                         isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x100 as libc::c_int,
                    b"dmamgr\x00" as *const u8 as *const libc::c_char);
    osCreateThread(&mut sDmaMgrThread, 0x12 as libc::c_int,
                   Some(DmaMgr_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   0 as *mut libc::c_void,
                   sDmaMgrStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1280]>()
                                                        as libc::c_ulong as
                                                        isize) as
                       *mut libc::c_void, 16 as libc::c_int);
    osStartThread(&mut sDmaMgrThread);
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_SendRequest2(mut req: *mut DmaRequest,
                                             mut ram: u32_0, mut vrom: u32_0,
                                             mut size: u32_0, mut unk5: u32_0,
                                             mut queue: *mut OSMesgQueue,
                                             mut msg: OSMesg,
                                             mut file: *const libc::c_char,
                                             mut line: s32) -> s32 {
    (*req).filename = file;
    (*req).line = line;
    DmaMgr_SendRequestImpl(req, ram, vrom, size, unk5, queue, msg);
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn DmaMgr_SendRequest1(mut ram0: *mut libc::c_void,
                                             mut vrom: u32_0, mut size: u32_0,
                                             mut file: *const libc::c_char,
                                             mut line: s32) -> s32 {
    let mut req: DmaRequest =
        DmaRequest{vromAddr: 0,
                   dramAddr: 0 as *mut libc::c_void,
                   size: 0,
                   filename: 0 as *const libc::c_char,
                   line: 0,
                   unk_14: 0,
                   notifyQueue: 0 as *mut OSMesgQueue,
                   notifyMsg: 0 as *mut libc::c_void,};
    let mut ret: s32 = 0;
    let mut queue: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut ram: u32_0 = ram0 as u32_0;
    req.filename = file;
    req.line = line;
    osCreateMesgQueue(&mut queue, &mut msg, 1 as libc::c_int);
    ret =
        DmaMgr_SendRequestImpl(&mut req, ram, vrom, size,
                               0 as libc::c_int as u32_0, &mut queue,
                               0 as OSMesg);
    if ret == -(1 as libc::c_int) { return ret }
    osRecvMesg(&mut queue, 0 as *mut OSMesg, 1 as libc::c_int);
    return 0 as libc::c_int;
}
