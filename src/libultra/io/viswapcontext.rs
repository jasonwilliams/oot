#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osVirtualToPhysical(vaddr: *mut libc::c_void) -> u32_0;
    #[no_mangle]
    static mut __additional_scanline: u32_0;
    #[no_mangle]
    static mut __osViNext: *mut OSViContext;
    #[no_mangle]
    static mut __osViCurr: *mut OSViContext;
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
pub struct __OSViScale {
    pub factor: f32_0,
    pub offset: u16_0,
    pub scale: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViContext {
    pub state: u16_0,
    pub retraceCount: u16_0,
    pub buffer: *mut libc::c_void,
    pub modep: *mut OSViMode,
    pub features: u32_0,
    pub mq: *mut OSMesgQueue,
    pub msg: *mut OSMesg,
    pub x: __OSViScale,
    pub y: __OSViScale,
}
#[no_mangle]
pub unsafe extern "C" fn __osViSwapContext() {
    let mut viMode: *mut OSViMode = 0 as *mut OSViMode;
    let mut viNext: *mut OSViContext = 0 as *mut OSViContext;
    let mut origin: u32_0 = 0;
    let mut hStart: u32_0 = 0;
    let mut vstart: u32_0 = 0;
    let mut sp34: u32_0 = 0;
    let mut field: u32_0 = 0;
    let mut s2: u32_0 = 0;
    field = 0 as libc::c_int as u32_0;
    viNext = __osViNext;
    viMode = (*viNext).modep;
    field =
        *((0x4400010 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) &
            1 as libc::c_int as libc::c_uint;
    s2 = osVirtualToPhysical((*viNext).buffer);
    origin = (*viMode).fldRegs[field as usize].origin.wrapping_add(s2);
    if (*viNext).state as libc::c_int & 2 as libc::c_int != 0 {
        (*viNext).x.scale |=
            (*viMode).comRegs.xScale & !(0xfff as libc::c_int) as libc::c_uint
    } else { (*viNext).x.scale = (*viMode).comRegs.xScale }
    if (*viNext).state as libc::c_int & 4 as libc::c_int != 0 {
        sp34 =
            (*viMode).fldRegs[field as usize].yScale &
                0xfff as libc::c_int as libc::c_uint;
        (*viNext).y.scale =
            ((*viNext).y.factor * sp34 as libc::c_float) as u32_0;
        (*viNext).y.scale |=
            (*viMode).fldRegs[field as usize].yScale &
                !(0xfff as libc::c_int) as libc::c_uint
    } else { (*viNext).y.scale = (*viMode).fldRegs[field as usize].yScale }
    vstart =
        (*viMode).fldRegs[field as
                              usize].vStart.wrapping_sub(__additional_scanline
                                                             <<
                                                             0x10 as
                                                                 libc::c_int).wrapping_add(__additional_scanline);
    hStart = (*viMode).comRegs.hStart;
    if (*viNext).state as libc::c_int & 0x20 as libc::c_int != 0 {
        hStart = 0 as libc::c_int as u32_0
    }
    if (*viNext).state as libc::c_int & 0x40 as libc::c_int != 0 {
        (*viNext).y.scale = 0 as libc::c_int as u32_0;
        origin = osVirtualToPhysical((*viNext).buffer)
    }
    if (*viNext).state as libc::c_int & 0x80 as libc::c_int != 0 {
        (*viNext).y.scale =
            (((*viNext).y.offset as libc::c_int) << 0x10 as libc::c_int &
                 0x3ff0000 as libc::c_int) as u32_0;
        origin = osVirtualToPhysical((*viNext).buffer)
    }
    ::std::ptr::write_volatile((0x4400004 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               origin);
    ::std::ptr::write_volatile((0x4400008 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viMode).comRegs.width);
    ::std::ptr::write_volatile((0x4400014 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viMode).comRegs.burst);
    ::std::ptr::write_volatile((0x4400018 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viMode).comRegs.vSync);
    ::std::ptr::write_volatile((0x440001c as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viMode).comRegs.hSync);
    ::std::ptr::write_volatile((0x4400020 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viMode).comRegs.leap);
    ::std::ptr::write_volatile((0x4400024 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               hStart);
    ::std::ptr::write_volatile((0x4400028 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               vstart);
    ::std::ptr::write_volatile((0x440002c as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viMode).fldRegs[field as usize].vBurst);
    ::std::ptr::write_volatile((0x440000c as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viMode).fldRegs[field as usize].vIntr);
    ::std::ptr::write_volatile((0x4400030 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viNext).x.scale);
    ::std::ptr::write_volatile((0x4400034 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viNext).y.scale);
    ::std::ptr::write_volatile((0x4400000 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (*viNext).features);
    __osViNext = __osViCurr;
    __osViCurr = viNext;
    *__osViNext = *__osViCurr;
}
