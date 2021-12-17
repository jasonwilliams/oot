#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uObjBg_t {
    pub imageX: u16_0,
    pub imageW: u16_0,
    pub frameX: s16,
    pub frameW: u16_0,
    pub imageY: u16_0,
    pub imageH: u16_0,
    pub frameY: s16,
    pub frameH: u16_0,
    pub imagePtr: *mut u64_0,
    pub imageLoad: u16_0,
    pub imageFmt: u8_0,
    pub imageSiz: u8_0,
    pub imagePal: u16_0,
    pub imageFlip: u16_0,
    pub tmemW: u16_0,
    pub tmemH: u16_0,
    pub tmemLoadSH: u16_0,
    pub tmemLoadTH: u16_0,
    pub tmemSizeW: u16_0,
    pub tmemSize: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uObjScaleBg_t {
    pub imageX: u16_0,
    pub imageW: u16_0,
    pub frameX: s16,
    pub frameW: u16_0,
    pub imageY: u16_0,
    pub imageH: u16_0,
    pub frameY: s16,
    pub frameH: u16_0,
    pub imagePtr: *mut u64_0,
    pub imageLoad: u16_0,
    pub imageFmt: u8_0,
    pub imageSiz: u8_0,
    pub imagePal: u16_0,
    pub imageFlip: u16_0,
    pub scaleW: u16_0,
    pub scaleH: u16_0,
    pub imageYorig: s32,
    pub padding: [u8_0; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union uObjBg {
    pub b: uObjBg_t,
    pub s: uObjScaleBg_t,
    pub force_structure_alignment: libc::c_longlong,
}
#[no_mangle]
pub unsafe extern "C" fn guS2DInitBg(mut bg: *mut uObjBg) {
    let mut size: u32_0 = 0; // G_BGLT_LOADTILE
    let mut tmem: s32 =
        if (*bg).b.imageFmt as libc::c_int == 2 as libc::c_int {
            0x100 as libc::c_int
        } else { 0x200 as libc::c_int };
    let mut shift: u16_0 =
        (6 as libc::c_int - (*bg).b.imageSiz as libc::c_int) as u16_0;
    if (*bg).b.imageLoad as libc::c_int == 0x33 as libc::c_int {
        (*bg).b.tmemW =
            ((*bg).b.imageW as libc::c_int >> shift as libc::c_int) as u16_0;
        (*bg).b.tmemH =
            (tmem / (*bg).b.tmemW as libc::c_int * 4 as libc::c_int) as u16_0;
        (*bg).b.tmemSizeW =
            ((*bg).b.tmemW as libc::c_int * 2 as libc::c_int) as u16_0;
        (*bg).b.tmemSize =
            ((*bg).b.tmemH as libc::c_int * (*bg).b.tmemSizeW as libc::c_int)
                as u16_0;
        (*bg).b.tmemLoadSH =
            (((*bg).b.tmemSize as libc::c_int >> 1 as libc::c_int) -
                 1 as libc::c_int) as u16_0;
        (*bg).b.tmemLoadTH =
            (0x7ff as libc::c_int / (*bg).b.tmemW as libc::c_int +
                 1 as libc::c_int) as u16_0
    } else {
        (*bg).b.tmemW =
            (((*bg).b.frameW as libc::c_int >> shift as libc::c_int) +
                 3 as libc::c_int) as u16_0;
        (*bg).b.tmemH =
            (tmem / (*bg).b.tmemW as libc::c_int * 4 as libc::c_int) as u16_0;
        (*bg).b.tmemSizeW =
            (((*bg).b.imageW as libc::c_int >> shift as libc::c_int) *
                 2 as libc::c_int) as u16_0;
        size =
            ((*bg).b.tmemH as libc::c_int * (*bg).b.tmemSizeW as libc::c_int)
                as u32_0;
        (*bg).b.tmemSize = (size >> 16 as libc::c_int) as u16_0;
        (*bg).b.tmemLoadSH =
            (size >> 0 as libc::c_int & 0xffff as libc::c_int as libc::c_uint)
                as u16_0;
        (*bg).b.tmemLoadTH =
            ((*bg).b.tmemH as libc::c_int - 1 as libc::c_int) as u16_0
    };
}
