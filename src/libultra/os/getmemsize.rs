#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn osGetMemSize() -> u32_0 {
    let mut ptr: *mut u32_0 = 0 as *mut u32_0;
    let mut size: u32_0 = 0x400000 as libc::c_int as u32_0;
    let mut data0: u32_0 = 0;
    let mut data1: u32_0 = 0;
    while size < 0x800000 as libc::c_int as libc::c_uint {
        ptr = (0xa0000000 as libc::c_uint).wrapping_add(size) as *mut u32_0;
        data0 = *ptr;
        data1 =
            *ptr.offset((0x100000 as libc::c_int / 4 as libc::c_int -
                             1 as libc::c_int) as isize);
        *ptr ^= !(0 as libc::c_int) as libc::c_uint;
        let ref mut fresh0 =
            *ptr.offset((0x100000 as libc::c_int / 4 as libc::c_int -
                             1 as libc::c_int) as isize);
        *fresh0 ^= !(0 as libc::c_int) as libc::c_uint;
        if *ptr != data0 ^ !(0 as libc::c_int) as libc::c_uint ||
               *ptr.offset((0x100000 as libc::c_int / 4 as libc::c_int -
                                1 as libc::c_int) as isize) !=
                   data1 ^ !(0 as libc::c_int) as libc::c_uint {
            return size
        }
        *ptr = data0;
        *ptr.offset((0x100000 as libc::c_int / 4 as libc::c_int -
                         1 as libc::c_int) as isize) = data1;
        size =
            (size as
                 libc::c_uint).wrapping_add(0x100000 as libc::c_int as
                                                libc::c_uint) as u32_0 as
                u32_0
    }
    return size;
}
