#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osSpGetStatus() -> u32_0;
}
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
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
pub type OSYieldResult = u32_0;
#[no_mangle]
pub unsafe extern "C" fn osSpTaskYielded(mut task: *mut OSTask)
 -> OSYieldResult {
    let mut ret: u32_0 = 0;
    let mut status: u32_0 = __osSpGetStatus();
    if status & 0x100 as libc::c_int as libc::c_uint != 0 {
        ret = 0x1 as libc::c_int as u32_0
    } else { ret = 0 as libc::c_int as u32_0 }
    if status & 0x80 as libc::c_int as libc::c_uint != 0 {
        (*task).t.flags |= ret;
        (*task).t.flags &= !(0x2 as libc::c_int) as libc::c_uint
    }
    return ret;
}
