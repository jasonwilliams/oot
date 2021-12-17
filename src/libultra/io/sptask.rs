#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osVirtualToPhysical(vaddr: *mut libc::c_void) -> u32_0;
    #[no_mangle]
    fn bcopy(__src: *mut libc::c_void, __dest: *mut libc::c_void, __n: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __osSpRawStartDma(direction: s32, devAddr: *mut libc::c_void,
                         dramAddr: *mut libc::c_void, size: u32_0) -> s32;
    #[no_mangle]
    fn __osSpDeviceBusy() -> u32_0;
    #[no_mangle]
    fn __osSpSetPc(pc: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn __osSpSetStatus(status: u32_0);
}
pub type s32 = libc::c_int;
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
static mut sTmpTask: OSTask =
    OSTask{t:
               OSTask_t{type_0: 0,
                        flags: 0,
                        ucode_boot: 0 as *const u64_0 as *mut u64_0,
                        ucode_boot_size: 0,
                        ucode: 0 as *const u64_0 as *mut u64_0,
                        ucode_size: 0,
                        ucode_data: 0 as *const u64_0 as *mut u64_0,
                        ucode_data_size: 0,
                        dram_stack: 0 as *const u64_0 as *mut u64_0,
                        dram_stack_size: 0,
                        output_buff: 0 as *const u64_0 as *mut u64_0,
                        output_buff_size: 0 as *const u64_0 as *mut u64_0,
                        data_ptr: 0 as *const u64_0 as *mut u64_0,
                        data_size: 0,
                        yield_data_ptr: 0 as *const u64_0 as *mut u64_0,
                        yield_data_size: 0,},};
#[no_mangle]
pub unsafe extern "C" fn _VirtualToPhysicalTask(mut intp: *mut OSTask)
 -> *mut OSTask {
    let mut tp: *mut OSTask = &mut sTmpTask;
    bcopy(intp as *mut libc::c_void, tp as *mut libc::c_void,
          ::std::mem::size_of::<OSTask>() as libc::c_ulong);
    if !(*tp).t.ucode.is_null() {
        (*tp).t.ucode =
            osVirtualToPhysical((*tp).t.ucode as *mut libc::c_void) as
                *mut libc::c_void as *mut u64_0
    }
    if !(*tp).t.ucode_data.is_null() {
        (*tp).t.ucode_data =
            osVirtualToPhysical((*tp).t.ucode_data as *mut libc::c_void) as
                *mut libc::c_void as *mut u64_0
    }
    if !(*tp).t.dram_stack.is_null() {
        (*tp).t.dram_stack =
            osVirtualToPhysical((*tp).t.dram_stack as *mut libc::c_void) as
                *mut libc::c_void as *mut u64_0
    }
    if !(*tp).t.output_buff.is_null() {
        (*tp).t.output_buff =
            osVirtualToPhysical((*tp).t.output_buff as *mut libc::c_void) as
                *mut libc::c_void as *mut u64_0
    }
    if !(*tp).t.output_buff_size.is_null() {
        (*tp).t.output_buff_size =
            osVirtualToPhysical((*tp).t.output_buff_size as *mut libc::c_void)
                as *mut libc::c_void as *mut u64_0
    }
    if !(*tp).t.data_ptr.is_null() {
        (*tp).t.data_ptr =
            osVirtualToPhysical((*tp).t.data_ptr as *mut libc::c_void) as
                *mut libc::c_void as *mut u64_0
    }
    if !(*tp).t.yield_data_ptr.is_null() {
        (*tp).t.yield_data_ptr =
            osVirtualToPhysical((*tp).t.yield_data_ptr as *mut libc::c_void)
                as *mut libc::c_void as *mut u64_0
    }
    return tp;
}
#[no_mangle]
pub unsafe extern "C" fn osSpTaskLoad(mut intp: *mut OSTask) {
    let mut tp: *mut OSTask = _VirtualToPhysicalTask(intp);
    if (*tp).t.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*tp).t.ucode_data = (*tp).t.yield_data_ptr;
        (*tp).t.ucode_data_size = (*tp).t.yield_data_size;
        (*intp).t.flags &= !(0x1 as libc::c_int) as libc::c_uint;
        if (*tp).t.flags & 0x4 as libc::c_int as libc::c_uint != 0 {
            (*tp).t.ucode =
                *((((*intp).t.yield_data_ptr as
                        u32_0).wrapping_add(0xc00 as libc::c_int as
                                                libc::c_uint).wrapping_sub(4
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                       | 0xa0000000 as libc::c_uint) as *mut u32_0) as
                    *mut u64_0
        }
    }
    osWritebackDCache(tp as *mut libc::c_void,
                      ::std::mem::size_of::<OSTask>() as libc::c_ulong as
                          s32);
    __osSpSetStatus((0x200 as libc::c_int | 0x800 as libc::c_int |
                         0x2000 as libc::c_int | 0x100 as libc::c_int) as
                        u32_0);
    while __osSpSetPc(0x4001000 as libc::c_int as *mut libc::c_void) ==
              -(1 as libc::c_int) {
    }
    while __osSpRawStartDma(1 as libc::c_int,
                            (0x4001000 as libc::c_int as
                                 libc::c_uint).wrapping_sub(::std::mem::size_of::<OSTask>()
                                                                as
                                                                libc::c_ulong)
                                as *mut libc::c_void, tp as *mut libc::c_void,
                            ::std::mem::size_of::<OSTask>() as libc::c_ulong)
              == -(1 as libc::c_int) {
    }
    while __osSpDeviceBusy() != 0 { }
    while __osSpRawStartDma(1 as libc::c_int,
                            0x4001000 as libc::c_int as *mut libc::c_void,
                            (*tp).t.ucode_boot as *mut libc::c_void,
                            (*tp).t.ucode_boot_size) == -(1 as libc::c_int) {
    };
}
#[no_mangle]
pub unsafe extern "C" fn osSpTaskStartGo(mut tp: *mut OSTask) {
    while __osSpDeviceBusy() != 0 { }
    __osSpSetStatus((0x100 as libc::c_int | 0x20 as libc::c_int |
                         0x4 as libc::c_int | 0x1 as libc::c_int) as u32_0);
}
