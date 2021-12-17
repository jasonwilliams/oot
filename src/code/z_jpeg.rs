#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osInvalDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn MsgEvent_SendNullTask();
    #[no_mangle]
    fn Sched_SendEntryMsg(sc: *mut SchedContext);
    #[no_mangle]
    fn SysUcode_GetUCodeBoot() -> u32_0;
    #[no_mangle]
    fn SysUcode_GetUCodeBootSize() -> u32_0;
    #[no_mangle]
    static mut gSchedContext: SchedContext;
    #[no_mangle]
    static mut gJpegUCodeData: [u64_0; 0];
    #[no_mangle]
    static mut gJpegUCode: [u64_0; 0];
    #[no_mangle]
    fn JpegDecoder_Decode(decoder: *mut JpegDecoder, mcuBuff: *mut u16_0,
                          count: s32, isFollowing: u8_0,
                          state: *mut JpegDecoderState) -> s32;
    #[no_mangle]
    fn JpegUtils_ProcessHuffmanTable(dht: *mut u8_0,
                                     ht: *mut JpegHuffmanTable,
                                     codesLengths: *mut u8_0,
                                     codes: *mut u16_0, count: u8_0) -> u32_0;
    #[no_mangle]
    fn JpegUtils_ProcessQuantizationTable(dqt: *mut u8_0,
                                          qt: *mut JpegQuantizationTable,
                                          count: u8_0);
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
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
pub struct OSViCommonRegs {
    pub ctrl: u32_0,
    pub width: u32_0,
    pub burst: u32_0,
    pub vSync: u32_0,
    pub hSync: u32_0,
    pub leap: u32_0,
    pub hStart: u32_0,
    pub xScale: u32_0,
    pub vCurrent: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViFieldRegs {
    pub origin: u32_0,
    pub yScale: u32_0,
    pub vStart: u32_0,
    pub vBurst: u32_0,
    pub vIntr: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViMode {
    pub type_0: u8_0,
    pub comRegs: OSViCommonRegs,
    pub fldRegs: [OSViFieldRegs; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSScTask {
    pub next: *mut OSScTask,
    pub state: u32_0,
    pub flags: u32_0,
    pub framebuffer: *mut CfbInfo,
    pub list: OSTask,
    pub msgQ: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CfbInfo {
    pub fb1: *mut u16_0,
    pub swapBuffer: *mut u16_0,
    pub viMode: *mut OSViMode,
    pub features: u32_0,
    pub unk_10: u8_0,
    pub updateRate: s8,
    pub updateRate2: s8,
    pub unk_13: u8_0,
    pub xScale: f32_0,
    pub yScale: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqMgrClient {
    pub prev: *mut IrqMgrClient,
    pub queue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SchedContext {
    pub interruptQ: OSMesgQueue,
    pub intBuf: [OSMesg; 8],
    pub cmdQ: OSMesgQueue,
    pub cmdMsgBuf: [OSMesg; 8],
    pub thread: OSThread,
    pub audioListHead: *mut OSScTask,
    pub gfxListHead: *mut OSScTask,
    pub audioListTail: *mut OSScTask,
    pub gfxListTail: *mut OSScTask,
    pub curRSPTask: *mut OSScTask,
    pub curRDPTask: *mut OSScTask,
    pub retraceCnt: s32,
    pub doAudio: s32,
    pub curBuf: *mut CfbInfo,
    pub pendingSwapBuf1: *mut CfbInfo,
    pub pendingSwapBuf2: *mut CfbInfo,
    pub unk_24C: s32,
    pub irqClient: IrqMgrClient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegQuantizationTable {
    pub table: [u16_0; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegHuffmanTable {
    pub codeOffs: [u8_0; 16],
    pub codesA: [u16_0; 16],
    pub codesB: [u16_0; 16],
    pub symbols: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegTaskData {
    pub address: u32_0,
    pub mbCount: u32_0,
    pub mode: u32_0,
    pub qTableYPtr: u32_0,
    pub qTableUPtr: u32_0,
    pub qTableVPtr: u32_0,
    pub unk_18: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegWork {
    pub taskData: JpegTaskData,
    pub yieldData: [libc::c_char; 512],
    pub qTableY: JpegQuantizationTable,
    pub qTableU: JpegQuantizationTable,
    pub qTableV: JpegQuantizationTable,
    pub codesLengths: [u8_0; 272],
    pub codes: [u16_0; 264],
    pub data: [[u16_0; 384]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegDecoder {
    pub imageData: *mut libc::c_void,
    pub mode: u8_0,
    pub unk_05: u8_0,
    pub hTablePtrs: [*mut JpegHuffmanTable; 4],
    pub unk_18: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegContext {
    pub dqtCount: u8_0,
    pub dqtPtr: [*mut u8_0; 3],
    pub dhtCount: u8_0,
    pub dhtPtr: [*mut u8_0; 4],
    pub imageData: *mut libc::c_void,
    pub mode: u32_0,
    pub unk_2C: [libc::c_char; 4],
    pub scTask: OSScTask,
    pub unk_88: [libc::c_char; 16],
    pub mq: OSMesgQueue,
    pub msg: OSMesg,
    pub workBuf: *mut JpegWork,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegDecoderState {
    pub byteIdx: u32_0,
    pub bitIdx: u8_0,
    pub dontSkip: u8_0,
    pub curWord: u32_0,
    pub unk_0C: s16,
    pub unk_0E: s16,
    pub unk_10: s16,
}
/* *
 * Configures and schedules a JPEG decoder task and waits for it to finish.
 */
#[no_mangle]
pub unsafe extern "C" fn Jpeg_ScheduleDecoderTask(mut ctx: *mut JpegContext) {
    static mut sJpegTask: OSTask_t =
        unsafe {
            {
                let mut init =
                    OSTask_t{type_0: 4 as libc::c_int as u32_0,
                             flags: 0 as libc::c_int as u32_0,
                             ucode_boot: 0 as *const u64_0 as *mut u64_0,
                             ucode_boot_size: 0 as libc::c_int as u32_0,
                             ucode: gJpegUCode.as_ptr() as *mut _,
                             ucode_size: 0x1000 as libc::c_int as u32_0,
                             ucode_data: gJpegUCodeData.as_ptr() as *mut _,
                             ucode_data_size: 0x800 as libc::c_int as u32_0,
                             dram_stack: 0 as *const u64_0 as *mut u64_0,
                             dram_stack_size: 0 as libc::c_int as u32_0,
                             output_buff: 0 as *const u64_0 as *mut u64_0,
                             output_buff_size:
                                 0 as *const u64_0 as *mut u64_0,
                             data_ptr: 0 as *const u64_0 as *mut u64_0,
                             data_size:
                                 ::std::mem::size_of::<JpegTaskData>() as
                                     libc::c_ulong,
                             yield_data_ptr: 0 as *const u64_0 as *mut u64_0,
                             yield_data_size:
                                 0x200 as libc::c_int as
                                     u32_0,}; // osScKickEntryMsg
                init
            }
        };
    let mut workBuf: *mut JpegWork = (*ctx).workBuf;
    let mut pad: [s32; 2] = [0; 2];
    (*workBuf).taskData.address =
        (&mut (*workBuf).data as *mut [[u16_0; 384]; 4] as
             u32_0).wrapping_add(0x80000000 as libc::c_uint) as
            *mut libc::c_void as u32_0;
    (*workBuf).taskData.mode = (*ctx).mode;
    (*workBuf).taskData.mbCount = 4 as libc::c_int as u32_0;
    (*workBuf).taskData.qTableYPtr =
        (&mut (*workBuf).qTableY as *mut JpegQuantizationTable as
             u32_0).wrapping_add(0x80000000 as libc::c_uint) as
            *mut libc::c_void as u32_0;
    (*workBuf).taskData.qTableUPtr =
        (&mut (*workBuf).qTableU as *mut JpegQuantizationTable as
             u32_0).wrapping_add(0x80000000 as libc::c_uint) as
            *mut libc::c_void as u32_0;
    (*workBuf).taskData.qTableVPtr =
        (&mut (*workBuf).qTableV as *mut JpegQuantizationTable as
             u32_0).wrapping_add(0x80000000 as libc::c_uint) as
            *mut libc::c_void as u32_0;
    sJpegTask.flags = 0 as libc::c_int as u32_0;
    sJpegTask.ucode_boot = SysUcode_GetUCodeBoot() as *mut u64_0;
    sJpegTask.ucode_boot_size = SysUcode_GetUCodeBootSize();
    sJpegTask.yield_data_ptr =
        &mut (*workBuf).yieldData as *mut [libc::c_char; 512] as *mut u64_0;
    sJpegTask.data_ptr =
        &mut (*workBuf).taskData as *mut JpegTaskData as *mut u64_0;
    (*ctx).scTask.next = 0 as *mut OSScTask;
    (*ctx).scTask.flags = 0x2 as libc::c_int as u32_0;
    (*ctx).scTask.msgQ = &mut (*ctx).mq;
    (*ctx).scTask.msg = 0 as *mut libc::c_void;
    (*ctx).scTask.framebuffer = 0 as *mut CfbInfo;
    (*ctx).scTask.list.t = sJpegTask;
    osSendMesg(&mut gSchedContext.cmdQ,
               &mut (*ctx).scTask as *mut OSScTask as OSMesg,
               1 as libc::c_int);
    Sched_SendEntryMsg(&mut gSchedContext);
    osRecvMesg(&mut (*ctx).mq, 0 as *mut OSMesg, 1 as libc::c_int);
}
/* *
 * Copies a 16x16 block of decoded image data to the Z-buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn Jpeg_CopyToZbuffer(mut src: *mut u16_0,
                                            mut zbuffer: *mut u16_0,
                                            mut x: s32, mut y: s32) {
    let mut dst: *mut u16_0 =
        zbuffer.offset(((y * 320 as libc::c_int + x) * 16 as libc::c_int) as
                           isize);
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        *dst.offset(0 as libc::c_int as isize) =
            *src.offset(0 as libc::c_int as isize);
        *dst.offset(1 as libc::c_int as isize) =
            *src.offset(1 as libc::c_int as isize);
        *dst.offset(2 as libc::c_int as isize) =
            *src.offset(2 as libc::c_int as isize);
        *dst.offset(3 as libc::c_int as isize) =
            *src.offset(3 as libc::c_int as isize);
        *dst.offset(4 as libc::c_int as isize) =
            *src.offset(4 as libc::c_int as isize);
        *dst.offset(5 as libc::c_int as isize) =
            *src.offset(5 as libc::c_int as isize);
        *dst.offset(6 as libc::c_int as isize) =
            *src.offset(6 as libc::c_int as isize);
        *dst.offset(7 as libc::c_int as isize) =
            *src.offset(7 as libc::c_int as isize);
        *dst.offset(8 as libc::c_int as isize) =
            *src.offset(8 as libc::c_int as isize);
        *dst.offset(9 as libc::c_int as isize) =
            *src.offset(9 as libc::c_int as isize);
        *dst.offset(10 as libc::c_int as isize) =
            *src.offset(10 as libc::c_int as isize);
        *dst.offset(11 as libc::c_int as isize) =
            *src.offset(11 as libc::c_int as isize);
        *dst.offset(12 as libc::c_int as isize) =
            *src.offset(12 as libc::c_int as isize);
        *dst.offset(13 as libc::c_int as isize) =
            *src.offset(13 as libc::c_int as isize);
        *dst.offset(14 as libc::c_int as isize) =
            *src.offset(14 as libc::c_int as isize);
        *dst.offset(15 as libc::c_int as isize) =
            *src.offset(15 as libc::c_int as isize);
        src = src.offset(16 as libc::c_int as isize);
        dst = dst.offset(320 as libc::c_int as isize);
        i += 1
    };
}
/* *
 * Reads an u16 from a possibly unaligned address in memory.
 *
 * Replaces unaligned 16-bit reads with a pair of aligned reads, allowing for reading the possibly
 * unaligned values in JPEG header files.
 */
#[no_mangle]
pub unsafe extern "C" fn Jpeg_GetUnalignedU16(mut ptr: *mut u8_0) -> u16_0 {
    if ptr as u32_0 & 1 as libc::c_int as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        // Read the value normally if it's aligned to a 16-bit address.
        return *(ptr as *mut u16_0)
    } else {
        // Read unaligned values using two separate aligned memory accesses when it's not.
        return ((*(ptr.offset(-(1 as libc::c_int as isize)) as *mut u16_0) as
                     libc::c_int) << 8 as libc::c_int |
                    *(ptr.offset(1 as libc::c_int as isize) as *mut u16_0) as
                        libc::c_int >> 8 as libc::c_int) as u16_0
    };
}
/* *
 * Parses the markers in the JPEG file, storing information such as the pointer to the image data
 * in `ctx` for later processing.
 */
#[no_mangle]
pub unsafe extern "C" fn Jpeg_ParseMarkers(mut ptr: *mut u8_0,
                                           mut ctx: *mut JpegContext) {
    let mut exit: u32_0 = 0 as libc::c_int as u32_0;
    (*ctx).dqtCount = 0 as libc::c_int as u8_0;
    (*ctx).dhtCount = 0 as libc::c_int as u8_0;
    while !(exit != 0) {
        // 0xFF indicates the start of a JPEG marker, so look for the next.
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        if *fresh0 as libc::c_int == 0xff as libc::c_int {
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            match *fresh1 as libc::c_int {
                0 => { }
                216 => {
                    // Start of Image
                    osSyncPrintf(b"MARKER_SOI\n\x00" as *const u8 as
                                     *const libc::c_char);
                }
                224 => {
                    // Application marker for JFIF
                    osSyncPrintf(b"MARKER_APP0 %d\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
                225 => {
                    // Application marker for EXIF
                    osSyncPrintf(b"MARKER_APP1 %d\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
                226 => {
                    osSyncPrintf(b"MARKER_APP2 %d\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
                219 => {
                    // Define Quantization Table, stored for later processing
                    osSyncPrintf(b"MARKER_DQT %d %d %02x\n\x00" as *const u8
                                     as *const libc::c_char,
                                 (*ctx).dqtCount as libc::c_int,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int,
                                 *ptr.offset(2 as libc::c_int as isize) as
                                     libc::c_int);
                    let fresh2 = (*ctx).dqtCount;
                    (*ctx).dqtCount = (*ctx).dqtCount.wrapping_add(1);
                    (*ctx).dqtPtr[fresh2 as usize] =
                        ptr.offset(2 as libc::c_int as isize);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
                196 => {
                    // Define Huffman Table, stored for later processing
                    osSyncPrintf(b"MARKER_DHT %d %d %02x\n\x00" as *const u8
                                     as *const libc::c_char,
                                 (*ctx).dhtCount as libc::c_int,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int,
                                 *ptr.offset(2 as libc::c_int as isize) as
                                     libc::c_int);
                    let fresh3 = (*ctx).dhtCount;
                    (*ctx).dhtCount = (*ctx).dhtCount.wrapping_add(1);
                    (*ctx).dhtPtr[fresh3 as usize] =
                        ptr.offset(2 as libc::c_int as isize);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
                221 => {
                    // Define Restart Interval
                    osSyncPrintf(b"MARKER_DRI %d\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
                192 => {
                    // Start of Frame, stores important metadata of the image.
                    // Only used for extracting the sampling factors (ctx->mode).
                    osSyncPrintf(b"MARKER_SOF   %d \xe7\xb2\xbe\xe5\xba\xa6%02x \xe5\x9e\x82\xe7\x9b\xb4%d \xe6\xb0\xb4\xe5\xb9\xb3%d compo%02x (1:Y)%d (H0=2,V0=1(422) or 2(420))%02x (\xe9\x87\x8f\xe5\xad\x90\xe5\x8c\x96\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab)%02x (2:Cb)%d (H1=1,V1=1)%02x (\xe9\x87\x8f\xe5\xad\x90\xe5\x8c\x96\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab)%02x (3:Cr)%d (H2=1,V2=1)%02x (\xe9\x87\x8f\xe5\xad\x90\xe5\x8c\x96\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab)%02x\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int,
                                 *ptr.offset(2 as libc::c_int as isize) as
                                     libc::c_int,
                                 Jpeg_GetUnalignedU16(ptr.offset(3 as
                                                                     libc::c_int
                                                                     as
                                                                     isize))
                                     as libc::c_int,
                                 Jpeg_GetUnalignedU16(ptr.offset(5 as
                                                                     libc::c_int
                                                                     as
                                                                     isize))
                                     as libc::c_int,
                                 *ptr.offset(7 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(8 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(9 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(10 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(11 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(12 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(13 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(14 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(15 as libc::c_int as isize) as
                                     libc::c_int,
                                 *ptr.offset(16 as libc::c_int as isize) as
                                     libc::c_int);
                    if *ptr.offset(9 as libc::c_int as isize) as libc::c_int
                           == 0x21 as libc::c_int {
                        // component Y : V0 == 1
                        (*ctx).mode = 0 as libc::c_int as u32_0
                    } else if *ptr.offset(9 as libc::c_int as isize) as
                                  libc::c_int == 0x22 as libc::c_int {
                        // component Y : V0 == 2
                        (*ctx).mode = 2 as libc::c_int as u32_0
                    }
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
                218 => {
                    // Start of Scan marker, indicates the start of the image data.
                    osSyncPrintf(b"MARKER_SOS %d\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 Jpeg_GetUnalignedU16(ptr) as libc::c_int);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize);
                    (*ctx).imageData = ptr as *mut libc::c_void
                }
                217 => {
                    // End of Image
                    osSyncPrintf(b"MARKER_EOI\n\x00" as *const u8 as
                                     *const libc::c_char); // "Unknown marker"
                    exit = 1 as libc::c_int as u32_0
                }
                _ => {
                    osSyncPrintf(b"\xe3\x83\x9e\xe3\x83\xbc\xe3\x82\xab\xe3\x83\xbc\xe4\xb8\x8d\xe6\x98\x8e %02x\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 *ptr.offset(-(1 as libc::c_int) as isize) as
                                     libc::c_int);
                    ptr =
                        ptr.offset(Jpeg_GetUnalignedU16(ptr) as libc::c_int as
                                       isize)
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Jpeg_Decode(mut data: *mut libc::c_void,
                                     mut zbuffer: *mut libc::c_void,
                                     mut work: *mut libc::c_void,
                                     mut workSize: u32_0) -> s32 {
    let mut y: s32 = 0;
    let mut x: s32 = 0;
    let mut j: u32_0 = 0;
    let mut i: u32_0 = 0;
    let mut ctx: JpegContext =
        JpegContext{dqtCount: 0,
                    dqtPtr: [0 as *mut u8_0; 3],
                    dhtCount: 0,
                    dhtPtr: [0 as *mut u8_0; 4],
                    imageData: 0 as *mut libc::c_void,
                    mode: 0,
                    unk_2C: [0; 4],
                    scTask:
                        OSScTask{next: 0 as *mut OSScTask,
                                 state: 0,
                                 flags: 0,
                                 framebuffer: 0 as *mut CfbInfo,
                                 list:
                                     OSTask{t:
                                                OSTask_t{type_0: 0,
                                                         flags: 0,
                                                         ucode_boot:
                                                             0 as *mut u64_0,
                                                         ucode_boot_size: 0,
                                                         ucode:
                                                             0 as *mut u64_0,
                                                         ucode_size: 0,
                                                         ucode_data:
                                                             0 as *mut u64_0,
                                                         ucode_data_size: 0,
                                                         dram_stack:
                                                             0 as *mut u64_0,
                                                         dram_stack_size: 0,
                                                         output_buff:
                                                             0 as *mut u64_0,
                                                         output_buff_size:
                                                             0 as *mut u64_0,
                                                         data_ptr:
                                                             0 as *mut u64_0,
                                                         data_size: 0,
                                                         yield_data_ptr:
                                                             0 as *mut u64_0,
                                                         yield_data_size:
                                                             0,},},
                                 msgQ: 0 as *mut OSMesgQueue,
                                 msg: 0 as *mut libc::c_void,},
                    unk_88: [0; 16],
                    mq:
                        OSMesgQueue{mtqueue:
                                        0 as *const OSThread as *mut OSThread,
                                    fullqueue:
                                        0 as *const OSThread as *mut OSThread,
                                    validCount: 0,
                                    first: 0,
                                    msgCount: 0,
                                    msg: 0 as *const OSMesg as *mut OSMesg,},
                    msg: 0 as *mut libc::c_void,
                    workBuf: 0 as *mut JpegWork,};
    let mut hTables: [JpegHuffmanTable; 4] =
        [JpegHuffmanTable{codeOffs: [0; 16],
                          codesA: [0; 16],
                          codesB: [0; 16],
                          symbols: 0 as *mut u8_0,}; 4];
    let mut decoder: JpegDecoder =
        JpegDecoder{imageData: 0 as *mut libc::c_void,
                    mode: 0,
                    unk_05: 0,
                    hTablePtrs: [0 as *mut JpegHuffmanTable; 4],
                    unk_18: 0,};
    let mut state: JpegDecoderState =
        JpegDecoderState{byteIdx: 0,
                         bitIdx: 0,
                         dontSkip: 0,
                         curWord: 0,
                         unk_0C: 0,
                         unk_0E: 0,
                         unk_10: 0,};
    let mut workBuff: *mut JpegWork = 0 as *mut JpegWork;
    let mut diff: OSTime = 0;
    let mut time: OSTime = 0;
    let mut curTime: OSTime = 0;
    workBuff = work as *mut JpegWork;
    time = osGetTime();
    // (?) I guess MB_SIZE=0x180, PROC_OF_MBS=5 which means data is not a part of JpegWork
    if workSize >= ::std::mem::size_of::<JpegWork>() as libc::c_ulong {
    } else {
        __assert(b"worksize >= sizeof(JPEGWork) + MB_SIZE * (PROC_OF_MBS - 1)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../z_jpeg.c\x00" as *const u8 as *const libc::c_char,
                 527 as libc::c_int);
    };
    osCreateMesgQueue(&mut ctx.mq, &mut ctx.msg, 1 as libc::c_int);
    MsgEvent_SendNullTask();
    curTime = osGetTime();
    diff = curTime.wrapping_sub(time);
    time = curTime;
    // "Wait for synchronization of fifo buffer"
    osSyncPrintf(b"*** fifo\xe3\x83\x90\xe3\x83\x83\xe3\x83\x95\xe3\x82\xa1\xe3\x81\xae\xe5\x90\x8c\xe6\x9c\x9f\xe5\xbe\x85\xe3\x81\xa1 time = %6.3f ms ***\n\x00"
                     as *const u8 as *const libc::c_char,
                 (diff.wrapping_mul((1000000 as libc::c_longlong /
                                         15625 as libc::c_longlong) as
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
                                                                            libc::c_ulonglong)
                      as libc::c_float / 1000.0f32) as libc::c_double);
    ctx.workBuf = workBuff;
    Jpeg_ParseMarkers(data as *mut u8_0, &mut ctx);
    curTime = osGetTime();
    diff = curTime.wrapping_sub(time);
    time = curTime;
    // "Check markers for each segment"
    osSyncPrintf(b"*** \xe5\x90\x84\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae\xe3\x83\x9e\xe3\x83\xbc\xe3\x82\xab\xe3\x83\xbc\xe3\x81\xae\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf time = %6.3f ms ***\n\x00"
                     as *const u8 as *const libc::c_char,
                 (diff.wrapping_mul((1000000 as libc::c_longlong /
                                         15625 as libc::c_longlong) as
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
                                                                            libc::c_ulonglong)
                      as libc::c_float / 1000.0f32) as libc::c_double);
    match ctx.dqtCount as libc::c_int {
        1 => {
            JpegUtils_ProcessQuantizationTable(ctx.dqtPtr[0 as libc::c_int as
                                                              usize],
                                               &mut (*workBuff).qTableY,
                                               3 as libc::c_int as u8_0);
        }
        2 => {
            JpegUtils_ProcessQuantizationTable(ctx.dqtPtr[0 as libc::c_int as
                                                              usize],
                                               &mut (*workBuff).qTableY,
                                               1 as libc::c_int as u8_0);
            JpegUtils_ProcessQuantizationTable(ctx.dqtPtr[1 as libc::c_int as
                                                              usize],
                                               &mut (*workBuff).qTableU,
                                               1 as libc::c_int as u8_0);
            JpegUtils_ProcessQuantizationTable(ctx.dqtPtr[1 as libc::c_int as
                                                              usize],
                                               &mut (*workBuff).qTableV,
                                               1 as libc::c_int as u8_0);
        }
        3 => {
            JpegUtils_ProcessQuantizationTable(ctx.dqtPtr[0 as libc::c_int as
                                                              usize],
                                               &mut (*workBuff).qTableY,
                                               1 as libc::c_int as u8_0);
            JpegUtils_ProcessQuantizationTable(ctx.dqtPtr[1 as libc::c_int as
                                                              usize],
                                               &mut (*workBuff).qTableU,
                                               1 as libc::c_int as u8_0);
            JpegUtils_ProcessQuantizationTable(ctx.dqtPtr[2 as libc::c_int as
                                                              usize],
                                               &mut (*workBuff).qTableV,
                                               1 as libc::c_int as u8_0);
        }
        _ => { return -(1 as libc::c_int) }
    }
    curTime = osGetTime();
    diff = curTime.wrapping_sub(time);
    time = curTime;
    // "Create quantization table"
    osSyncPrintf(b"*** \xe9\x87\x8f\xe5\xad\x90\xe5\x8c\x96\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab\xe4\xbd\x9c\xe6\x88\x90 time = %6.3f ms ***\n\x00"
                     as *const u8 as *const libc::c_char,
                 (diff.wrapping_mul((1000000 as libc::c_longlong /
                                         15625 as libc::c_longlong) as
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
                                                                            libc::c_ulonglong)
                      as libc::c_float / 1000.0f32) as libc::c_double);
    match ctx.dhtCount as libc::c_int {
        1 => {
            if JpegUtils_ProcessHuffmanTable(ctx.dhtPtr[0 as libc::c_int as
                                                            usize],
                                             &mut *hTables.as_mut_ptr().offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                             (*workBuff).codesLengths.as_mut_ptr(),
                                             (*workBuff).codes.as_mut_ptr(),
                                             4 as libc::c_int as u8_0) != 0 {
                osSyncPrintf(b"Error : Cant\' make huffman table.\n\x00" as
                                 *const u8 as *const libc::c_char);
            }
        }
        4 => {
            if JpegUtils_ProcessHuffmanTable(ctx.dhtPtr[0 as libc::c_int as
                                                            usize],
                                             &mut *hTables.as_mut_ptr().offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                             (*workBuff).codesLengths.as_mut_ptr(),
                                             (*workBuff).codes.as_mut_ptr(),
                                             1 as libc::c_int as u8_0) != 0 {
                osSyncPrintf(b"Error : Cant\' make huffman table.\n\x00" as
                                 *const u8 as *const libc::c_char);
            }
            if JpegUtils_ProcessHuffmanTable(ctx.dhtPtr[1 as libc::c_int as
                                                            usize],
                                             &mut *hTables.as_mut_ptr().offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                             (*workBuff).codesLengths.as_mut_ptr(),
                                             (*workBuff).codes.as_mut_ptr(),
                                             1 as libc::c_int as u8_0) != 0 {
                osSyncPrintf(b"Error : Cant\' make huffman table.\n\x00" as
                                 *const u8 as *const libc::c_char);
            }
            if JpegUtils_ProcessHuffmanTable(ctx.dhtPtr[2 as libc::c_int as
                                                            usize],
                                             &mut *hTables.as_mut_ptr().offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                             (*workBuff).codesLengths.as_mut_ptr(),
                                             (*workBuff).codes.as_mut_ptr(),
                                             1 as libc::c_int as u8_0) != 0 {
                osSyncPrintf(b"Error : Cant\' make huffman table.\n\x00" as
                                 *const u8 as *const libc::c_char);
            }
            if JpegUtils_ProcessHuffmanTable(ctx.dhtPtr[3 as libc::c_int as
                                                            usize],
                                             &mut *hTables.as_mut_ptr().offset(3
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                             (*workBuff).codesLengths.as_mut_ptr(),
                                             (*workBuff).codes.as_mut_ptr(),
                                             1 as libc::c_int as u8_0) != 0 {
                osSyncPrintf(b"Error : Cant\' make huffman table.\n\x00" as
                                 *const u8 as *const libc::c_char);
            }
        }
        _ => { return -(1 as libc::c_int) }
    }
    curTime = osGetTime();
    diff = curTime.wrapping_sub(time);
    time = curTime;
    // "Huffman table creation"
    osSyncPrintf(b"*** \xe3\x83\x8f\xe3\x83\x95\xe3\x83\x9e\xe3\x83\xb3\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab\xe4\xbd\x9c\xe6\x88\x90 time = %6.3f ms ***\n\x00"
                     as *const u8 as *const libc::c_char,
                 (diff.wrapping_mul((1000000 as libc::c_longlong /
                                         15625 as libc::c_longlong) as
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
                                                                            libc::c_ulonglong)
                      as libc::c_float / 1000.0f32) as libc::c_double);
    decoder.imageData = ctx.imageData;
    decoder.mode = ctx.mode as u8_0;
    decoder.unk_05 = 2 as libc::c_int as u8_0;
    decoder.hTablePtrs[0 as libc::c_int as usize] =
        &mut *hTables.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut JpegHuffmanTable;
    decoder.hTablePtrs[1 as libc::c_int as usize] =
        &mut *hTables.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut JpegHuffmanTable;
    decoder.hTablePtrs[2 as libc::c_int as usize] =
        &mut *hTables.as_mut_ptr().offset(2 as libc::c_int as isize) as
            *mut JpegHuffmanTable;
    decoder.hTablePtrs[3 as libc::c_int as usize] =
        &mut *hTables.as_mut_ptr().offset(3 as libc::c_int as isize) as
            *mut JpegHuffmanTable;
    decoder.unk_18 = 0 as libc::c_int as u8_0;
    y = 0 as libc::c_int;
    x = y;
    i = 0 as libc::c_int as u32_0;
    while i < 300 as libc::c_int as libc::c_uint {
        if JpegDecoder_Decode(&mut decoder,
                              (*workBuff).data.as_mut_ptr() as *mut u16_0,
                              4 as libc::c_int,
                              (i != 0 as libc::c_int as libc::c_uint) as
                                  libc::c_int as u8_0, &mut state) != 0 {
            osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
            osSyncPrintf(b"Error : Can\'t decode jpeg\n\x00" as *const u8 as
                             *const libc::c_char);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        } else {
            Jpeg_ScheduleDecoderTask(&mut ctx);
            osInvalDCache(&mut (*workBuff).data as *mut [[u16_0; 384]; 4] as
                              *mut libc::c_void,
                          ::std::mem::size_of::<[u16_0; 384]>() as
                              libc::c_ulong as s32);
            j = 0 as libc::c_int as u32_0;
            while j <
                      (::std::mem::size_of::<[[u16_0; 384]; 4]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<[u16_0; 384]>()
                                                           as libc::c_ulong)
                          as s32 as libc::c_uint {
                Jpeg_CopyToZbuffer((*workBuff).data[j as usize].as_mut_ptr(),
                                   zbuffer as *mut u16_0, x, y);
                x += 1;
                if x >= 20 as libc::c_int { x = 0 as libc::c_int; y += 1 }
                j = j.wrapping_add(1)
            }
        }
        i =
            (i as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint)
                as u32_0 as u32_0
    }
    curTime = osGetTime();
    diff = curTime.wrapping_sub(time);
    time = curTime;
    // "Unfold & draw"
    osSyncPrintf(b"*** \xe5\xb1\x95\xe9\x96\x8b & \xe6\x8f\x8f\xe7\x94\xbb time = %6.3f ms ***\n\x00"
                     as *const u8 as *const libc::c_char,
                 (diff.wrapping_mul((1000000 as libc::c_longlong /
                                         15625 as libc::c_longlong) as
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
                                                                            libc::c_ulonglong)
                      as libc::c_float / 1000.0f32) as libc::c_double);
    return 0 as libc::c_int;
}
