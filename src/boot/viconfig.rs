#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osViExtendVStart(arg0: u32_0);
    #[no_mangle]
    fn osViBlack(active: u8_0);
    #[no_mangle]
    fn osViSetMode(mode: *mut OSViMode);
    #[no_mangle]
    fn osViSetSpecialFeatures(func: u32_0);
    #[no_mangle]
    fn osViSetYScale(scale: f32_0);
    #[no_mangle]
    fn osViSetXScale(value: f32_0);
    #[no_mangle]
    static mut osTvType: u32_0;
    #[no_mangle]
    static mut osViModePalLan1: OSViMode;
    #[no_mangle]
    static mut gViConfigMode: OSViMode;
    #[no_mangle]
    static mut gViConfigAdditionalScanLines: u8_0;
    #[no_mangle]
    static mut gViConfigFeatures: u32_0;
    #[no_mangle]
    static mut gViConfigXScale: f32_0;
    #[no_mangle]
    static mut gViConfigYScale: f32_0;
    #[no_mangle]
    static mut gViConfigUseDefault: vu8;
}
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type vu8 = u8_0;
pub type f32_0 = libc::c_float;
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
// this should probably go elsewhere but right now viconfig.o is the only object between idle and z_std_dma
#[no_mangle]
pub static mut gCartHandle: *mut OSPiHandle =
    0 as *const OSPiHandle as *mut OSPiHandle;
#[no_mangle]
pub unsafe extern "C" fn ViConfig_UpdateVi(mut mode: u32_0) {
    if mode != 0 as libc::c_int as libc::c_uint {
        osSyncPrintf(b"\x1b[43;30mosViSetYScale1(%f);\n\x1b[m\x00" as
                         *const u8 as *const libc::c_char,
                     1.0f32 as libc::c_double);
        if osTvType == 0 as libc::c_int as libc::c_uint {
            osViSetMode(&mut osViModePalLan1);
        }
        osViSetYScale(1.0f32);
    } else {
        osViSetMode(&mut gViConfigMode);
        if gViConfigAdditionalScanLines as libc::c_int != 0 as libc::c_int {
            osViExtendVStart(gViConfigAdditionalScanLines as u32_0);
        }
        if gViConfigFeatures != 0 as libc::c_int as libc::c_uint {
            osViSetSpecialFeatures(gViConfigFeatures);
        }
        if gViConfigXScale != 1.0f32 { osViSetXScale(gViConfigXScale); }
        if gViConfigYScale != 1.0f32 {
            osSyncPrintf(b"\x1b[43;30mosViSetYScale3(%f);\n\x1b[m\x00" as
                             *const u8 as *const libc::c_char,
                         gViConfigYScale as libc::c_double);
            osViSetYScale(gViConfigYScale);
        }
    }
    ::std::ptr::write_volatile(&mut gViConfigUseDefault as *mut vu8,
                               mode as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn ViConfig_UpdateBlack() {
    if gViConfigUseDefault as libc::c_int != 0 as libc::c_int {
        osViBlack(1 as libc::c_int as u8_0);
    } else { osViBlack(0 as libc::c_int as u8_0); };
}
