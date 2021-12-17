#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn Idle_ThreadEntry(arg: *mut libc::c_void);
    #[no_mangle]
    fn Locale_Init();
    #[no_mangle]
    fn isPrintfInit();
    #[no_mangle]
    fn osDriveRomInit() -> *mut OSPiHandle;
    #[no_mangle]
    fn StackCheck_Init(entry: *mut StackEntry, stackTop: *mut libc::c_void,
                       stackBottom: *mut libc::c_void, initValue: u32_0,
                       minSpace: s32, name: *const libc::c_char);
    #[no_mangle]
    fn __osInitialize_common();
    #[no_mangle]
    fn __osInitialize_autodetect();
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn osGetMemSize() -> u32_0;
    #[no_mangle]
    fn osCartRomInit() -> *mut OSPiHandle;
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    static mut gCartHandle: *mut OSPiHandle;
    #[no_mangle]
    static mut osMemSize: u32_0;
    #[no_mangle]
    static mut _dmadataSegmentStart: [u8_0; 0];
}
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
pub static mut sBootThreadInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sIdleThread: OSThread =
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
pub static mut sIdleThreadStack: [u8_0; 1024] = [0; 1024];
#[no_mangle]
pub static mut sIdleThreadInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sBootThreadStack: [u8_0; 1024] = [0; 1024];
#[no_mangle]
pub unsafe extern "C" fn cleararena() {
    bzero(_dmadataSegmentStart.as_mut_ptr() as *mut libc::c_void,
          osMemSize.wrapping_sub((_dmadataSegmentStart.as_mut_ptr() as
                                      *mut libc::c_char).offset(-(0x80000000
                                                                      as
                                                                      libc::c_uint
                                                                      as
                                                                      isize))
                                     as u32_0));
}
#[no_mangle]
pub unsafe extern "C" fn bootproc() {
    StackCheck_Init(&mut sBootThreadInfo,
                    sBootThreadStack.as_mut_ptr() as *mut libc::c_void,
                    sBootThreadStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1024]>()
                                                             as libc::c_ulong
                                                             as isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    -(1 as libc::c_int),
                    b"boot\x00" as *const u8 as *const libc::c_char);
    osMemSize = osGetMemSize();
    cleararena();
    __osInitialize_common();
    __osInitialize_autodetect();
    gCartHandle = osCartRomInit();
    osDriveRomInit();
    isPrintfInit();
    Locale_Init();
    StackCheck_Init(&mut sIdleThreadInfo,
                    sIdleThreadStack.as_mut_ptr() as *mut libc::c_void,
                    sIdleThreadStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1024]>()
                                                             as libc::c_ulong
                                                             as isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    256 as libc::c_int,
                    b"idle\x00" as *const u8 as *const libc::c_char);
    osCreateThread(&mut sIdleThread, 1 as libc::c_int,
                   Some(Idle_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   0 as *mut libc::c_void,
                   sIdleThreadStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1024]>()
                                                            as libc::c_ulong
                                                            as isize) as
                       *mut libc::c_void, 10 as libc::c_int);
    osStartThread(&mut sIdleThread);
}
