#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osVirtualToPhysical(vaddr: *mut libc::c_void) -> u32_0;
    #[no_mangle]
    fn __osSpDeviceBusy() -> u32_0;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __osSpRawStartDma(mut direction: s32,
                                           mut devAddr: *mut libc::c_void,
                                           mut dramAddr: *mut libc::c_void,
                                           mut size: u32_0) -> s32 {
    if __osSpDeviceBusy() != 0 { return -(1 as libc::c_int) }
    ::std::ptr::write_volatile((0x4040000 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               devAddr as u32_0);
    ::std::ptr::write_volatile((0x4040004 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               osVirtualToPhysical(dramAddr));
    if direction == 0 as libc::c_int {
        ::std::ptr::write_volatile((0x404000c as libc::c_int as libc::c_uint |
                                        0xa0000000 as libc::c_uint) as
                                       *mut u32_0,
                                   size.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint))
    } else {
        ::std::ptr::write_volatile((0x4040008 as libc::c_int as libc::c_uint |
                                        0xa0000000 as libc::c_uint) as
                                       *mut u32_0,
                                   size.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint))
    }
    return 0 as libc::c_int;
}
