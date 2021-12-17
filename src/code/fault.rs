#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn StackCheck_Init(entry: *mut StackEntry, stackTop: *mut libc::c_void,
                       stackBottom: *mut libc::c_void, initValue: u32_0,
                       minSpace: s32, name: *const libc::c_char);
    #[no_mangle]
    fn sprintf(dst: *mut libc::c_char, fmt: *const libc::c_char, _: ...)
     -> s32;
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osStopThread(thread: *mut OSThread);
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn __osCleanupThread();
    #[no_mangle]
    fn osDestroyThread(thread: *mut OSThread);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osViGetNextFramebuffer() -> *mut libc::c_void;
    #[no_mangle]
    fn osViBlack(active: u8_0);
    #[no_mangle]
    fn osGetThreadId(thread: *mut OSThread) -> OSId;
    #[no_mangle]
    fn osViSetMode(mode: *mut OSViMode);
    #[no_mangle]
    fn osSetEventMesg(e: OSEvent, mq: *mut OSMesgQueue, msg: OSMesg);
    #[no_mangle]
    fn osInvalICache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osViSwapBuffer(vaddr: *mut libc::c_void);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn osViSetSpecialFeatures(func: u32_0);
    #[no_mangle]
    fn __osSetFpcCsr(_: u32_0);
    #[no_mangle]
    fn __osGetFpcCsr() -> u32_0;
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn osViSetYScale(scale: f32_0);
    #[no_mangle]
    fn FaultDrawer_SetOsSyncPrintfEnabled(_: u32_0);
    #[no_mangle]
    fn FaultDrawer_DrawRecImpl(_: s32, _: s32, _: s32, _: s32, _: u16_0);
    #[no_mangle]
    fn FaultDrawer_SetForeColor(_: u16_0);
    #[no_mangle]
    fn FaultDrawer_SetBackColor(_: u16_0);
    #[no_mangle]
    fn FaultDrawer_SetCharPad(_: s8, _: s8);
    #[no_mangle]
    fn FaultDrawer_SetCursor(_: s32, _: s32);
    #[no_mangle]
    fn FaultDrawer_FillScreen();
    #[no_mangle]
    fn FaultDrawer_Printf(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FaultDrawer_DrawText(_: s32, _: s32, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FaultDrawer_SetDrawerFB(_: *mut libc::c_void, _: u16_0, _: u16_0);
    #[no_mangle]
    fn FaultDrawer_SetInputCallback(_: Option<unsafe extern "C" fn() -> ()>);
    #[no_mangle]
    fn FaultDrawer_SetDefault();
    #[no_mangle]
    fn osSetIntMask(_: OSIntMask) -> OSIntMask;
    #[no_mangle]
    fn Sleep_Cycles(cycles: OSTime);
    #[no_mangle]
    fn osStopTimer(timer: *mut OSTimer) -> s32;
    #[no_mangle]
    fn osSetTimer(timer: *mut OSTimer, countdown: OSTime, interval: OSTime,
                  mq: *mut OSMesgQueue, msg: OSMesg) -> s32;
    #[no_mangle]
    fn __osGetActiveQueue() -> *mut OSThread;
    #[no_mangle]
    fn osWritebackDCacheAll();
    #[no_mangle]
    static mut osMemSize: u32_0;
    #[no_mangle]
    static mut osViModeNtscLan1: OSViMode;
    #[no_mangle]
    fn __osGetCurrFaultedThread() -> *mut OSThread;
    #[no_mangle]
    static mut gFaultStruct: FaultThreadStruct;
    // bss
    #[no_mangle]
    static mut sFaultStructPtr: *mut FaultThreadStruct;
    #[no_mangle]
    static mut sFaultIsWaitingForInput: u8_0;
    #[no_mangle]
    static mut sFaultStack: [libc::c_char; 1536];
    #[no_mangle]
    static mut sFaultThreadInfo: StackEntry;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
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
pub type OSEvent = u32_0;
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
pub struct OSContPad {
    pub button: u16_0,
    pub stick_x: s8,
    pub stick_y: s8,
    pub errno: u8_0,
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
pub struct FaultClient {
    pub next: *mut FaultClient,
    pub callback: u32_0,
    pub param1: u32_0,
    pub param2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultAddrConvClient {
    pub next: *mut FaultAddrConvClient,
    pub callback: u32_0,
    pub param: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultClientContext {
    pub callback: Option<unsafe extern "C" fn(_: u32_0, _: u32_0) -> u32_0>,
    pub param0: u32_0,
    pub param1: u32_0,
    pub ret: u32_0,
    pub queue: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultThreadStruct {
    pub thread: OSThread,
    pub unk_1B0: [u8_0; 1536],
    pub queue: OSMesgQueue,
    pub msg: OSMesg,
    pub exitDebugger: u8_0,
    pub msgId: u8_0,
    pub faultHandlerEnabled: u8_0,
    pub faultActive: u8_0,
    pub faultedThread: *mut OSThread,
    pub padCallback: Option<unsafe extern "C" fn(_: *mut Input) -> ()>,
    pub clients: *mut FaultClient,
    pub addrConvClients: *mut FaultAddrConvClient,
    pub unk_7E0: [u8_0; 4],
    pub padInput: Input,
    pub colors: [u16_0; 36],
    pub fb: *mut libc::c_void,
    pub currClientThreadSp: u32_0,
    pub unk_84C: [u8_0; 4],
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
// data
#[no_mangle]
pub static mut sExceptionNames: [*const libc::c_char; 24] =
    [b"Interrupt\x00" as *const u8 as *const libc::c_char,
     b"TLB modification\x00" as *const u8 as *const libc::c_char,
     b"TLB exception on load\x00" as *const u8 as *const libc::c_char,
     b"TLB exception on store\x00" as *const u8 as *const libc::c_char,
     b"Address error on load\x00" as *const u8 as *const libc::c_char,
     b"Address error on store\x00" as *const u8 as *const libc::c_char,
     b"Bus error on inst.\x00" as *const u8 as *const libc::c_char,
     b"Bus error on data\x00" as *const u8 as *const libc::c_char,
     b"System call exception\x00" as *const u8 as *const libc::c_char,
     b"Breakpoint exception\x00" as *const u8 as *const libc::c_char,
     b"Reserved instruction\x00" as *const u8 as *const libc::c_char,
     b"Coprocessor unusable\x00" as *const u8 as *const libc::c_char,
     b"Arithmetic overflow\x00" as *const u8 as *const libc::c_char,
     b"Trap exception\x00" as *const u8 as *const libc::c_char,
     b"Virtual coherency on inst.\x00" as *const u8 as *const libc::c_char,
     b"Floating point exception\x00" as *const u8 as *const libc::c_char,
     b"Watchpoint exception\x00" as *const u8 as *const libc::c_char,
     b"Virtual coherency on data\x00" as *const u8 as *const libc::c_char,
     b"Unimplemented operation\x00" as *const u8 as *const libc::c_char,
     b"Invalid operation\x00" as *const u8 as *const libc::c_char,
     b"Division by zero\x00" as *const u8 as *const libc::c_char,
     b"Overflow\x00" as *const u8 as *const libc::c_char,
     b"Underflow\x00" as *const u8 as *const libc::c_char,
     b"Inexact operation\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn Fault_SleepImpl(mut duration: u32_0) {
    let mut value: u64_0 =
        ((duration as libc::c_longlong *
              (62500000 as libc::c_longlong *
                   3 as libc::c_int as libc::c_longlong /
                   4 as libc::c_int as libc::c_longlong)) as
             libc::c_ulonglong).wrapping_div(1000 as libc::c_ulonglong);
    Sleep_Cycles(value);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_ClientProcessThread(mut arg:
                                                       *mut libc::c_void) {
    let mut ctx: *mut FaultClientContext = arg as *mut FaultClientContext;
    if (*ctx).callback.is_some() {
        (*ctx).ret =
            (*ctx).callback.expect("non-null function pointer")((*ctx).param0,
                                                                (*ctx).param1)
    }
    if !(*ctx).queue.is_null() {
        osSendMesg((*ctx).queue, (*ctx).msg, 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_ProcessClientContext(mut ctx:
                                                        *mut FaultClientContext) {
    let mut queue: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut recMsg: OSMesg = 0 as *mut libc::c_void;
    let mut thread: *mut OSThread = 0 as *mut OSThread;
    let mut timer: OSTimer =
        OSTimer{next: 0 as *mut OSTimer,
                prev: 0 as *mut OSTimer,
                interval: 0,
                value: 0,
                mq: 0 as *mut OSMesgQueue,
                msg: 0 as *mut libc::c_void,};
    let mut timerMsgVal: u32_0 = 0;
    timerMsgVal = 666 as libc::c_int as u32_0;
    thread = 0 as *mut OSThread;
    osCreateMesgQueue(&mut queue, &mut msg, 1 as libc::c_int);
    (*ctx).queue = &mut queue;
    (*ctx).msg = 0 as *mut libc::c_void;
    if (*sFaultStructPtr).currClientThreadSp !=
           0 as libc::c_int as libc::c_uint {
        let mut fresh0 =
            ::std::vec::from_elem(0,
                                  ::std::mem::size_of::<OSThread>() as
                                      libc::c_ulong as usize);
        thread = fresh0.as_mut_ptr() as *mut OSThread;
        osCreateThread(thread, 2 as libc::c_int,
                       Some(Fault_ClientProcessThread as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()), ctx as *mut libc::c_void,
                       (*sFaultStructPtr).currClientThreadSp as
                           *mut libc::c_void,
                       127 as libc::c_int - 1 as libc::c_int);
        osStartThread(thread);
    } else { Fault_ClientProcessThread(ctx as *mut libc::c_void); }
    loop  {
        osSetTimer(&mut timer,
                   (1000000 as libc::c_int as
                        u64_0).wrapping_mul((62500000 as libc::c_longlong *
                                                 3 as libc::c_int as
                                                     libc::c_longlong /
                                                 4 as libc::c_int as
                                                     libc::c_longlong /
                                                 15625 as libc::c_longlong) as
                                                libc::c_ulonglong).wrapping_div((1000000
                                                                                     as
                                                                                     libc::c_longlong
                                                                                     /
                                                                                     15625
                                                                                         as
                                                                                         libc::c_longlong)
                                                                                    as
                                                                                    libc::c_ulonglong),
                   0 as libc::c_int as OSTime, &mut queue,
                   timerMsgVal as OSMesg);
        osRecvMesg(&mut queue, &mut recMsg, 1 as libc::c_int);
        if recMsg != 666 as libc::c_int as OSMesg { break ; }
        if !(sFaultIsWaitingForInput == 0) { continue ; }
        (*ctx).ret = -(1 as libc::c_int) as u32_0;
        break ;
    }
    osStopTimer(&mut timer);
    if !thread.is_null() { osStopThread(thread); osDestroyThread(thread); };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_ProcessClient(mut callback: u32_0,
                                             mut param0: u32_0,
                                             mut param1: u32_0) -> u32_0 {
    let mut a: FaultClientContext =
        FaultClientContext{callback: None,
                           param0: 0,
                           param1: 0,
                           ret: 0,
                           queue: 0 as *mut OSMesgQueue,
                           msg: 0 as *mut libc::c_void,};
    a.callback =
        ::std::mem::transmute::<libc::intptr_t,
                                Option<unsafe extern "C" fn(_: u32_0,
                                                            _: u32_0)
                                           ->
                                               u32_0>>(callback as
                                                           libc::intptr_t);
    a.param0 = param0;
    a.param1 = param1;
    a.ret = 0 as libc::c_int as u32_0;
    Fault_ProcessClientContext(&mut a);
    return a.ret;
}
#[no_mangle]
pub unsafe extern "C" fn Fault_AddClient(mut client: *mut FaultClient,
                                         mut callback: *mut libc::c_void,
                                         mut param0: *mut libc::c_void,
                                         mut param1: *mut libc::c_void) {
    let mut current_block: u64;
    let mut mask: OSIntMask = 0;
    let mut alreadyExists: s32 = 0 as libc::c_int;
    mask = osSetIntMask(1 as libc::c_int as OSIntMask);
    let mut iter: *mut FaultClient = (*sFaultStructPtr).clients;
    loop  {
        if iter.is_null() { current_block = 17216689946888361452; break ; }
        if iter == client {
            alreadyExists = 1 as libc::c_int;
            current_block = 18078948196177819822;
            break ;
        } else { iter = (*iter).next }
    }
    match current_block {
        17216689946888361452 => {
            (*client).callback = callback as u32_0;
            (*client).param1 = param0 as u32_0;
            (*client).param2 = param1 as u32_0;
            (*client).next = (*sFaultStructPtr).clients;
            (*sFaultStructPtr).clients = client
        }
        _ => { }
    }
    osSetIntMask(mask);
    if alreadyExists != 0 {
        osSyncPrintf(b"\x1b[41;37mfault_AddClient: %08x \xe3\x81\xaf\xe6\x97\xa2\xe3\x81\xab\xe3\x83\xaa\xe3\x82\xb9\xe3\x83\x88\xe4\xb8\xad\xe3\x81\xab\xe3\x81\x82\xe3\x82\x8b\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, client);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_RemoveClient(mut client: *mut FaultClient) {
    let mut iter: *mut FaultClient = 0 as *mut FaultClient;
    let mut lastIter: *mut FaultClient = 0 as *mut FaultClient;
    let mut mask: OSIntMask = 0;
    let mut listIsEmpty: u32_0 = 0;
    iter = (*sFaultStructPtr).clients;
    listIsEmpty = 0 as libc::c_int as u32_0;
    lastIter = 0 as *mut FaultClient;
    mask = osSetIntMask(1 as libc::c_int as OSIntMask);
    while !iter.is_null() {
        if iter == client {
            if !lastIter.is_null() {
                (*lastIter).next = (*client).next
            } else {
                (*sFaultStructPtr).clients = client;
                if !(*sFaultStructPtr).clients.is_null() {
                    (*sFaultStructPtr).clients = (*client).next
                } else { listIsEmpty = 1 as libc::c_int as u32_0 }
            }
            break ;
        } else { lastIter = iter; iter = (*iter).next }
    }
    osSetIntMask(mask);
    if listIsEmpty != 0 {
        osSyncPrintf(b"\x1b[41;37mfault_RemoveClient: %08x \xe3\x83\xaa\xe3\x82\xb9\xe3\x83\x88\xe4\xb8\x8d\xe6\x95\xb4\xe5\x90\x88\xe3\x81\xa7\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, client);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_AddAddrConvClient(mut client:
                                                     *mut FaultAddrConvClient,
                                                 mut callback:
                                                     *mut libc::c_void,
                                                 mut param:
                                                     *mut libc::c_void) {
    let mut current_block: u64;
    let mut mask: OSIntMask = 0;
    let mut alreadyExists: u32_0 = 0 as libc::c_int as u32_0;
    mask = osSetIntMask(1 as libc::c_int as OSIntMask);
    let mut iter: *mut FaultAddrConvClient =
        (*sFaultStructPtr).addrConvClients;
    loop  {
        if iter.is_null() { current_block = 17216689946888361452; break ; }
        if iter == client {
            alreadyExists = 1 as libc::c_int as u32_0;
            current_block = 4295735818851218869;
            break ;
        } else { iter = (*iter).next }
    }
    match current_block {
        17216689946888361452 => {
            (*client).callback = callback as u32_0;
            (*client).param = param as u32_0;
            (*client).next = (*sFaultStructPtr).addrConvClients;
            (*sFaultStructPtr).addrConvClients = client
        }
        _ => { }
    }
    osSetIntMask(mask);
    if alreadyExists != 0 {
        osSyncPrintf(b"\x1b[41;37mfault_AddressConverterAddClient: %08x \xe3\x81\xaf\xe6\x97\xa2\xe3\x81\xab\xe3\x83\xaa\xe3\x82\xb9\xe3\x83\x88\xe4\xb8\xad\xe3\x81\xab\xe3\x81\x82\xe3\x82\x8b\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, client);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_RemoveAddrConvClient(mut client:
                                                        *mut FaultAddrConvClient) {
    let mut iter: *mut FaultAddrConvClient = 0 as *mut FaultAddrConvClient;
    let mut lastIter: *mut FaultAddrConvClient =
        0 as *mut FaultAddrConvClient;
    let mut mask: OSIntMask = 0;
    let mut listIsEmpty: u32_0 = 0;
    iter = (*sFaultStructPtr).addrConvClients;
    listIsEmpty = 0 as libc::c_int as u32_0;
    lastIter = 0 as *mut FaultAddrConvClient;
    mask = osSetIntMask(1 as libc::c_int as OSIntMask);
    while !iter.is_null() {
        if iter == client {
            if !lastIter.is_null() {
                (*lastIter).next = (*client).next
            } else {
                (*sFaultStructPtr).addrConvClients = client;
                if !(*sFaultStructPtr).addrConvClients.is_null() {
                    (*sFaultStructPtr).addrConvClients = (*client).next
                } else { listIsEmpty = 1 as libc::c_int as u32_0 }
            }
            break ;
        } else { lastIter = iter; iter = (*iter).next }
    }
    osSetIntMask(mask);
    if listIsEmpty != 0 {
        osSyncPrintf(b"\x1b[41;37mfault_AddressConverterRemoveClient: %08x \xe3\x81\xaf\xe6\x97\xa2\xe3\x81\xab\xe3\x83\xaa\xe3\x82\xb9\xe3\x83\x88\xe4\xb8\xad\xe3\x81\xab\xe3\x81\x82\xe3\x82\x8b\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, client);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_ConvertAddress(mut client:
                                                  *mut FaultAddrConvClient)
 -> u32_0 {
    let mut ret: u32_0 = 0;
    let mut iter: *mut FaultAddrConvClient =
        (*sFaultStructPtr).addrConvClients;
    while !iter.is_null() {
        if (*iter).callback != 0 as libc::c_int as libc::c_uint {
            ret =
                Fault_ProcessClient((*iter).callback, client as u32_0,
                                    (*iter).param);
            if ret as s32 == -(1 as libc::c_int) {
                Fault_RemoveAddrConvClient(iter);
            } else if ret != 0 as libc::c_int as libc::c_uint { return ret }
        }
        iter = (*iter).next
    }
    return 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn Fault_Sleep(mut duration: u32_0) {
    Fault_SleepImpl(duration);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_PadCallback(mut input: *mut Input) {
    // ! @bug This function is not called correctly and thus will crash from reading a bad pointer at 0x800C7E4C
    PadMgr_RequestPadData(input,
                          0 as
                              libc::c_int); // to make the function allocate 0x28 bytes of stack instead of 0x20
}
#[no_mangle]
pub unsafe extern "C" fn Fault_UpdatePadImpl() {
    (*sFaultStructPtr).padCallback.expect("non-null function pointer")(&mut (*sFaultStructPtr).padInput);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_WaitForInputImpl() -> u32_0 {
    let mut input: *mut Input = &mut (*sFaultStructPtr).padInput;
    let mut count: s32 = 600 as libc::c_int;
    let mut kDown: u32_0 = 0;
    loop  {
        Fault_Sleep(0x10 as libc::c_int as u32_0);
        Fault_UpdatePadImpl();
        kDown = (*input).press.button as u32_0;
        if kDown == 0x20 as libc::c_int as libc::c_uint {
            (*sFaultStructPtr).faultActive =
                ((*sFaultStructPtr).faultActive == 0) as libc::c_int as u8_0
        }
        if (*sFaultStructPtr).faultActive != 0 {
            let fresh1 = count;
            count = count - 1;
            if fresh1 < 1 as libc::c_int { return 0 as libc::c_int as u32_0 }
        } else {
            if kDown == 0x8000 as libc::c_int as libc::c_uint ||
                   kDown == 0x100 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int as u32_0
            }
            if kDown == 0x200 as libc::c_int as libc::c_uint {
                return 1 as libc::c_int as u32_0
            }
            if kDown == 0x800 as libc::c_int as libc::c_uint {
                FaultDrawer_SetOsSyncPrintfEnabled(1 as libc::c_int as u32_0);
            }
            if kDown == 0x400 as libc::c_int as libc::c_uint {
                FaultDrawer_SetOsSyncPrintfEnabled(0 as libc::c_int as u32_0);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_WaitForInput() {
    sFaultIsWaitingForInput = 1 as libc::c_int as u8_0;
    Fault_WaitForInputImpl();
    sFaultIsWaitingForInput = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Fault_DrawRec(mut x: s32, mut y: s32, mut w: s32,
                                       mut h: s32, mut color: u16_0) {
    FaultDrawer_DrawRecImpl(x, y, x + w - 1 as libc::c_int,
                            y + h - 1 as libc::c_int, color);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_FillScreenBlack() {
    FaultDrawer_SetForeColor(0xffff as libc::c_int as u16_0);
    FaultDrawer_SetBackColor(1 as libc::c_int as u16_0);
    FaultDrawer_FillScreen();
    FaultDrawer_SetBackColor(0 as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_FillScreenRed() {
    FaultDrawer_SetForeColor(0xffff as libc::c_int as u16_0);
    FaultDrawer_SetBackColor(0xf001 as libc::c_int as u16_0);
    FaultDrawer_FillScreen();
    FaultDrawer_SetBackColor(0 as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_DrawCornerRec(mut color: u16_0) {
    Fault_DrawRec(0x16 as libc::c_int, 0x10 as libc::c_int, 8 as libc::c_int,
                  1 as libc::c_int, color);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_PrintFReg(mut idx: s32,
                                         mut value: *mut f32_0) {
    let mut raw: u32_0 = *(value as *mut u32_0);
    let mut v0: s32 =
        ((raw & 0x7f800000 as libc::c_int as libc::c_uint) >>
             0x17 as
                 libc::c_int).wrapping_sub(0x7f as libc::c_int as
                                               libc::c_uint) as s32;
    if v0 >= -(0x7e as libc::c_int) && v0 < 0x80 as libc::c_int ||
           raw == 0 as libc::c_int as libc::c_uint {
        FaultDrawer_Printf(b"F%02d:%14.7e \x00" as *const u8 as
                               *const libc::c_char, idx,
                           *value as libc::c_double);
    } else {
        FaultDrawer_Printf(b"F%02d:  %08x(16) \x00" as *const u8 as
                               *const libc::c_char, idx, raw);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_LogFReg(mut idx: s32, mut value: *mut f32_0) {
    let mut raw: u32_0 = *(value as *mut u32_0);
    let mut v0: s32 =
        ((raw & 0x7f800000 as libc::c_int as libc::c_uint) >>
             0x17 as
                 libc::c_int).wrapping_sub(0x7f as libc::c_int as
                                               libc::c_uint) as s32;
    if v0 >= -(0x7e as libc::c_int) && v0 < 0x80 as libc::c_int ||
           raw == 0 as libc::c_int as libc::c_uint {
        osSyncPrintf(b"F%02d:%14.7e \x00" as *const u8 as *const libc::c_char,
                     idx, *value as libc::c_double);
    } else {
        osSyncPrintf(b"F%02d:  %08x(16) \x00" as *const u8 as
                         *const libc::c_char, idx, *(value as *mut u32_0));
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_PrintFPCR(mut value: u32_0) {
    let mut i: s32 = 0;
    let mut flag: u32_0 = 0x20000 as libc::c_int as u32_0;
    FaultDrawer_Printf(b"FPCSR:%08xH \x00" as *const u8 as
                           *const libc::c_char, value);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if value & flag != 0 {
            FaultDrawer_Printf(b"(%s)\x00" as *const u8 as
                                   *const libc::c_char,
                               sExceptionNames[(i + 18 as libc::c_int) as
                                                   usize]);
            break ;
        } else { flag >>= 1 as libc::c_int; i += 1 }
    }
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_LogFPCR(mut value: u32_0) {
    let mut i: s32 = 0;
    let mut flag: u32_0 = 0x20000 as libc::c_int as u32_0;
    osSyncPrintf(b"FPCSR:%08xH  \x00" as *const u8 as *const libc::c_char,
                 value);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if value & flag != 0 {
            osSyncPrintf(b"(%s)\n\x00" as *const u8 as *const libc::c_char,
                         sExceptionNames[(i + 18 as libc::c_int) as usize]);
            break ;
        } else { flag >>= 1 as libc::c_int; i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_PrintThreadContext(mut t: *mut OSThread) {
    let mut ctx: *mut __OSThreadContext = 0 as *mut __OSThreadContext;
    let mut causeStrIdx: s32 =
        (((*t).context.cause >> 2 as libc::c_int &
              0x1f as libc::c_int as libc::c_uint) << 0x10 as libc::c_int) as
            s32 >> 0x10 as libc::c_int;
    if causeStrIdx == 0x17 as libc::c_int {
        causeStrIdx = 0x10 as libc::c_int
    }
    if causeStrIdx == 0x1f as libc::c_int {
        causeStrIdx = 0x11 as libc::c_int
    }
    FaultDrawer_FillScreen();
    FaultDrawer_SetCharPad(-(2 as libc::c_int) as s8, 4 as libc::c_int as s8);
    FaultDrawer_SetCursor(0x16 as libc::c_int, 0x14 as libc::c_int);
    ctx = &mut (*t).context;
    FaultDrawer_Printf(b"THREAD:%d (%d:%s)\n\x00" as *const u8 as
                           *const libc::c_char, (*t).id, causeStrIdx,
                       sExceptionNames[causeStrIdx as usize]);
    FaultDrawer_SetCharPad(-(1 as libc::c_int) as s8, 0 as libc::c_int as s8);
    FaultDrawer_Printf(b"PC:%08xH SR:%08xH VA:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).pc, (*ctx).sr,
                       (*ctx).badvaddr);
    FaultDrawer_Printf(b"AT:%08xH V0:%08xH V1:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).at as u32_0,
                       (*ctx).v0 as u32_0, (*ctx).v1 as u32_0);
    FaultDrawer_Printf(b"A0:%08xH A1:%08xH A2:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).a0 as u32_0,
                       (*ctx).a1 as u32_0, (*ctx).a2 as u32_0);
    FaultDrawer_Printf(b"A3:%08xH T0:%08xH T1:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).a3 as u32_0,
                       (*ctx).t0 as u32_0, (*ctx).t1 as u32_0);
    FaultDrawer_Printf(b"T2:%08xH T3:%08xH T4:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).t2 as u32_0,
                       (*ctx).t3 as u32_0, (*ctx).t4 as u32_0);
    FaultDrawer_Printf(b"T5:%08xH T6:%08xH T7:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).t5 as u32_0,
                       (*ctx).t6 as u32_0, (*ctx).t7 as u32_0);
    FaultDrawer_Printf(b"S0:%08xH S1:%08xH S2:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).s0 as u32_0,
                       (*ctx).s1 as u32_0, (*ctx).s2 as u32_0);
    FaultDrawer_Printf(b"S3:%08xH S4:%08xH S5:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).s3 as u32_0,
                       (*ctx).s4 as u32_0, (*ctx).s5 as u32_0);
    FaultDrawer_Printf(b"S6:%08xH S7:%08xH T8:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).s6 as u32_0,
                       (*ctx).s7 as u32_0, (*ctx).t8 as u32_0);
    FaultDrawer_Printf(b"T9:%08xH GP:%08xH SP:%08xH\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).t9 as u32_0,
                       (*ctx).gp as u32_0, (*ctx).sp as u32_0);
    FaultDrawer_Printf(b"S8:%08xH RA:%08xH LO:%08xH\n\n\x00" as *const u8 as
                           *const libc::c_char, (*ctx).s8 as u32_0,
                       (*ctx).ra as u32_0, (*ctx).lo as u32_0);
    Fault_PrintFPCR((*ctx).fpcsr);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(0 as libc::c_int, &mut (*ctx).fp0.f.f_even);
    Fault_PrintFReg(2 as libc::c_int, &mut (*ctx).fp2.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(4 as libc::c_int, &mut (*ctx).fp4.f.f_even);
    Fault_PrintFReg(6 as libc::c_int, &mut (*ctx).fp6.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(8 as libc::c_int, &mut (*ctx).fp8.f.f_even);
    Fault_PrintFReg(0xa as libc::c_int, &mut (*ctx).fp10.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(0xc as libc::c_int, &mut (*ctx).fp12.f.f_even);
    Fault_PrintFReg(0xe as libc::c_int, &mut (*ctx).fp14.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(0x10 as libc::c_int, &mut (*ctx).fp16.f.f_even);
    Fault_PrintFReg(0x12 as libc::c_int, &mut (*ctx).fp18.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(0x14 as libc::c_int, &mut (*ctx).fp20.f.f_even);
    Fault_PrintFReg(0x16 as libc::c_int, &mut (*ctx).fp22.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(0x18 as libc::c_int, &mut (*ctx).fp24.f.f_even);
    Fault_PrintFReg(0x1a as libc::c_int, &mut (*ctx).fp26.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_PrintFReg(0x1c as libc::c_int, &mut (*ctx).fp28.f.f_even);
    Fault_PrintFReg(0x1e as libc::c_int, &mut (*ctx).fp30.f.f_even);
    FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    FaultDrawer_SetCharPad(0 as libc::c_int as s8, 0 as libc::c_int as s8);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_LogThreadContext(mut t: *mut OSThread) {
    let mut ctx: *mut __OSThreadContext = 0 as *mut __OSThreadContext;
    let mut causeStrIdx: s32 =
        (((*t).context.cause >> 2 as libc::c_int &
              0x1f as libc::c_int as libc::c_uint) << 0x10 as libc::c_int) as
            s32 >> 0x10 as libc::c_int;
    if causeStrIdx == 0x17 as libc::c_int {
        causeStrIdx = 0x10 as libc::c_int
    }
    if causeStrIdx == 0x1f as libc::c_int {
        causeStrIdx = 0x11 as libc::c_int
    }
    ctx = &mut (*t).context;
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"THREAD ID:%d (%d:%s)\n\x00" as *const u8 as
                     *const libc::c_char, (*t).id, causeStrIdx,
                 sExceptionNames[causeStrIdx as usize]);
    osSyncPrintf(b"PC:%08xH   SR:%08xH   VA:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).pc, (*ctx).sr,
                 (*ctx).badvaddr);
    osSyncPrintf(b"AT:%08xH   V0:%08xH   V1:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).at as u32_0,
                 (*ctx).v0 as u32_0, (*ctx).v1 as u32_0);
    osSyncPrintf(b"A0:%08xH   A1:%08xH   A2:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).a0 as u32_0,
                 (*ctx).a1 as u32_0, (*ctx).a2 as u32_0);
    osSyncPrintf(b"A3:%08xH   T0:%08xH   T1:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).a3 as u32_0,
                 (*ctx).t0 as u32_0, (*ctx).t1 as u32_0);
    osSyncPrintf(b"T2:%08xH   T3:%08xH   T4:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).t2 as u32_0,
                 (*ctx).t3 as u32_0, (*ctx).t4 as u32_0);
    osSyncPrintf(b"T5:%08xH   T6:%08xH   T7:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).t5 as u32_0,
                 (*ctx).t6 as u32_0, (*ctx).t7 as u32_0);
    osSyncPrintf(b"S0:%08xH   S1:%08xH   S2:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).s0 as u32_0,
                 (*ctx).s1 as u32_0, (*ctx).s2 as u32_0);
    osSyncPrintf(b"S3:%08xH   S4:%08xH   S5:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).s3 as u32_0,
                 (*ctx).s4 as u32_0, (*ctx).s5 as u32_0);
    osSyncPrintf(b"S6:%08xH   S7:%08xH   T8:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).s6 as u32_0,
                 (*ctx).s7 as u32_0, (*ctx).t8 as u32_0);
    osSyncPrintf(b"T9:%08xH   GP:%08xH   SP:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).t9 as u32_0,
                 (*ctx).gp as u32_0, (*ctx).sp as u32_0);
    osSyncPrintf(b"S8:%08xH   RA:%08xH   LO:%08xH\n\x00" as *const u8 as
                     *const libc::c_char, (*ctx).s8 as u32_0,
                 (*ctx).ra as u32_0, (*ctx).lo as u32_0);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFPCR((*ctx).fpcsr);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(0 as libc::c_int, &mut (*ctx).fp0.f.f_even);
    Fault_LogFReg(2 as libc::c_int, &mut (*ctx).fp2.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(4 as libc::c_int, &mut (*ctx).fp4.f.f_even);
    Fault_LogFReg(6 as libc::c_int, &mut (*ctx).fp6.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(8 as libc::c_int, &mut (*ctx).fp8.f.f_even);
    Fault_LogFReg(0xa as libc::c_int, &mut (*ctx).fp10.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(0xc as libc::c_int, &mut (*ctx).fp12.f.f_even);
    Fault_LogFReg(0xe as libc::c_int, &mut (*ctx).fp14.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(0x10 as libc::c_int, &mut (*ctx).fp16.f.f_even);
    Fault_LogFReg(0x12 as libc::c_int, &mut (*ctx).fp18.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(0x14 as libc::c_int, &mut (*ctx).fp20.f.f_even);
    Fault_LogFReg(0x16 as libc::c_int, &mut (*ctx).fp22.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(0x18 as libc::c_int, &mut (*ctx).fp24.f.f_even);
    Fault_LogFReg(0x1a as libc::c_int, &mut (*ctx).fp26.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    Fault_LogFReg(0x1c as libc::c_int, &mut (*ctx).fp28.f.f_even);
    Fault_LogFReg(0x1e as libc::c_int, &mut (*ctx).fp30.f.f_even);
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_FindFaultedThread() -> *mut OSThread {
    let mut iter: *mut OSThread = __osGetActiveQueue();
    while (*iter).priority != -(1 as libc::c_int) {
        if (*iter).priority > 0 as libc::c_int &&
               (*iter).priority < 0x7f as libc::c_int &&
               (*iter).flags as libc::c_int & 3 as libc::c_int != 0 {
            return iter
        }
        iter = (*iter).tlnext
    }
    return 0 as *mut OSThread;
}
#[no_mangle]
pub unsafe extern "C" fn Fault_Wait5Seconds() {
    let mut start: [OSTime; 2] = [0; 2];
    start[0 as libc::c_int as usize] = osGetTime();
    loop  {
        Fault_Sleep(0x10 as libc::c_int as u32_0);
        if !(osGetTime().wrapping_sub(start[0 as libc::c_int as usize]) <
                 (5000000 as libc::c_int as
                      u64_0).wrapping_mul((62500000 as libc::c_longlong *
                                               3 as libc::c_int as
                                                   libc::c_longlong /
                                               4 as libc::c_int as
                                                   libc::c_longlong /
                                               15625 as libc::c_longlong) as
                                              libc::c_ulonglong).wrapping_div((1000000
                                                                                   as
                                                                                   libc::c_longlong
                                                                                   /
                                                                                   15625
                                                                                       as
                                                                                       libc::c_longlong)
                                                                                  as
                                                                                  libc::c_ulonglong).wrapping_add(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulonglong))
           {
            break ;
        }
    }
    (*sFaultStructPtr).faultActive = 1 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Fault_WaitForButtonCombo() {
    let mut input: *mut Input = &mut (*sFaultStructPtr).padInput;
    let mut state: s32 = 0;
    let mut s1: u32_0 = 0;
    let mut s2: u32_0 = 0;
    let mut kDown: u32_0 = 0;
    let mut kCur: u32_0 = 0;
    osSyncPrintf(b"\x1b[37mKeyWaitB (\xef\xbc\xac\xef\xbc\xb2\xef\xbc\xba \x1b[37m\xe4\xb8\x8a\x1b[33m\xe4\xb8\x8b \x1b[33m\xe4\xb8\x8a\x1b[37m\xe4\xb8\x8b \x1b[37m\xe5\xb7\xa6\x1b[33m\xe5\xb7\xa6 \x1b[33m\xe5\x8f\xb3\x1b[37m\xe5\x8f\xb3 \x1b[32m\xef\xbc\xa2\x1b[34m\xef\xbc\xa1\x1b[31mSTART\x1b[37m)\x1b[m\n\x00"
                     as *const u8 as *const libc::c_char);
    osSyncPrintf(b"\x1b[37mKeyWaitB\'(\xef\xbc\xac\xef\xbc\xb2\xe5\xb7\xa6\x1b[33m\xe5\x8f\xb3 +\x1b[31mSTART\x1b[37m)\x1b[m\n\x00"
                     as *const u8 as *const libc::c_char);
    FaultDrawer_SetForeColor(0xffff as libc::c_int as u16_0);
    FaultDrawer_SetBackColor(1 as libc::c_int as u16_0);
    state = 0 as libc::c_int;
    s1 = 0 as libc::c_int as u32_0;
    s2 = 1 as libc::c_int as u32_0;
    while state != 11 as libc::c_int {
        Fault_Sleep(0x10 as libc::c_int as u32_0);
        Fault_UpdatePadImpl();
        kDown = (*input).press.button as u32_0;
        kCur = (*input).cur.button as u32_0;
        if kCur == 0 as libc::c_int as libc::c_uint && s1 == s2 {
            s1 = 0 as libc::c_int as u32_0
        } else if kDown != 0 as libc::c_int as libc::c_uint {
            if s1 == s2 { state = 0 as libc::c_int }
            match state {
                0 => {
                    if kCur ==
                           (0x2000 as libc::c_int | 0x20 as libc::c_int |
                                0x10 as libc::c_int) as libc::c_uint &&
                           kDown == 0x2000 as libc::c_int as libc::c_uint {
                        state = s2 as s32;
                        s1 = s2
                    }
                }
                1 => {
                    if kDown == 0x800 as libc::c_int as libc::c_uint {
                        state = 2 as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                2 => {
                    if kDown == 0x4 as libc::c_int as libc::c_uint {
                        state = 3 as libc::c_int;
                        s1 = s2
                    } else { state = 0 as libc::c_int }
                }
                3 => {
                    if kDown == 0x8 as libc::c_int as libc::c_uint {
                        state = 4 as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                4 => {
                    if kDown == 0x400 as libc::c_int as libc::c_uint {
                        state = 5 as libc::c_int;
                        s1 = s2
                    } else { state = 0 as libc::c_int }
                }
                5 => {
                    if kDown == 0x200 as libc::c_int as libc::c_uint {
                        state = 6 as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                6 => {
                    if kDown == 0x2 as libc::c_int as libc::c_uint {
                        state = 7 as libc::c_int;
                        s1 = s2
                    } else { state = 0 as libc::c_int }
                }
                7 => {
                    if kDown == 0x1 as libc::c_int as libc::c_uint {
                        state = 8 as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                8 => {
                    if kDown == 0x100 as libc::c_int as libc::c_uint {
                        state = 9 as libc::c_int;
                        s1 = s2
                    } else { state = 0 as libc::c_int }
                }
                9 => {
                    if kDown ==
                           (0x8000 as libc::c_int | 0x4000 as libc::c_int) as
                               libc::c_uint {
                        state = 10 as libc::c_int
                    } else if kDown == 0x8000 as libc::c_int as libc::c_uint {
                        state = 0x5b as libc::c_int
                    } else if kDown == 0x4000 as libc::c_int as libc::c_uint {
                        state = 0x5c as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                91 => {
                    if kDown == 0x4000 as libc::c_int as libc::c_uint {
                        state = 10 as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                92 => {
                    if kDown == 0x8000 as libc::c_int as libc::c_uint {
                        state = 10 as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                10 => {
                    if kDown == 0x1000 as libc::c_int as libc::c_uint {
                        state = 11 as libc::c_int
                    } else { state = 0 as libc::c_int }
                }
                _ => { }
            }
        }
        osWritebackDCacheAll();
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_DrawMemDumpPage(mut title: *const libc::c_char,
                                               mut addr: *mut u32_0,
                                               mut param_3: u32_0) {
    let mut alignedAddr: *mut u32_0 = 0 as *mut u32_0;
    let mut writeAddr: *mut u32_0 = 0 as *mut u32_0;
    let mut y: s32 = 0;
    let mut x: s32 = 0;
    alignedAddr = addr;
    if alignedAddr < 0x80000000 as libc::c_uint as *mut u32_0 {
        alignedAddr = 0x80000000 as libc::c_uint as *mut u32_0
    }
    if alignedAddr > 0x807fff00 as libc::c_uint as *mut u32_0 {
        alignedAddr = 0x807fff00 as libc::c_uint as *mut u32_0
    }
    alignedAddr =
        (alignedAddr as u32_0 & !(3 as libc::c_int) as libc::c_uint) as
            *mut u32_0;
    writeAddr = alignedAddr;
    Fault_FillScreenBlack();
    FaultDrawer_SetCharPad(-(2 as libc::c_int) as s8, 0 as libc::c_int as s8);
    FaultDrawer_DrawText(0x24 as libc::c_int, 0x12 as libc::c_int,
                         b"%s %08x\x00" as *const u8 as *const libc::c_char,
                         if !title.is_null() {
                             title
                         } else {
                             b"PrintDump\x00" as *const u8 as
                                 *const libc::c_char
                         }, alignedAddr);
    if alignedAddr >= 0x80000000 as libc::c_uint as *mut u32_0 &&
           alignedAddr < 0xc0000000 as libc::c_uint as *mut u32_0 {
        y = 0x1c as libc::c_int;
        while y != 0xe2 as libc::c_int {
            FaultDrawer_DrawText(0x18 as libc::c_int, y,
                                 b"%06x\x00" as *const u8 as
                                     *const libc::c_char, writeAddr);
            x = 0x52 as libc::c_int;
            while x != 0x122 as libc::c_int {
                let fresh2 = writeAddr;
                writeAddr = writeAddr.offset(1);
                FaultDrawer_DrawText(x, y,
                                     b"%08x\x00" as *const u8 as
                                         *const libc::c_char, *fresh2);
                x += 0x34 as libc::c_int
            }
            y += 9 as libc::c_int
        }
    }
    FaultDrawer_SetCharPad(0 as libc::c_int as s8, 0 as libc::c_int as s8);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_DrawMemDump(mut pc: u32_0, mut sp: u32_0,
                                           mut unk0: u32_0, mut unk1: u32_0) {
    let mut input: *mut Input = &mut (*sFaultStructPtr).padInput;
    let mut addr: u32_0 = pc;
    let mut count: s32 = 0;
    let mut off: u32_0 = 0;
    loop  {
        count = 0 as libc::c_int;
        if addr < 0x80000000 as libc::c_uint {
            addr = 0x80000000 as libc::c_uint
        }
        if addr > 0x807fff00 as libc::c_uint {
            addr = 0x807fff00 as libc::c_uint
        }
        addr &= !(0xf as libc::c_int) as libc::c_uint;
        Fault_DrawMemDumpPage(b"Dump\x00" as *const u8 as *const libc::c_char,
                              addr as *mut u32_0, 0 as libc::c_int as u32_0);
        count = 600 as libc::c_int;
        while (*sFaultStructPtr).faultActive != 0 {
            if count == 0 as libc::c_int { return }
            count -= 1;
            Fault_Sleep(0x10 as libc::c_int as u32_0);
            Fault_UpdatePadImpl();
            if !((*input).press.button as libc::c_int |
                     !(0x20 as libc::c_int)) == 0 as libc::c_int {
                (*sFaultStructPtr).faultActive = 0 as libc::c_int as u8_0
            }
        }
        loop  {
            Fault_Sleep(0x10 as libc::c_int as u32_0);
            Fault_UpdatePadImpl();
            if !((*input).press.button as libc::c_int == 0 as libc::c_int) {
                break ;
            }
        }
        if !((*input).press.button as libc::c_int | !(0x1000 as libc::c_int))
               == 0 as libc::c_int ||
               !((*input).cur.button as libc::c_int |
                     !(0x8000 as libc::c_int)) == 0 as libc::c_int {
            return
        }
        off = 0x10 as libc::c_int as u32_0;
        if !((*input).cur.button as libc::c_int | !(0x2000 as libc::c_int)) ==
               0 as libc::c_int {
            off = 0x100 as libc::c_int as u32_0
        }
        if !((*input).cur.button as libc::c_int | !(0x4000 as libc::c_int)) ==
               0 as libc::c_int {
            off <<= 8 as libc::c_int
        }
        if !((*input).press.button as libc::c_int | !(0x800 as libc::c_int))
               == 0 as libc::c_int {
            addr = (addr as libc::c_uint).wrapping_sub(off) as u32_0 as u32_0
        }
        if !((*input).press.button as libc::c_int | !(0x400 as libc::c_int))
               == 0 as libc::c_int {
            addr = (addr as libc::c_uint).wrapping_add(off) as u32_0 as u32_0
        }
        if !((*input).press.button as libc::c_int | !(0x8 as libc::c_int)) ==
               0 as libc::c_int {
            addr = pc
        }
        if !((*input).press.button as libc::c_int | !(0x4 as libc::c_int)) ==
               0 as libc::c_int {
            addr = sp
        }
        if !((*input).press.button as libc::c_int | !(0x2 as libc::c_int)) ==
               0 as libc::c_int {
            addr = unk0
        }
        if !((*input).press.button as libc::c_int | !(0x1 as libc::c_int)) ==
               0 as libc::c_int {
            addr = unk1
        }
        if !((*input).press.button as libc::c_int | !(0x20 as libc::c_int)) ==
               0 as libc::c_int {
            break ;
        }
    }
    (*sFaultStructPtr).faultActive = 1 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Fault_WalkStack(mut spPtr: *mut u32_0,
                                         mut pcPtr: *mut u32_0,
                                         mut raPtr: *mut u32_0) {
    let mut current_block: u64;
    let mut sp: u32_0 = *spPtr;
    let mut pc: u32_0 = *pcPtr;
    let mut ra: u32_0 = *raPtr;
    let mut count: s32 = 0x10000 as libc::c_int;
    let mut lastOpc: u32_0 = 0;
    let mut opc: u32_0 = 0;
    let mut opcHi: u16_0 = 0;
    let mut opcLo: s16 = 0;
    let mut imm: u32_0 = 0;
    if sp & 3 as libc::c_int as libc::c_uint != 0 ||
           sp < 0x80000000 as libc::c_uint || sp >= 0xa0000000 as libc::c_uint
           || ra & 3 as libc::c_int as libc::c_uint != 0 ||
           ra < 0x80000000 as libc::c_uint || ra >= 0xa0000000 as libc::c_uint
       {
        *spPtr = 0 as libc::c_int as u32_0;
        *pcPtr = 0 as libc::c_int as u32_0;
        *raPtr = 0 as libc::c_int as u32_0;
        return
    }
    if pc & 3 as libc::c_int as libc::c_uint != 0 ||
           pc < 0x80000000 as libc::c_uint || pc >= 0xa0000000 as libc::c_uint
       {
        *pcPtr = ra;
        return
    }
    lastOpc = 0 as libc::c_int as u32_0;
    loop  {
        opc = *((pc | 0xa0000000 as libc::c_uint) as *mut u32_0);
        opcHi = (opc >> 16 as libc::c_int) as u16_0;
        opcLo = (opc & 0xffff as libc::c_int as libc::c_uint) as s16;
        imm = opcLo as u32_0;
        if opcHi as libc::c_int == 0x8fbf as libc::c_int {
            ra =
                *((sp.wrapping_add(imm) | 0xa0000000 as libc::c_uint) as
                      *mut u32_0)
        } else if opcHi as libc::c_int == 0x27bd as libc::c_int {
            sp = (sp as libc::c_uint).wrapping_add(imm) as u32_0 as u32_0
        } else if opc == 0x42000018 as libc::c_int as libc::c_uint {
            sp = 0 as libc::c_int as u32_0;
            pc = 0 as libc::c_int as u32_0;
            ra = 0 as libc::c_int as u32_0;
            current_block = 15127831175933161559;
            break ;
        }
        if lastOpc == 0x3e00008 as libc::c_int as libc::c_uint {
            pc = ra;
            current_block = 15127831175933161559;
            break ;
        } else if lastOpc >> 26 as libc::c_int ==
                      2 as libc::c_int as libc::c_uint {
            pc =
                (pc >> 28 as libc::c_int) << 28 as libc::c_int |
                    lastOpc << 6 as libc::c_int >> 4 as libc::c_int;
            current_block = 15127831175933161559;
            break ;
        } else {
            lastOpc = opc;
            pc =
                (pc as
                     libc::c_uint).wrapping_add(4 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0;
            if count == 0 as libc::c_int {
                current_block = 14832935472441733737;
                break ;
            }
            count -= 1
        }
    }
    match current_block {
        14832935472441733737 => {
            sp = 0 as libc::c_int as u32_0;
            pc = 0 as libc::c_int as u32_0;
            ra = 0 as libc::c_int as u32_0
        }
        _ => { }
    }
    *spPtr = sp;
    *pcPtr = pc;
    *raPtr = ra;
}
#[no_mangle]
pub unsafe extern "C" fn Fault_DrawStackTrace(mut thread: *mut OSThread,
                                              mut x: s32, mut y: s32,
                                              mut height: s32) {
    let mut line: s32 = 0;
    let mut sp: u32_0 = (*thread).context.sp as u32_0;
    let mut ra: u32_0 = (*thread).context.ra as u32_0;
    let mut pc: u32_0 = (*thread).context.pc;
    let mut addr: u32_0 = 0;
    FaultDrawer_DrawText(x, y,
                         b"SP       PC       (VPC)\x00" as *const u8 as
                             *const libc::c_char);
    line = 1 as libc::c_int;
    while line < height &&
              (ra != 0 as libc::c_int as libc::c_uint ||
                   sp != 0 as libc::c_int as libc::c_uint) &&
              pc !=
                  ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                     -> ()>,
                                          u32_0>(Some(__osCleanupThread as
                                                          unsafe extern "C" fn()
                                                              -> ())) {
        FaultDrawer_DrawText(x, y + line * 8 as libc::c_int,
                             b"%08x %08x\x00" as *const u8 as
                                 *const libc::c_char, sp, pc);
        addr = Fault_ConvertAddress(pc as *mut FaultAddrConvClient);
        if addr != 0 as libc::c_int as libc::c_uint {
            FaultDrawer_Printf(b" -> %08x\x00" as *const u8 as
                                   *const libc::c_char, addr);
        }
        Fault_WalkStack(&mut sp, &mut pc, &mut ra);
        line += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_LogStackTrace(mut thread: *mut OSThread,
                                             mut height: s32) {
    let mut line: s32 = 0;
    let mut sp: u32_0 = (*thread).context.sp as u32_0;
    let mut ra: u32_0 = (*thread).context.ra as u32_0;
    let mut pc: u32_0 = (*thread).context.pc;
    let mut addr: u32_0 = 0;
    let mut pad: s32 = 0;
    osSyncPrintf(b"STACK TRACE\nSP       PC       (VPC)\n\x00" as *const u8 as
                     *const libc::c_char);
    line = 1 as libc::c_int;
    while line < height &&
              (ra != 0 as libc::c_int as libc::c_uint ||
                   sp != 0 as libc::c_int as libc::c_uint) &&
              pc !=
                  ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                     -> ()>,
                                          u32_0>(Some(__osCleanupThread as
                                                          unsafe extern "C" fn()
                                                              -> ())) {
        osSyncPrintf(b"%08x %08x\x00" as *const u8 as *const libc::c_char, sp,
                     pc);
        addr = Fault_ConvertAddress(pc as *mut FaultAddrConvClient);
        if addr != 0 as libc::c_int as libc::c_uint {
            osSyncPrintf(b" -> %08x\x00" as *const u8 as *const libc::c_char,
                         addr);
        }
        osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
        Fault_WalkStack(&mut sp, &mut pc, &mut ra);
        line += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_ResumeThread(mut t: *mut OSThread) {
    (*t).context.cause = 0 as libc::c_int as u32_0;
    (*t).context.fpcsr = 0 as libc::c_int as u32_0;
    (*t).context.pc =
        ((*t).context.pc as
             libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as
            u32_0 as u32_0;
    *((*t).context.pc as *mut u32_0) = 0xd as libc::c_int as u32_0;
    osWritebackDCache((*t).context.pc as *mut libc::c_void, 4 as libc::c_int);
    osInvalICache((*t).context.pc as *mut libc::c_void, 4 as libc::c_int);
    osStartThread(t);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_CommitFB() {
    let mut fb: *mut u16_0 = 0 as *mut u16_0;
    osViSetYScale(1.0f32);
    osViSetMode(&mut osViModeNtscLan1);
    osViSetSpecialFeatures((0x2 as libc::c_int | 0x40 as libc::c_int) as
                               u32_0);
    osViBlack(0 as libc::c_int as u8_0);
    if !(*sFaultStructPtr).fb.is_null() {
        fb = (*sFaultStructPtr).fb as *mut u16_0
    } else {
        fb = osViGetNextFramebuffer() as *mut u16_0;
        if fb as u32_0 == 0x80000000 as libc::c_uint {
            fb =
                (osMemSize |
                     0x80000000 as
                         libc::c_uint).wrapping_sub(::std::mem::size_of::<[[u16_0; 320]; 240]>()
                                                        as libc::c_ulong) as
                    *mut u16_0
        }
    }
    osViSwapBuffer(fb as *mut libc::c_void);
    FaultDrawer_SetDrawerFB(fb as *mut libc::c_void,
                            320 as libc::c_int as u16_0,
                            240 as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_ProcessClients() {
    let mut iter: *mut FaultClient = (*sFaultStructPtr).clients;
    let mut idx: s32 = 0 as libc::c_int;
    while !iter.is_null() {
        if (*iter).callback != 0 as libc::c_int as libc::c_uint {
            Fault_FillScreenBlack();
            FaultDrawer_SetCharPad(-(2 as libc::c_int) as s8,
                                   0 as libc::c_int as s8);
            let fresh3 = idx;
            idx = idx + 1;
            FaultDrawer_Printf(b"\x1a8CallBack (%d) %08x %08x %08x\n\x1a7\x00"
                                   as *const u8 as *const libc::c_char,
                               fresh3, iter, (*iter).param1, (*iter).param2);
            FaultDrawer_SetCharPad(0 as libc::c_int as s8,
                                   0 as libc::c_int as s8);
            Fault_ProcessClient((*iter).callback, (*iter).param1,
                                (*iter).param2);
            Fault_WaitForInput();
            Fault_CommitFB();
        }
        iter = (*iter).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_UpdatePad() { Fault_UpdatePadImpl(); }
#[no_mangle]
pub unsafe extern "C" fn Fault_ThreadEntry(mut arg: *mut libc::c_void) {
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut faultedThread: *mut OSThread = 0 as *mut OSThread;
    let mut pad: s32 = 0;
    osSetEventMesg(10 as libc::c_int as OSEvent,
                   &mut (*sFaultStructPtr).queue, 1 as libc::c_int as OSMesg);
    osSetEventMesg(12 as libc::c_int as OSEvent,
                   &mut (*sFaultStructPtr).queue, 2 as libc::c_int as OSMesg);
    loop  {
        let mut current_block_19: u64;
        loop  {
            osRecvMesg(&mut (*sFaultStructPtr).queue, &mut msg,
                       1 as libc::c_int);
            if msg == 1 as libc::c_int as OSMesg {
                (*sFaultStructPtr).msgId = 1 as libc::c_int as u8_0;
                osSyncPrintf(b"\xe3\x83\x95\xe3\x82\xa9\xe3\x83\xab\xe3\x83\x88\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3:OS_EVENT_CPU_BREAK\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                                 as *const u8 as *const libc::c_char);
                current_block_19 = 6057473163062296781;
            } else if 1 as libc::c_int != 0 &&
                          msg == 2 as libc::c_int as OSMesg {
                (*sFaultStructPtr).msgId = 2 as libc::c_int as u8_0;
                osSyncPrintf(b"\xe3\x83\x95\xe3\x82\xa9\xe3\x83\xab\xe3\x83\x88\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3:OS_EVENT_FAULT\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                                 as *const u8 as *const libc::c_char);
                current_block_19 = 6057473163062296781;
            } else if msg == 3 as libc::c_int as OSMesg {
                Fault_UpdatePad();
                faultedThread = 0 as *mut OSThread;
                current_block_19 = 7502529970979898288;
            } else {
                (*sFaultStructPtr).msgId = 3 as libc::c_int as u8_0;
                osSyncPrintf(b"\xe3\x83\x95\xe3\x82\xa9\xe3\x83\xab\xe3\x83\x88\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3:\xe4\xb8\x8d\xe6\x98\x8e\xe3\x81\xaa\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                                 as *const u8 as *const libc::c_char);
                current_block_19 = 6057473163062296781;
            }
            match current_block_19 {
                6057473163062296781 => {
                    faultedThread = __osGetCurrFaultedThread();
                    osSyncPrintf(b"__osGetCurrFaultedThread()=%08x\n\x00" as
                                     *const u8 as *const libc::c_char,
                                 faultedThread);
                    if faultedThread.is_null() {
                        faultedThread = Fault_FindFaultedThread();
                        osSyncPrintf(b"FindFaultedThread()=%08x\n\x00" as
                                         *const u8 as *const libc::c_char,
                                     faultedThread);
                    }
                }
                _ => { }
            }
            if !faultedThread.is_null() { break ; }
        }
        __osSetFpcCsr(__osGetFpcCsr() &
                          -(0xf81 as libc::c_int) as libc::c_uint);
        (*sFaultStructPtr).faultedThread = faultedThread;
        while (*sFaultStructPtr).faultHandlerEnabled == 0 {
            Fault_Sleep(1000 as libc::c_int as u32_0);
        }
        Fault_Sleep(500 as libc::c_int as u32_0);
        Fault_CommitFB();
        if (*sFaultStructPtr).faultActive != 0 {
            Fault_Wait5Seconds();
        } else {
            Fault_DrawCornerRec(0xf801 as libc::c_int as u16_0);
            Fault_WaitForButtonCombo();
        }
        (*sFaultStructPtr).faultActive = 1 as libc::c_int as u8_0;
        FaultDrawer_SetForeColor(0xffff as libc::c_int as u16_0);
        FaultDrawer_SetBackColor(0 as libc::c_int as u16_0);
        loop  {
            Fault_PrintThreadContext(faultedThread);
            Fault_LogThreadContext(faultedThread);
            Fault_WaitForInput();
            Fault_FillScreenBlack();
            FaultDrawer_DrawText(0x78 as libc::c_int, 0x10 as libc::c_int,
                                 b"STACK TRACE\x00" as *const u8 as
                                     *const libc::c_char);
            Fault_DrawStackTrace(faultedThread, 0x24 as libc::c_int,
                                 0x18 as libc::c_int, 0x16 as libc::c_int);
            Fault_LogStackTrace(faultedThread, 0x32 as libc::c_int);
            Fault_WaitForInput();
            Fault_ProcessClients();
            Fault_DrawMemDump((*faultedThread).context.pc.wrapping_sub(0x100
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint),
                              (*faultedThread).context.sp as u32_0,
                              0 as libc::c_int as u32_0,
                              0 as libc::c_int as u32_0);
            Fault_FillScreenRed();
            FaultDrawer_DrawText(0x40 as libc::c_int, 0x50 as libc::c_int,
                                 b"    CONGRATURATIONS!    \x00" as *const u8
                                     as *const libc::c_char);
            FaultDrawer_DrawText(0x40 as libc::c_int, 0x5a as libc::c_int,
                                 b"All Pages are displayed.\x00" as *const u8
                                     as *const libc::c_char);
            FaultDrawer_DrawText(0x40 as libc::c_int, 0x64 as libc::c_int,
                                 b"       THANK YOU!       \x00" as *const u8
                                     as *const libc::c_char);
            FaultDrawer_DrawText(0x40 as libc::c_int, 0x6e as libc::c_int,
                                 b" You are great debugger!\x00" as *const u8
                                     as *const libc::c_char);
            Fault_WaitForInput();
            if !((*sFaultStructPtr).exitDebugger == 0) { break ; }
        }
        while (*sFaultStructPtr).exitDebugger == 0 { }
        Fault_ResumeThread(faultedThread);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fault_SetFB(mut fb: *mut libc::c_void, mut w: u16_0,
                                     mut h: u16_0) {
    (*sFaultStructPtr).fb = fb;
    FaultDrawer_SetDrawerFB(fb, w, h);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_Init() {
    sFaultStructPtr = &mut gFaultStruct;
    bzero(sFaultStructPtr as *mut libc::c_void,
          ::std::mem::size_of::<FaultThreadStruct>() as libc::c_ulong);
    FaultDrawer_SetDefault();
    FaultDrawer_SetInputCallback(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                  -> (),
                                                              unsafe extern "C" fn()
                                                                  ->
                                                                      ()>(Fault_WaitForInput)));
    (*sFaultStructPtr).exitDebugger = 0 as libc::c_int as u8_0;
    (*sFaultStructPtr).msgId = 0 as libc::c_int as u8_0;
    (*sFaultStructPtr).faultHandlerEnabled = 0 as libc::c_int as u8_0;
    (*sFaultStructPtr).faultedThread = 0 as *mut OSThread;
    (*sFaultStructPtr).padCallback =
        Some(Fault_PadCallback as unsafe extern "C" fn(_: *mut Input) -> ());
    (*sFaultStructPtr).clients = 0 as *mut FaultClient;
    (*sFaultStructPtr).faultActive = 0 as libc::c_int as u8_0;
    gFaultStruct.faultHandlerEnabled = 1 as libc::c_int as u8_0;
    osCreateMesgQueue(&mut (*sFaultStructPtr).queue,
                      &mut (*sFaultStructPtr).msg, 1 as libc::c_int);
    StackCheck_Init(&mut sFaultThreadInfo,
                    &mut sFaultStack as *mut [libc::c_char; 1536] as
                        *mut libc::c_void,
                    sFaultStack.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 1536]>()
                                                        as libc::c_ulong as
                                                        isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x100 as libc::c_int,
                    b"fault\x00" as *const u8 as *const libc::c_char);
    osCreateThread(&mut (*sFaultStructPtr).thread, 2 as libc::c_int,
                   Some(Fault_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   0 as *mut libc::c_void,
                   sFaultStack.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 1536]>()
                                                       as libc::c_ulong as
                                                       isize) as
                       *mut libc::c_void, 127 as libc::c_int);
    osStartThread(&mut (*sFaultStructPtr).thread);
}
#[no_mangle]
pub unsafe extern "C" fn Fault_HangupFaultClient(mut arg0:
                                                     *const libc::c_char,
                                                 mut arg1:
                                                     *const libc::c_char) {
    osSyncPrintf(b"HungUp on Thread %d\n\x00" as *const u8 as
                     *const libc::c_char, osGetThreadId(0 as *mut OSThread));
    osSyncPrintf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                 if !arg0.is_null() {
                     arg0
                 } else {
                     b"(NULL)\x00" as *const u8 as *const libc::c_char
                 });
    osSyncPrintf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                 if !arg1.is_null() {
                     arg1
                 } else {
                     b"(NULL)\x00" as *const u8 as *const libc::c_char
                 });
    FaultDrawer_Printf(b"HungUp on Thread %d\n\x00" as *const u8 as
                           *const libc::c_char,
                       osGetThreadId(0 as *mut OSThread));
    FaultDrawer_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                       if !arg0.is_null() {
                           arg0
                       } else {
                           b"(NULL)\x00" as *const u8 as *const libc::c_char
                       });
    FaultDrawer_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                       if !arg1.is_null() {
                           arg1
                       } else {
                           b"(NULL)\x00" as *const u8 as *const libc::c_char
                       });
}
#[no_mangle]
pub unsafe extern "C" fn Fault_AddHungupAndCrashImpl(mut arg0:
                                                         *const libc::c_char,
                                                     mut arg1:
                                                         *const libc::c_char) {
    let mut client: FaultClient =
        FaultClient{next: 0 as *mut FaultClient,
                    callback: 0,
                    param1: 0,
                    param2: 0,};
    let mut pad: s32 = 0;
    Fault_AddClient(&mut client,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *const libc::c_char,
                                                                        _:
                                                                            *const libc::c_char)
                                                       -> ()>,
                                            *mut libc::c_void>(Some(Fault_HangupFaultClient
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *const libc::c_char,
                                                                                             _:
                                                                                                 *const libc::c_char)
                                                                            ->
                                                                                ())),
                    arg0 as *mut libc::c_void, arg1 as *mut libc::c_void);
    *(0x11111111 as libc::c_int as *mut u32_0) = 0 as libc::c_int as u32_0;
    // trigger an exception
}
#[no_mangle]
pub unsafe extern "C" fn Fault_AddHungupAndCrash(mut filename:
                                                     *const libc::c_char,
                                                 mut line: u32_0) {
    let mut msg: [libc::c_char; 256] = [0; 256];
    sprintf(msg.as_mut_ptr(),
            b"HungUp %s:%d\x00" as *const u8 as *const libc::c_char, filename,
            line);
    Fault_AddHungupAndCrashImpl(msg.as_mut_ptr(), 0 as *const libc::c_char);
}
