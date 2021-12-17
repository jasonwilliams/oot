#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __osCleanupThread();
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
}
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
#[no_mangle]
pub static mut __osThreadTail: [*mut OSThread; 2] =
    unsafe {
        [0 as *const OSThread as *mut OSThread,
         -(1 as libc::c_int) as *mut OSThread]
    };
#[no_mangle]
pub static mut __osRunQueue: *mut OSThread =
    unsafe { __osThreadTail.as_ptr() as *mut _ as *mut OSThread };
#[no_mangle]
pub static mut __osActiveQueue: *mut OSThread =
    unsafe { __osThreadTail.as_ptr() as *mut _ as *mut OSThread };
#[no_mangle]
pub static mut __osRunningThread: *mut OSThread =
    0 as *const OSThread as *mut OSThread;
#[no_mangle]
pub static mut __osFaultedThread: *mut OSThread =
    0 as *const OSThread as *mut OSThread;
#[no_mangle]
pub unsafe extern "C" fn osCreateThread(mut thread: *mut OSThread,
                                        mut id: OSId,
                                        mut entry:
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut libc::c_void)
                                                       -> ()>,
                                        mut arg: *mut libc::c_void,
                                        mut sp: *mut libc::c_void,
                                        mut pri: OSPri) {
    let mut prevInt: u32_0 = 0;
    let mut mask: OSIntMask = 0;
    (*thread).id = id;
    (*thread).priority = pri;
    (*thread).next = 0 as *mut OSThread;
    (*thread).queue = 0 as *mut *mut OSThread;
    (*thread).context.pc =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           -> ()>, u32_0>(entry);
    (*thread).context.a0 = arg as u64_0;
    (*thread).context.sp =
        (sp as s32 as
             u64_0).wrapping_sub(16 as libc::c_int as libc::c_ulonglong);
    (*thread).context.ra =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                u64_0>(Some(__osCleanupThread as
                                                unsafe extern "C" fn()
                                                    -> ()));
    mask = 0x3fff01 as libc::c_int as OSIntMask;
    (*thread).context.sr =
        mask & 0xff01 as libc::c_int as libc::c_uint |
            2 as libc::c_int as libc::c_uint;
    (*thread).context.rcp =
        (mask & 0x3f0000 as libc::c_int as libc::c_uint) >> 16 as libc::c_int;
    (*thread).context.fpcsr =
        (0x1000000 as libc::c_int | 0x800 as libc::c_int) as u32_0;
    (*thread).fp = 0 as libc::c_int;
    (*thread).state = 1 as libc::c_int as u16_0;
    (*thread).flags = 0 as libc::c_int as u16_0;
    prevInt = __osDisableInt() as u32_0;
    (*thread).tlnext = __osActiveQueue;
    __osActiveQueue = thread;
    __osRestoreInt(prevInt as s32);
}
