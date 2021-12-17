#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn PadMgr_LockSerialMesgQueue(padmgr: *mut PadMgr) -> *mut OSMesgQueue;
    #[no_mangle]
    fn PadMgr_UnlockSerialMesgQueue(padmgr: *mut PadMgr,
                                    ctrlrqueue: *mut OSMesgQueue);
    #[no_mangle]
    static mut gPadMgr: PadMgr;
    #[no_mangle]
    fn osPfsFreeBlocks(pfs: *mut OSPfs, leftoverBytes: *mut s32) -> s32;
    #[no_mangle]
    fn osPfsInitPak(mq: *mut OSMesgQueue, pfs: *mut OSPfs, channel: s32)
     -> s32;
    #[no_mangle]
    fn osPfsFindFile(pfs: *mut OSPfs, companyCode: u16_0, gameCode: u32_0,
                     gameName: *mut u8_0, extName: *mut u8_0,
                     fileNo: *mut s32) -> s32;
    #[no_mangle]
    fn osPfsReadWriteFile(pfs: *mut OSPfs, fileNo: s32, flag: u8_0,
                          offset: s32, size: s32, data: *mut u8_0) -> s32;
    #[no_mangle]
    fn osPfsAllocateFile(pfs: *mut OSPfs, companyCode: u16_0, gameCode: u32_0,
                         gameName: *mut u8_0, extName: *mut u8_0, length: s32,
                         fileNo: *mut s32) -> s32;
    #[no_mangle]
    fn osPfsDeleteFile(pfs: *mut OSPfs, companyCode: u16_0, gameCode: u32_0,
                       gameName: *mut u8_0, extName: *mut u8_0) -> s32;
    #[no_mangle]
    fn osPfsFileState(pfs: *mut OSPfs, fileNo: s32, state: *mut OSPfsState)
     -> s32;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type vu8 = u8_0;
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
pub struct OSTimer {
    pub next: *mut OSTimer,
    pub prev: *mut OSTimer,
    pub interval: OSTime,
    pub value: OSTime,
    pub mq: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSContStatus {
    pub type_0: u16_0,
    pub status: u8_0,
    pub errno: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSContPad {
    pub button: u16_0,
    pub stick_x: s8,
    pub stick_y: s8,
    pub errno: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSPfs {
    pub status: s32,
    pub queue: *mut OSMesgQueue,
    pub channel: s32,
    pub id: [u8_0; 32],
    pub label: [u8_0; 32],
    pub version: s32,
    pub dir_size: s32,
    pub inode_table: s32,
    pub minode_table: s32,
    pub dir_table: s32,
    pub inodeStartPage: s32,
    pub banks: u8_0,
    pub activebank: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSPfsState {
    pub file_size: u32_0,
    pub game_code: u32_0,
    pub company_code: u16_0,
    pub ext_name: [libc::c_char; 4],
    pub game_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSScMsg {
    pub type_0: s16,
    pub misc: [libc::c_char; 30],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqMgrClient {
    pub prev: *mut IrqMgrClient,
    pub queue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqMgr {
    pub retraceMsg: OSScMsg,
    pub prenmiMsg: OSScMsg,
    pub nmiMsg: OSScMsg,
    pub queue: OSMesgQueue,
    pub msgBuf: [OSMesg; 8],
    pub thread: OSThread,
    pub clients: *mut IrqMgrClient,
    pub resetStatus: u8_0,
    pub resetTime: OSTime,
    pub timer: OSTimer,
    pub retraceTime: OSTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PadMgr {
    pub padStatus: [OSContStatus; 4],
    pub serialMsgBuf: [OSMesg; 1],
    pub lockMsgBuf: [OSMesg; 1],
    pub interruptMsgBuf: [OSMesg; 4],
    pub serialMsgQ: OSMesgQueue,
    pub lockMsgQ: OSMesgQueue,
    pub interruptMsgQ: OSMesgQueue,
    pub irqClient: IrqMgrClient,
    pub irqMgr: *mut IrqMgr,
    pub thread: OSThread,
    pub inputs: [Input; 4],
    pub pads: [OSContPad; 4],
    pub validCtrlrsMask: vu8,
    pub nControllers: u8_0,
    pub ctrlrIsConnected: [u8_0; 4],
    pub pakType: [u8_0; 4],
    pub rumbleEnable: [vu8; 4],
    pub rumbleCounter: [u8_0; 4],
    pub pfs: [OSPfs; 4],
    pub rumbleOffFrames: vu8,
    pub rumbleOnFrames: vu8,
    pub preNMIShutdown: u8_0,
    pub retraceCallback: Option<unsafe extern "C" fn(_: *mut PadMgr, _: s32)
                                    -> ()>,
    pub retraceCallbackValue: u32_0,
}
#[no_mangle]
pub static mut sMempakPfsHandle: OSPfs =
    OSPfs{status: 0,
          queue: 0 as *const OSMesgQueue as *mut OSMesgQueue,
          channel: 0,
          id: [0; 32],
          label: [0; 32],
          version: 0,
          dir_size: 0,
          inode_table: 0,
          minode_table: 0,
          dir_table: 0,
          inodeStartPage: 0,
          banks: 0,
          activebank: 0,};
#[no_mangle]
pub static mut sMempakFreeBytes: s32 = 0;
#[no_mangle]
pub static mut sMempakFiles: [s32; 10] = [0; 10];
#[no_mangle]
pub static mut sMempakCompanyCode: u16_0 = 1 as libc::c_int as u16_0;
#[no_mangle]
pub static mut sMempakGameCode: u32_0 = 1 as libc::c_int as u32_0;
// "ZELDA DEMO TOOL "
#[no_mangle]
pub static mut sMempakGameName: [u8_0; 16] =
    [0x33 as libc::c_int as u8_0, 0x1e as libc::c_int as u8_0,
     0x25 as libc::c_int as u8_0, 0x1d as libc::c_int as u8_0,
     0x1a as libc::c_int as u8_0, 0xf as libc::c_int as u8_0,
     0x1d as libc::c_int as u8_0, 0x1e as libc::c_int as u8_0,
     0x26 as libc::c_int as u8_0, 0x28 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0x2d as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0x28 as libc::c_int as u8_0,
     0x25 as libc::c_int as u8_0, 0xf as libc::c_int as u8_0];
#[no_mangle]
pub static mut sMempakExtName: [u8_0; 8] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn Mempak_Init(mut controllerNb: s32) -> s32 {
    let mut mq: *mut OSMesgQueue = 0 as *mut OSMesgQueue;
    let mut pad: s32 = 0;
    let mut ret: s32 = 0 as libc::c_int;
    mq = PadMgr_LockSerialMesgQueue(&mut gPadMgr);
    if osPfsInitPak(mq, &mut sMempakPfsHandle, controllerNb) == 0 {
        ret = 1 as libc::c_int
    }
    osPfsFreeBlocks(&mut sMempakPfsHandle, &mut sMempakFreeBytes);
    PadMgr_UnlockSerialMesgQueue(&mut gPadMgr, mq);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Mempak_GetFreeBytes(mut controllerNb: s32) -> s32 {
    return sMempakFreeBytes;
}
#[no_mangle]
pub unsafe extern "C" fn Mempak_FindFile(mut controllerNb: s32,
                                         mut start: libc::c_char,
                                         mut end: libc::c_char) -> s32 {
    let mut mq: *mut OSMesgQueue = 0 as *mut OSMesgQueue;
    let mut error: s32 = 0;
    let mut idx: libc::c_char = 0;
    let mut bit: u32_0 = 1 as libc::c_int as u32_0;
    let mut flag: s32 = 0 as libc::c_int;
    mq = PadMgr_LockSerialMesgQueue(&mut gPadMgr);
    idx = start;
    while idx as libc::c_int <= end as libc::c_int {
        sMempakExtName[0 as libc::c_int as usize] =
            (idx as libc::c_int - 0x27 as libc::c_int) as u8_0;
        error =
            osPfsFindFile(&mut sMempakPfsHandle, sMempakCompanyCode,
                          sMempakGameCode, sMempakGameName.as_mut_ptr(),
                          sMempakExtName.as_mut_ptr(),
                          &mut *sMempakFiles.as_mut_ptr().offset((idx as
                                                                      libc::c_int
                                                                      -
                                                                      'A' as
                                                                          i32)
                                                                     as
                                                                     isize));
        if error == 0 as libc::c_int {
            flag = (flag as libc::c_uint | bit) as s32
        } else {
            sMempakFiles[(idx as libc::c_int - 'A' as i32) as usize] =
                -(1 as libc::c_int)
        }
        bit <<= 1 as libc::c_int;
        osSyncPrintf(b"mempak: find \'%c\' (%d)\n\x00" as *const u8 as
                         *const libc::c_char, idx as libc::c_int, error);
        idx += 1
    }
    PadMgr_UnlockSerialMesgQueue(&mut gPadMgr, mq);
    osSyncPrintf(b"mempak: find \'%c\' - \'%c\' %02x\n\x00" as *const u8 as
                     *const libc::c_char, start as libc::c_int,
                 end as libc::c_int, flag);
    return flag;
}
#[no_mangle]
pub unsafe extern "C" fn Mempak_Write(mut controllerNb: s32,
                                      mut idx: libc::c_char,
                                      mut buffer: *mut libc::c_void,
                                      mut offset: s32, mut size: s32) -> s32 {
    let mut mq: *mut OSMesgQueue = 0 as *mut OSMesgQueue;
    let mut error: s32 = 0;
    let mut ret: s32 = 0 as libc::c_int;
    let mut pad: s32 = 0;
    mq = PadMgr_LockSerialMesgQueue(&mut gPadMgr);
    if size < sMempakFreeBytes {
        error =
            osPfsReadWriteFile(&mut sMempakPfsHandle,
                               sMempakFiles[(idx as libc::c_int - 'A' as i32)
                                                as usize],
                               1 as libc::c_int as u8_0, offset, size,
                               buffer as *mut u8_0);
        if error == 0 as libc::c_int { ret = 1 as libc::c_int }
        osSyncPrintf(b"mempak: write %d byte \'%c\' (%d)->%d\n\x00" as
                         *const u8 as *const libc::c_char, size,
                     idx as libc::c_int,
                     sMempakFiles[(idx as libc::c_int - 'A' as i32) as usize],
                     error);
    }
    PadMgr_UnlockSerialMesgQueue(&mut gPadMgr, mq);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Mempak_Read(mut controllerNb: s32,
                                     mut idx: libc::c_char,
                                     mut buffer: *mut libc::c_void,
                                     mut offset: s32, mut size: s32) -> s32 {
    let mut mq: *mut OSMesgQueue = 0 as *mut OSMesgQueue;
    let mut error: s32 = 0;
    let mut ret: s32 = 0 as libc::c_int;
    let mut pad: s32 = 0;
    mq = PadMgr_LockSerialMesgQueue(&mut gPadMgr);
    if size < sMempakFreeBytes {
        error =
            osPfsReadWriteFile(&mut sMempakPfsHandle,
                               sMempakFiles[(idx as libc::c_int - 'A' as i32)
                                                as usize],
                               0 as libc::c_int as u8_0, offset, size,
                               buffer as *mut u8_0);
        if error == 0 as libc::c_int { ret = 1 as libc::c_int }
        osSyncPrintf(b"mempak: read %d byte \'%c\' (%d)<-%d\n\x00" as
                         *const u8 as *const libc::c_char, size,
                     idx as libc::c_int,
                     sMempakFiles[(idx as libc::c_int - 'A' as i32) as usize],
                     error);
    }
    PadMgr_UnlockSerialMesgQueue(&mut gPadMgr, mq);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Mempak_Alloc(mut controllerNb: s32,
                                      mut idx: *mut libc::c_char,
                                      mut size: s32) -> s32 {
    let mut mq: *mut OSMesgQueue = 0 as *mut OSMesgQueue;
    let mut error: s32 = 0;
    let mut ret: s32 = 0 as libc::c_int;
    let mut i: s32 = 0;
    let mut pad: s32 = 0;
    mq = PadMgr_LockSerialMesgQueue(&mut gPadMgr);
    if *idx as libc::c_int >= 'A' as i32 && (*idx as libc::c_int) < 'L' as i32
       {
        sMempakExtName[0 as libc::c_int as usize] =
            (*idx as libc::c_int - 0x27 as libc::c_int) as u8_0;
        if -(1 as libc::c_int) ==
               sMempakFiles[(*idx as libc::c_int - 'A' as i32) as usize] {
            error =
                osPfsAllocateFile(&mut sMempakPfsHandle, sMempakCompanyCode,
                                  sMempakGameCode,
                                  sMempakGameName.as_mut_ptr(),
                                  sMempakExtName.as_mut_ptr(), size,
                                  &mut *sMempakFiles.as_mut_ptr().offset((*idx
                                                                              as
                                                                              libc::c_int
                                                                              -
                                                                              'A'
                                                                                  as
                                                                                  i32)
                                                                             as
                                                                             isize));
            if error == 0 as libc::c_int { ret = 1 as libc::c_int }
            osSyncPrintf(b"mempak: alloc %d byte \'%c\' (%d)\n\x00" as
                             *const u8 as *const libc::c_char, size,
                         *idx as libc::c_int, error);
        } else {
            sMempakExtName[0 as libc::c_int as usize] =
                (*idx as libc::c_int - 0x27 as libc::c_int) as u8_0;
            if osPfsDeleteFile(&mut sMempakPfsHandle, sMempakCompanyCode,
                               sMempakGameCode, sMempakGameName.as_mut_ptr(),
                               sMempakExtName.as_mut_ptr()) ==
                   0 as libc::c_int {
                ret = 1 as libc::c_int
            }
            error =
                osPfsAllocateFile(&mut sMempakPfsHandle, sMempakCompanyCode,
                                  sMempakGameCode,
                                  sMempakGameName.as_mut_ptr(),
                                  sMempakExtName.as_mut_ptr(), size,
                                  &mut *sMempakFiles.as_mut_ptr().offset((*idx
                                                                              as
                                                                              libc::c_int
                                                                              -
                                                                              'A'
                                                                                  as
                                                                                  i32)
                                                                             as
                                                                             isize));
            if error == 0 as libc::c_int { ret |= 1 as libc::c_int }
            osSyncPrintf(b"mempak: resize %d byte \'%c\' (%d)\n\x00" as
                             *const u8 as *const libc::c_char, size,
                         *idx as libc::c_int, error);
        }
    } else {
        i = 0 as libc::c_int;
        while i <
                  (::std::mem::size_of::<[s32; 10]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<s32>()
                                                       as libc::c_ulong) as
                      s32 {
            if sMempakFiles[i as usize] == -(1 as libc::c_int) { break ; }
            i += 1
        }
        *idx = (i + 'A' as i32) as libc::c_char;
        sMempakExtName[0 as libc::c_int as usize] =
            (*idx as libc::c_int - 0x27 as libc::c_int) as u8_0;
        error =
            osPfsAllocateFile(&mut sMempakPfsHandle, sMempakCompanyCode,
                              sMempakGameCode, sMempakGameName.as_mut_ptr(),
                              sMempakExtName.as_mut_ptr(), size,
                              &mut *sMempakFiles.as_mut_ptr().offset(i as
                                                                         isize));
        osSyncPrintf(b"mempak: alloc %d byte \'%c\' (%d) with search\n\x00" as
                         *const u8 as *const libc::c_char, size,
                     *idx as libc::c_int, error);
        if error == 0 as libc::c_int { ret = 1 as libc::c_int }
    }
    PadMgr_UnlockSerialMesgQueue(&mut gPadMgr, mq);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Mempak_DeleteFile(mut controllerNb: s32,
                                           mut idx: libc::c_char) -> s32 {
    let mut mq: *mut OSMesgQueue = 0 as *mut OSMesgQueue;
    let mut error: s32 = 0;
    let mut ret: s32 = 0 as libc::c_int;
    mq = PadMgr_LockSerialMesgQueue(&mut gPadMgr);
    sMempakExtName[0 as libc::c_int as usize] =
        (idx as libc::c_int - 0x27 as libc::c_int) as u8_0;
    error =
        osPfsDeleteFile(&mut sMempakPfsHandle, sMempakCompanyCode,
                        sMempakGameCode, sMempakGameName.as_mut_ptr(),
                        sMempakExtName.as_mut_ptr());
    if error == 0 as libc::c_int { ret = 1 as libc::c_int }
    osSyncPrintf(b"mempak: delete \'%c\' (%d)\n\x00" as *const u8 as
                     *const libc::c_char, idx as libc::c_int, error);
    PadMgr_UnlockSerialMesgQueue(&mut gPadMgr, mq);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Mempak_GetFileSize(mut controllerNb: s32,
                                            mut idx: libc::c_char) -> s32 {
    let mut mq: *mut OSMesgQueue = PadMgr_LockSerialMesgQueue(&mut gPadMgr);
    let mut state: OSPfsState =
        OSPfsState{file_size: 0,
                   game_code: 0,
                   company_code: 0,
                   ext_name: [0; 4],
                   game_name: [0; 16],};
    let mut error: s32 =
        osPfsFileState(&mut sMempakPfsHandle,
                       sMempakFiles[(idx as libc::c_int - 'A' as i32) as
                                        usize], &mut state);
    let mut pad: s32 = 0;
    PadMgr_UnlockSerialMesgQueue(&mut gPadMgr, mq);
    if error != 0 as libc::c_int { return 0 as libc::c_int }
    return state.file_size as s32;
}
