#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osVirtualToPhysical(vaddr: *mut libc::c_void) -> u32_0;
    #[no_mangle]
    fn osInvalDCache(vaddr: *mut libc::c_void, nbytes: s32);
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __osSiRawStartDma(mut dir: s32,
                                           mut addr: *mut libc::c_void)
 -> s32 {
    if *((0x4800018 as libc::c_int as libc::c_uint |
              0xa0000000 as libc::c_uint) as *mut u32_0) &
           (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint != 0 {
        return -(1 as libc::c_int)
    }
    if dir == 1 as libc::c_int {
        osWritebackDCache(addr, 0x40 as libc::c_int);
    }
    let ref mut fresh0 =
        *((0x4800000 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut *mut libc::c_void);
    *fresh0 =
        osVirtualToPhysical(addr) as *mut libc::c_void as *mut libc::c_void;
    if dir == 0 as libc::c_int {
        let ref mut fresh1 =
            *((0x4800004 as libc::c_int as libc::c_uint |
                   0xa0000000 as libc::c_uint) as *mut *mut libc::c_void);
        *fresh1 =
            0x1fc007c0 as libc::c_int as *mut libc::c_void as
                *mut libc::c_void
    } else {
        let ref mut fresh2 =
            *((0x4800010 as libc::c_int as libc::c_uint |
                   0xa0000000 as libc::c_uint) as *mut *mut libc::c_void);
        *fresh2 =
            0x1fc007c0 as libc::c_int as *mut libc::c_void as
                *mut libc::c_void
    }
    if dir == 0 as libc::c_int { osInvalDCache(addr, 0x40 as libc::c_int); }
    return 0 as libc::c_int;
}
