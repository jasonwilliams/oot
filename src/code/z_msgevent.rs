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
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn Sched_SendEntryMsg(sc: *mut SchedContext);
    #[no_mangle]
    static mut gSchedContext: SchedContext;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
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
#[no_mangle]
pub unsafe extern "C" fn MsgEvent_SendNullTask() {
    let mut pad: [s32; 4] = [0; 4];
    let mut task: OSScTask =
        OSScTask{next: 0 as *mut OSScTask,
                 state: 0,
                 flags: 0,
                 framebuffer: 0 as *mut CfbInfo,
                 list:
                     OSTask{t:
                                OSTask_t{type_0: 0,
                                         flags: 0,
                                         ucode_boot: 0 as *mut u64_0,
                                         ucode_boot_size: 0,
                                         ucode: 0 as *mut u64_0,
                                         ucode_size: 0,
                                         ucode_data: 0 as *mut u64_0,
                                         ucode_data_size: 0,
                                         dram_stack: 0 as *mut u64_0,
                                         dram_stack_size: 0,
                                         output_buff: 0 as *mut u64_0,
                                         output_buff_size: 0 as *mut u64_0,
                                         data_ptr: 0 as *mut u64_0,
                                         data_size: 0,
                                         yield_data_ptr: 0 as *mut u64_0,
                                         yield_data_size: 0,},},
                 msgQ: 0 as *mut OSMesgQueue,
                 msg: 0 as *mut libc::c_void,};
    let mut queue: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut pad2: [u32_0; 1] = [0; 1];
    task.next = 0 as *mut OSScTask;
    task.flags = 0x3 as libc::c_int as u32_0;
    task.msgQ = &mut queue;
    task.msg = 0 as *mut libc::c_void;
    task.framebuffer = 0 as *mut CfbInfo;
    task.list.t.type_0 = 0 as libc::c_int as u32_0;
    osCreateMesgQueue(task.msgQ, &mut msg, 1 as libc::c_int);
    osSendMesg(&mut gSchedContext.cmdQ, &mut task as *mut OSScTask as OSMesg,
               1 as libc::c_int);
    Sched_SendEntryMsg(&mut gSchedContext);
    osRecvMesg(&mut queue, 0 as *mut OSMesg, 1 as libc::c_int);
}
