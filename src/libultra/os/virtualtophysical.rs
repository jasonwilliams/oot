#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osProbeTLB(_: *mut libc::c_void) -> u32_0;
}
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn osVirtualToPhysical(mut vaddr: *mut libc::c_void)
 -> u32_0 {
    if vaddr as u32_0 >= 0x80000000 as libc::c_uint &&
           (vaddr as u32_0) < 0xa0000000 as libc::c_uint {
        return vaddr as u32_0 & 0x1fffffff as libc::c_int as libc::c_uint
    }
    if vaddr as u32_0 >= 0xa0000000 as libc::c_uint &&
           (vaddr as u32_0) < 0xc0000000 as libc::c_uint {
        return vaddr as u32_0 & 0x1fffffff as libc::c_int as libc::c_uint
    }
    return __osProbeTLB(vaddr);
}
