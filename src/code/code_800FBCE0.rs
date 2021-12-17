#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osDpGetStatus() -> u32_0;
    #[no_mangle]
    fn __osSpGetStatus() -> u32_0;
    #[no_mangle]
    fn __osSpSetStatus(status: u32_0);
    #[no_mangle]
    fn osDpSetStatus(status: u32_0);
}
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn func_800FBCE0() {
    let mut spStatus: u32_0 = __osSpGetStatus();
    let mut dpStatus: u32_0 = osDpGetStatus();
    osSyncPrintf(b"osSpGetStatus=%08x: \x00" as *const u8 as
                     *const libc::c_char, spStatus);
    if spStatus & 0x1 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"HALT \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x2 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"BROKE \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x4 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"DMA_BUSY \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x8 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"DMA_FULL \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x10 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"IO_FULL \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x20 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"SSTEP \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x40 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"INTR_BREAK \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x80 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"YIELD \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x100 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"YIELDED \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x200 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"TASKDONE \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x400 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"SIG3 \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x800 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"SIG4 \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x1000 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"SIG5 \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x2000 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"SIG6 \x00" as *const u8 as *const libc::c_char);
    }
    if spStatus & 0x4000 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"SIG7 \x00" as *const u8 as *const libc::c_char);
    }
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"osDpGetStatus=%08x:\x00" as *const u8 as
                     *const libc::c_char, dpStatus);
    if dpStatus & 0x1 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"XBUS_DMEM_DMA \x00" as *const u8 as
                         *const libc::c_char);
    }
    if dpStatus & 0x2 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"FREEZE \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x4 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"FLUSH \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x8 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"START_GCLK \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x10 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"TMEM_BUSY \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x20 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"PIPE_BUSY \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x40 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"CMD_BUSY \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x80 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"CBUF_READY \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x100 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"DMA_BUSY \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x200 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"END_VALID \x00" as *const u8 as *const libc::c_char);
    }
    if dpStatus & 0x400 as libc::c_int as libc::c_uint != 0 {
        osSyncPrintf(b"START_VALID \x00" as *const u8 as *const libc::c_char);
    }
    osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn func_800FBFD8() {
    func_800FBCE0();
    osDpSetStatus((0x8 as libc::c_int | 0x20 as libc::c_int) as u32_0);
    __osSpSetStatus((0x2 as libc::c_int | 0x4000 as libc::c_int |
                         0x80 as libc::c_int) as u32_0);
    func_800FBCE0();
}
