#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type u32_0 = libc::c_uint;
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
#[no_mangle]
pub static mut osViModePalLan1: OSViMode =
    {
        let mut init =
            OSViMode{type_0: 0x10 as libc::c_int as u8_0,
                     comRegs:
                         {
                             let mut init =
                                 OSViCommonRegs{ctrl:
                                                    0x311e as libc::c_int as
                                                        u32_0,
                                                width:
                                                    320 as libc::c_int as
                                                        u32_0,
                                                burst:
                                                    0x4541e3a as libc::c_int
                                                        as u32_0,
                                                vSync:
                                                    0x271 as libc::c_int as
                                                        u32_0,
                                                hSync:
                                                    0x170c69 as libc::c_int as
                                                        u32_0,
                                                leap:
                                                    0xc6f0c6d as libc::c_int
                                                        as u32_0,
                                                hStart:
                                                    0x800300 as libc::c_int as
                                                        u32_0,
                                                xScale:
                                                    0x200 as libc::c_int as
                                                        u32_0,
                                                vCurrent:
                                                    0 as libc::c_int as
                                                        u32_0,};
                             init
                         },
                     fldRegs:
                         [{
                              let mut init =
                                  OSViFieldRegs{origin:
                                                    0x280 as libc::c_int as
                                                        u32_0,
                                                yScale:
                                                    0x400 as libc::c_int as
                                                        u32_0,
                                                vStart:
                                                    0x5f0239 as libc::c_int as
                                                        u32_0,
                                                vBurst:
                                                    0x9026b as libc::c_int as
                                                        u32_0,
                                                vIntr:
                                                    2 as libc::c_int as
                                                        u32_0,};
                              init
                          },
                          {
                              let mut init =
                                  OSViFieldRegs{origin:
                                                    0x280 as libc::c_int as
                                                        u32_0,
                                                yScale:
                                                    0x400 as libc::c_int as
                                                        u32_0,
                                                vStart:
                                                    0x5f0239 as libc::c_int as
                                                        u32_0,
                                                vBurst:
                                                    0x9026b as libc::c_int as
                                                        u32_0,
                                                vIntr:
                                                    2 as libc::c_int as
                                                        u32_0,};
                              init
                          }],};
        init
    };
