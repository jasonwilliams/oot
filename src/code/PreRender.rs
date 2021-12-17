#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
                                 ptr: *mut libc::c_void,
                                 file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn ListAlloc_Init(this: *mut ListAlloc) -> *mut ListAlloc;
    #[no_mangle]
    fn ListAlloc_FreeAll(this: *mut ListAlloc);
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
}
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tri {
    pub flag: libc::c_uchar,
    pub v: [libc::c_uchar; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gdma {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "par", ty = "libc::c_uint", bits = "8..=15")]
    #[bitfield(name = "len", ty = "libc::c_uint", bits = "16..=31")]
    pub cmd_par_len: [u8; 4],
    pub addr: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gtri {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad: [u8; 4],
    pub tri: Tri,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gpopmtx {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "8..=31")]
    #[bitfield(name = "pad2", ty = "libc::c_int", bits = "32..=55")]
    #[bitfield(name = "param", ty = "libc::c_uchar", bits = "56..=63")]
    pub cmd_pad1_pad2_param: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsegment {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "mw_index", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "number", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "32..=39")]
    #[bitfield(name = "base", ty = "libc::c_int", bits = "40..=63")]
    pub cmd_pad0_mw_index_number_pad1_base: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetothermodeL {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "sft", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "len", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "data", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_pad0_sft_len_data: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetothermodeH {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "sft", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "len", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "data", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_pad0_sft_len_data: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gtexture {
    pub cmd: libc::c_uchar,
    pub lodscale: libc::c_uchar,
    pub tile: libc::c_uchar,
    pub on: libc::c_uchar,
    pub s: libc::c_ushort,
    pub t: libc::c_ushort,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gline3D {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad: [u8; 4],
    pub line: Tri,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gperspnorm {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad1: [u8; 4],
    pub pad2: libc::c_short,
    pub scale: libc::c_short,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetimg {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "fmt", ty = "libc::c_uint", bits = "8..=10")]
    #[bitfield(name = "siz", ty = "libc::c_uint", bits = "11..=12")]
    #[bitfield(name = "pad", ty = "libc::c_uint", bits = "13..=19")]
    #[bitfield(name = "wd", ty = "libc::c_uint", bits = "20..=31")]
    pub cmd_fmt_siz_pad_wd: [u8; 4],
    pub dram: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetcombine {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "muxs0", ty = "libc::c_uint", bits = "8..=31")]
    #[bitfield(name = "muxs1", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_muxs0_muxs1: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetcolor {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    pub cmd: [u8; 1],
    pub pad: libc::c_uchar,
    pub prim_min_level: libc::c_uchar,
    pub prim_level: libc::c_uchar,
    pub color: libc::c_ulong,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gfillrect {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "x0", ty = "libc::c_int", bits = "8..=17")]
    #[bitfield(name = "x0frac", ty = "libc::c_int", bits = "18..=19")]
    #[bitfield(name = "y0", ty = "libc::c_int", bits = "20..=29")]
    #[bitfield(name = "y0frac", ty = "libc::c_int", bits = "30..=31")]
    #[bitfield(name = "pad", ty = "libc::c_uint", bits = "32..=39")]
    #[bitfield(name = "x1", ty = "libc::c_int", bits = "40..=49")]
    #[bitfield(name = "x1frac", ty = "libc::c_int", bits = "50..=51")]
    #[bitfield(name = "y1", ty = "libc::c_int", bits = "52..=61")]
    #[bitfield(name = "y1frac", ty = "libc::c_int", bits = "62..=63")]
    pub cmd_x0_x0frac_y0_y0frac_pad_x1_x1frac_y1_y1frac: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsettile {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "fmt", ty = "libc::c_uint", bits = "8..=10")]
    #[bitfield(name = "siz", ty = "libc::c_uint", bits = "11..=12")]
    #[bitfield(name = "pad0", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "line", ty = "libc::c_uint", bits = "14..=22")]
    #[bitfield(name = "tmem", ty = "libc::c_uint", bits = "23..=31")]
    #[bitfield(name = "pad1", ty = "libc::c_uint", bits = "32..=36")]
    #[bitfield(name = "tile", ty = "libc::c_uint", bits = "37..=39")]
    #[bitfield(name = "palette", ty = "libc::c_uint", bits = "40..=43")]
    #[bitfield(name = "ct", ty = "libc::c_uint", bits = "44..=44")]
    #[bitfield(name = "mt", ty = "libc::c_uint", bits = "45..=45")]
    #[bitfield(name = "maskt", ty = "libc::c_uint", bits = "46..=49")]
    #[bitfield(name = "shiftt", ty = "libc::c_uint", bits = "50..=53")]
    #[bitfield(name = "cs", ty = "libc::c_uint", bits = "54..=54")]
    #[bitfield(name = "ms", ty = "libc::c_uint", bits = "55..=55")]
    #[bitfield(name = "masks", ty = "libc::c_uint", bits = "56..=59")]
    #[bitfield(name = "shifts", ty = "libc::c_uint", bits = "60..=63")]
    pub cmd_fmt_siz_pad0_line_tmem_pad1_tile_palette_ct_mt_maskt_shiftt_cs_ms_masks_shifts: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gloadtile {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "sl", ty = "libc::c_uint", bits = "8..=19")]
    #[bitfield(name = "tl", ty = "libc::c_uint", bits = "20..=31")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "32..=36")]
    #[bitfield(name = "tile", ty = "libc::c_uint", bits = "37..=39")]
    #[bitfield(name = "sh", ty = "libc::c_uint", bits = "40..=51")]
    #[bitfield(name = "th", ty = "libc::c_uint", bits = "52..=63")]
    pub cmd_sl_tl_pad_tile_sh_th: [u8; 8],
}
pub type Gsettilesize = Gloadtile;
pub type Gloadtlut = Gloadtile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gwords {
    pub w0: libc::c_uint,
    pub w1: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Gfx {
    pub words: Gwords,
    pub dma: Gdma,
    pub tri: Gtri,
    pub line: Gline3D,
    pub popmtx: Gpopmtx,
    pub segment: Gsegment,
    pub setothermodeH: GsetothermodeH,
    pub setothermodeL: GsetothermodeL,
    pub texture: Gtexture,
    pub perspnorm: Gperspnorm,
    pub setimg: Gsetimg,
    pub setcombine: Gsetcombine,
    pub setcolor: Gsetcolor,
    pub fillrect: Gfillrect,
    pub settile: Gsettile,
    pub loadtile: Gloadtile,
    pub settilesize: Gsettilesize,
    pub loadtlut: Gloadtlut,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Color_RGBA16 {
    pub c2rust_unnamed: C2RustUnnamed,
    pub rgba: u16_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed {
    #[bitfield(name = "r", ty = "u16_0", bits = "0..=4")]
    #[bitfield(name = "g", ty = "u16_0", bits = "5..=9")]
    #[bitfield(name = "b", ty = "u16_0", bits = "10..=14")]
    #[bitfield(name = "a", ty = "u16_0", bits = "15..=15")]
    pub r_g_b_a: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreRender {
    pub width: s32,
    pub height: s32,
    pub widthSave: s32,
    pub heightSave: s32,
    pub fbuf: *mut u16_0,
    pub fbufSave: *mut u16_0,
    pub cvgSave: *mut u8_0,
    pub zbuf: *mut u16_0,
    pub zbufSave: *mut u16_0,
    pub ulxSave: s32,
    pub ulySave: s32,
    pub lrxSave: s32,
    pub lrySave: s32,
    pub ulx: s32,
    pub uly: s32,
    pub lrx: s32,
    pub lry: s32,
    pub alloc: ListAlloc,
    pub unk_4C: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListAlloc {
    pub prev: *mut ListAlloc,
    pub next: *mut ListAlloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameInfo {
    pub regPage: s32,
    pub regGroup: s32,
    pub regCur: s32,
    pub dpadLast: s32,
    pub repeat: s32,
    pub data: [s16; 2784],
}
#[no_mangle]
pub unsafe extern "C" fn PreRender_SetValuesSave(mut this: *mut PreRender,
                                                 mut width: u32_0,
                                                 mut height: u32_0,
                                                 mut fbuf: *mut libc::c_void,
                                                 mut zbuf: *mut libc::c_void,
                                                 mut cvg: *mut libc::c_void) {
    (*this).widthSave = width as s32;
    (*this).heightSave = height as s32;
    (*this).fbufSave = fbuf as *mut u16_0;
    (*this).cvgSave = cvg as *mut u8_0;
    (*this).zbufSave = zbuf as *mut u16_0;
    (*this).ulxSave = 0 as libc::c_int;
    (*this).ulySave = 0 as libc::c_int;
    (*this).lrxSave =
        width.wrapping_sub(1 as libc::c_int as libc::c_uint) as s32;
    (*this).lrySave =
        height.wrapping_sub(1 as libc::c_int as libc::c_uint) as s32;
}
#[no_mangle]
pub unsafe extern "C" fn PreRender_Init(mut this: *mut PreRender) {
    bzero(this as *mut libc::c_void,
          ::std::mem::size_of::<PreRender>() as libc::c_ulong);
    ListAlloc_Init(&mut (*this).alloc);
}
#[no_mangle]
pub unsafe extern "C" fn PreRender_SetValues(mut this: *mut PreRender,
                                             mut width: u32_0,
                                             mut height: u32_0,
                                             mut fbuf: *mut libc::c_void,
                                             mut zbuf: *mut libc::c_void) {
    (*this).width = width as s32;
    (*this).height = height as s32;
    (*this).fbuf = fbuf as *mut u16_0;
    (*this).zbuf = zbuf as *mut u16_0;
    (*this).ulx = 0 as libc::c_int;
    (*this).uly = 0 as libc::c_int;
    (*this).lrx = width.wrapping_sub(1 as libc::c_int as libc::c_uint) as s32;
    (*this).lry =
        height.wrapping_sub(1 as libc::c_int as libc::c_uint) as s32;
}
#[no_mangle]
pub unsafe extern "C" fn PreRender_Destroy(mut this: *mut PreRender) {
    ListAlloc_FreeAll(&mut (*this).alloc);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C0F28(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx,
                                       mut buf: *mut libc::c_void,
                                       mut bufSave: *mut libc::c_void) {
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    let mut x: s32 = 0;
    let mut x2: s32 = 0;
    let mut dx: s32 = 0;
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 215 as libc::c_int);
    LogUtils_CheckNullPointer(b"glistpp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfxp as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 216 as libc::c_int);
    gfx = *gfxp;
    LogUtils_CheckNullPointer(b"glistp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfx as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 218 as libc::c_int);
    let fresh0 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh1 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((0 as libc::c_int) << 4 as libc::c_int |
                  (0 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (0 as libc::c_int) << 9 as libc::c_int |
                  (0 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (2 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (0 as libc::c_int) << 2 as libc::c_int |
             (0 as libc::c_int) << 30 as libc::c_int |
             (0 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0 as libc::c_int) << 28 as libc::c_int |
             (0 as libc::c_int) << 24 as libc::c_int |
             (0 as libc::c_int) << 20 as libc::c_int |
             (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh2 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = bufSave as libc::c_uint;
    let fresh3 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*this).width as libc::c_float * 4.0f32) as libc::c_int as u32_0
                 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*this).height as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    dx = 0x1000 as libc::c_int / ((*this).width * 2 as libc::c_int);
    x = (*this).height;
    x2 = 0 as libc::c_int;
    while x > 0 as libc::c_int {
        let mut uls: s32 = 0 as libc::c_int;
        let mut lrs: s32 = (*this).width - 1 as libc::c_int;
        let mut ult: s32 = 0;
        let mut lrt: s32 = 0;
        dx = if dx > x { x } else { dx };
        ult = x2;
        lrt = ult + dx - 1 as libc::c_int;
        let fresh4 = gfx;
        gfx = gfx.offset(1);
        let mut _g_3: *mut Gfx = fresh4;
        (*_g_3).words.w0 =
            (0xfd as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((*this).width - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 = buf as libc::c_uint;
        let fresh5 = gfx;
        gfx = gfx.offset(1);
        let mut _g_4: *mut Gfx = fresh5;
        (*_g_4).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                      7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_4).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh6 = gfx;
        gfx = gfx.offset(1);
        let mut _g_5: *mut Gfx = fresh6;
        (*_g_5).words.w0 =
            (0xe6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh7 = gfx;
        gfx = gfx.offset(1);
        let mut _g_6: *mut Gfx = fresh7;
        (*_g_6).words.w0 =
            (0xf4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_6).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((lrs << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh8 = gfx;
        gfx = gfx.offset(1);
        let mut _g_7: *mut Gfx = fresh8;
        (*_g_7).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_7).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh9 = gfx;
        gfx = gfx.offset(1);
        let mut _g_8: *mut Gfx = fresh9;
        (*_g_8).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                      7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_8).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh10 = gfx;
        gfx = gfx.offset(1);
        let mut _g_9: *mut Gfx = fresh10;
        (*_g_9).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_9).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((lrs << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh11 = gfx;
        gfx = gfx.offset(1);
        let mut _g_10: *mut Gfx = fresh11;
        (*_g_10).words.w0 =
            (0xe4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((lrs << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_10).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh12 = gfx;
        gfx = gfx.offset(1);
        let mut _g_11: *mut Gfx = fresh12;
        (*_g_11).words.w0 =
            (0xe1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_11).words.w1 =
            ((uls << 5 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                ((ult << 5 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh13 = gfx;
        gfx = gfx.offset(1);
        let mut _g_12: *mut Gfx = fresh13;
        (*_g_12).words.w0 =
            (0xf1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_12).words.w1 =
            (((4 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        x -= dx;
        x2 += dx
    }
    let fresh14 = gfx;
    gfx = gfx.offset(1);
    let mut _g_13: *mut Gfx = fresh14;
    (*_g_13).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_13).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh15 = gfx;
    gfx = gfx.offset(1);
    let mut _g_14: *mut Gfx = fresh15;
    (*_g_14).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_14).words.w1 = (*this).fbuf as libc::c_uint;
    *gfxp = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C1258(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    let mut y: s32 = 0;
    let mut y2: s32 = 0;
    let mut dy: s32 = 0;
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 278 as libc::c_int);
    LogUtils_CheckNullPointer(b"glistpp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfxp as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 279 as libc::c_int);
    gfx = *gfxp;
    LogUtils_CheckNullPointer(b"glistp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfx as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 281 as libc::c_int);
    let fresh16 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh16;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh17 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh17;
    (*_g_0).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((0 as libc::c_int) << 4 as libc::c_int |
                  (0 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (0 as libc::c_int) << 9 as libc::c_int |
                  (0 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (2 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (0 as libc::c_int) << 2 as libc::c_int |
             (0 as libc::c_int) << 30 as libc::c_int |
             (0 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0 as libc::c_int) << 28 as libc::c_int |
             (0 as libc::c_int) << 24 as libc::c_int |
             (0 as libc::c_int) << 20 as libc::c_int |
             (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh18 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh18;
    (*_g_1).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = (*this).fbuf as libc::c_uint;
    let fresh19 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh19;
    (*_g_2).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*this).ulx as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*this).uly as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((((*this).lrx + 1 as libc::c_int) as libc::c_float * 4.0f32) as
                 libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((((*this).lry + 1 as libc::c_int) as libc::c_float * 4.0f32) as
                 libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    dy =
        0x1000 as libc::c_int /
            (((*this).lrxSave - (*this).ulxSave + 1 as libc::c_int) *
                 2 as libc::c_int);
    y = (*this).lrySave - (*this).ulySave + 1 as libc::c_int;
    y2 = 0 as libc::c_int;
    while y > 0 as libc::c_int {
        let mut ult: s32 = 0;
        let mut lrt: s32 = 0;
        let mut uly: s32 = 0;
        dy = if dy > y { y } else { dy };
        ult = (*this).ulySave + y2;
        lrt = ult + dy - 1 as libc::c_int;
        uly = (*this).uly + y2;
        let fresh20 = gfx;
        gfx = gfx.offset(1);
        let mut _g_3: *mut Gfx = fresh20;
        (*_g_3).words.w0 =
            (0xfd as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((*this).widthSave - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 = (*this).fbufSave as libc::c_uint;
        let fresh21 = gfx;
        gfx = gfx.offset(1);
        let mut _g_4: *mut Gfx = fresh21;
        (*_g_4).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((((*this).lrxSave - (*this).ulxSave + 1 as libc::c_int) *
                      2 as libc::c_int + 7 as libc::c_int >> 3 as libc::c_int)
                     as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_4).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh22 = gfx;
        gfx = gfx.offset(1);
        let mut _g_5: *mut Gfx = fresh22;
        (*_g_5).words.w0 =
            (0xe6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh23 = gfx;
        gfx = gfx.offset(1);
        let mut _g_6: *mut Gfx = fresh23;
        (*_g_6).words.w0 =
            (0xf4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*this).ulxSave << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_6).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*this).lrxSave << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh24 = gfx;
        gfx = gfx.offset(1);
        let mut _g_7: *mut Gfx = fresh24;
        (*_g_7).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_7).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh25 = gfx;
        gfx = gfx.offset(1);
        let mut _g_8: *mut Gfx = fresh25;
        (*_g_8).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((((*this).lrxSave - (*this).ulxSave + 1 as libc::c_int) *
                      2 as libc::c_int + 7 as libc::c_int >> 3 as libc::c_int)
                     as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_8).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh26 = gfx;
        gfx = gfx.offset(1);
        let mut _g_9: *mut Gfx = fresh26;
        (*_g_9).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*this).ulxSave << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_9).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*this).lrxSave << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh27 = gfx;
        gfx = gfx.offset(1);
        let mut _g_10: *mut Gfx = fresh27;
        (*_g_10).words.w0 =
            (0xe4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*this).lrx << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((uly + dy - 1 as libc::c_int) << 2 as libc::c_int) as u32_0
                     &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_10).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*this).ulx << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((uly << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh28 = gfx;
        gfx = gfx.offset(1);
        let mut _g_11: *mut Gfx = fresh28;
        (*_g_11).words.w0 =
            (0xe1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_11).words.w1 =
            (((*this).ulxSave << 5 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                ((ult << 5 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh29 = gfx;
        gfx = gfx.offset(1);
        let mut _g_12: *mut Gfx = fresh29;
        (*_g_12).words.w0 =
            (0xf1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_12).words.w1 =
            (((4 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        y -= dy;
        y2 += dy
    }
    let fresh30 = gfx;
    gfx = gfx.offset(1);
    let mut _g_13: *mut Gfx = fresh30;
    (*_g_13).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_13).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh31 = gfx;
    gfx = gfx.offset(1);
    let mut _g_14: *mut Gfx = fresh31;
    (*_g_14).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_14).words.w1 = (*this).fbuf as libc::c_uint;
    let fresh32 = gfx;
    gfx = gfx.offset(1);
    let mut _g_15: *mut Gfx = fresh32;
    (*_g_15).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_15).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*this).width as libc::c_float * 4.0f32) as libc::c_int as u32_0
                 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*this).height as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    *gfxp = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C170C(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx,
                                       mut fbuf: *mut libc::c_void,
                                       mut fbufSave: *mut libc::c_void,
                                       mut r: u32_0, mut g: u32_0,
                                       mut b: u32_0, mut a: u32_0) {
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    let mut x: s32 = 0;
    let mut x2: s32 = 0;
    let mut dx: s32 = 0;
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 343 as libc::c_int);
    LogUtils_CheckNullPointer(b"glistpp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfxp as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 344 as libc::c_int);
    gfx = *gfxp;
    LogUtils_CheckNullPointer(b"glistp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfx as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 346 as libc::c_int);
    let fresh33 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh33;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh34 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh34;
    (*_g_0).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((3 as libc::c_int) << 4 as libc::c_int |
                  (3 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (6 as libc::c_int) << 9 as libc::c_int |
                  (0 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (0 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int | 0 as libc::c_int |
             0x4000 as libc::c_int | 0 as libc::c_int |
             (0 as libc::c_int) << 30 as libc::c_int |
             (3 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (2 as libc::c_int) << 18 as libc::c_int | 0 as libc::c_int |
             0x4000 as libc::c_int | 0 as libc::c_int |
             (0 as libc::c_int) << 28 as libc::c_int |
             (3 as libc::c_int) << 24 as libc::c_int |
             (0 as libc::c_int) << 20 as libc::c_int |
             (2 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh35 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh35;
    (*_g_1).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 =
        (r &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (g &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (b &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (a &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh36 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh36;
    (*_g_2).words.w0 =
        (0xfc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((31 as libc::c_int as u32_0 &
                   (((0x1 as libc::c_int) << 4 as libc::c_int) -
                        1 as libc::c_int) as libc::c_uint) <<
                  20 as libc::c_int |
                  (31 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 5 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      15 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      12 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      9 as libc::c_int |
                  ((31 as libc::c_int as u32_0 &
                        (((0x1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int) as libc::c_uint) <<
                       5 as libc::c_int |
                       (31 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           0 as libc::c_int)) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (31 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 28 as libc::c_int |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            ((31 as libc::c_int as u32_0 &
                  (((0x1 as libc::c_int) << 4 as libc::c_int) -
                       1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                 |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     21 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     18 as libc::c_int |
                 (1 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (6 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh37 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh37;
    (*_g_3).words.w0 =
        (0xfc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((1 as libc::c_int as u32_0 &
                   (((0x1 as libc::c_int) << 4 as libc::c_int) -
                        1 as libc::c_int) as libc::c_uint) <<
                  20 as libc::c_int |
                  (5 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 5 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      15 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      12 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      9 as libc::c_int |
                  ((1 as libc::c_int as u32_0 &
                        (((0x1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int) as libc::c_uint) <<
                       5 as libc::c_int |
                       (5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           0 as libc::c_int)) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        (31 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 28 as libc::c_int |
            (31 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            ((31 as libc::c_int as u32_0 &
                  (((0x1 as libc::c_int) << 4 as libc::c_int) -
                       1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                 |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     21 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     18 as libc::c_int |
                 (31 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (6 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh38 = gfx;
    gfx = gfx.offset(1);
    let mut _g_4: *mut Gfx = fresh38;
    (*_g_4).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 = fbufSave as libc::c_uint;
    let fresh39 = gfx;
    gfx = gfx.offset(1);
    let mut _g_5: *mut Gfx = fresh39;
    (*_g_5).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*this).width as libc::c_float * 4.0f32) as libc::c_int as u32_0
                 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*this).height as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    dx = 0x1000 as libc::c_int / ((*this).width * 2 as libc::c_int);
    x = (*this).height;
    x2 = 0 as libc::c_int;
    while x > 0 as libc::c_int {
        let mut uls: s32 = 0 as libc::c_int;
        let mut lrs: s32 = (*this).width - 1 as libc::c_int;
        let mut ult: s32 = 0;
        let mut lrt: s32 = 0;
        dx = if dx > x { x } else { dx };
        ult = x2;
        lrt = x2 + dx - 1 as libc::c_int;
        let fresh40 = gfx;
        gfx = gfx.offset(1);
        let mut _g_6: *mut Gfx = fresh40;
        (*_g_6).words.w0 =
            (0xfd as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((*this).width - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_6).words.w1 = fbuf as libc::c_uint;
        let fresh41 = gfx;
        gfx = gfx.offset(1);
        let mut _g_7: *mut Gfx = fresh41;
        (*_g_7).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                      7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_7).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh42 = gfx;
        gfx = gfx.offset(1);
        let mut _g_8: *mut Gfx = fresh42;
        (*_g_8).words.w0 =
            (0xe6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh43 = gfx;
        gfx = gfx.offset(1);
        let mut _g_9: *mut Gfx = fresh43;
        (*_g_9).words.w0 =
            (0xf4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_9).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((lrs << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh44 = gfx;
        gfx = gfx.offset(1);
        let mut _g_10: *mut Gfx = fresh44;
        (*_g_10).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_10).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh45 = gfx;
        gfx = gfx.offset(1);
        let mut _g_11: *mut Gfx = fresh45;
        (*_g_11).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                      7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_11).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh46 = gfx;
        gfx = gfx.offset(1);
        let mut _g_12: *mut Gfx = fresh46;
        (*_g_12).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_12).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((lrs << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh47 = gfx;
        gfx = gfx.offset(1);
        let mut _g_13: *mut Gfx = fresh47;
        (*_g_13).words.w0 =
            (0xe4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((lrs + 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((lrt + 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_13).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh48 = gfx;
        gfx = gfx.offset(1);
        let mut _g_14: *mut Gfx = fresh48;
        (*_g_14).words.w0 =
            (0xe1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_14).words.w1 =
            ((uls << 5 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                ((ult << 5 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh49 = gfx;
        gfx = gfx.offset(1);
        let mut _g_15: *mut Gfx = fresh49;
        (*_g_15).words.w0 =
            (0xf1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_15).words.w1 =
            (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        x -= dx;
        x2 += dx
    }
    let fresh50 = gfx;
    gfx = gfx.offset(1);
    let mut _g_16: *mut Gfx = fresh50;
    (*_g_16).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_16).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh51 = gfx;
    gfx = gfx.offset(1);
    let mut _g_17: *mut Gfx = fresh51;
    (*_g_17).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_17).words.w1 = (*this).fbuf as libc::c_uint;
    *gfxp = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C1AE8(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx,
                                       mut fbuf: *mut libc::c_void,
                                       mut fbufSave: *mut libc::c_void) {
    func_800C170C(this, gfxp, fbuf, fbufSave, 255 as libc::c_int as u32_0,
                  255 as libc::c_int as u32_0, 255 as libc::c_int as u32_0,
                  255 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C1B24(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx,
                                       mut fbuf: *mut libc::c_void,
                                       mut cvgSave: *mut libc::c_void) {
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    let mut x: s32 = 0;
    let mut x2: s32 = 0;
    let mut dx: s32 = 0;
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 422 as libc::c_int);
    LogUtils_CheckNullPointer(b"glistpp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfxp as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 423 as libc::c_int);
    gfx = *gfxp;
    LogUtils_CheckNullPointer(b"glistp\x00" as *const u8 as
                                  *const libc::c_char,
                              gfx as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 425 as libc::c_int);
    let fresh52 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh52;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh53 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh53;
    (*_g_0).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((3 as libc::c_int) << 4 as libc::c_int |
                  (3 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (6 as libc::c_int) << 9 as libc::c_int |
                  (0 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (0 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int |
             (0 as libc::c_int) << 30 as libc::c_int |
             (3 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (2 as libc::c_int) << 18 as libc::c_int | 0 as libc::c_int |
             0 as libc::c_int | (0 as libc::c_int) << 28 as libc::c_int |
             (3 as libc::c_int) << 24 as libc::c_int |
             (0 as libc::c_int) << 20 as libc::c_int |
             (2 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh54 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh54;
    (*_g_1).words.w0 =
        (0xfc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((31 as libc::c_int as u32_0 &
                   (((0x1 as libc::c_int) << 4 as libc::c_int) -
                        1 as libc::c_int) as libc::c_uint) <<
                  20 as libc::c_int |
                  (31 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 5 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      15 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      12 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      9 as libc::c_int |
                  ((31 as libc::c_int as u32_0 &
                        (((0x1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int) as libc::c_uint) <<
                       5 as libc::c_int |
                       (31 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           0 as libc::c_int)) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        (31 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 28 as libc::c_int |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            ((31 as libc::c_int as u32_0 &
                  (((0x1 as libc::c_int) << 4 as libc::c_int) -
                       1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                 |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     21 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     18 as libc::c_int |
                 (1 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh55 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh55;
    (*_g_2).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = cvgSave as libc::c_uint;
    let fresh56 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh56;
    (*_g_3).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*this).width as libc::c_float * 4.0f32) as libc::c_int as u32_0
                 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*this).height as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    dx = 0x1000 as libc::c_int / ((*this).width * 2 as libc::c_int);
    x = (*this).height;
    x2 = 0 as libc::c_int;
    while x > 0 as libc::c_int {
        let mut uls: s32 = 0 as libc::c_int;
        let mut lrs: s32 = (*this).width - 1 as libc::c_int;
        let mut ult: s32 = 0;
        let mut lrt: s32 = 0;
        dx = if dx > x { x } else { dx };
        ult = x2;
        lrt = x2 + dx - 1 as libc::c_int;
        let fresh57 = gfx;
        gfx = gfx.offset(1);
        let mut _g_4: *mut Gfx = fresh57;
        (*_g_4).words.w0 =
            (0xfd as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((*this).width - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_4).words.w1 = fbuf as libc::c_uint;
        let fresh58 = gfx;
        gfx = gfx.offset(1);
        let mut _g_5: *mut Gfx = fresh58;
        (*_g_5).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                      7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh59 = gfx;
        gfx = gfx.offset(1);
        let mut _g_6: *mut Gfx = fresh59;
        (*_g_6).words.w0 =
            (0xe6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_6).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh60 = gfx;
        gfx = gfx.offset(1);
        let mut _g_7: *mut Gfx = fresh60;
        (*_g_7).words.w0 =
            (0xf4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_7).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((lrs << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh61 = gfx;
        gfx = gfx.offset(1);
        let mut _g_8: *mut Gfx = fresh61;
        (*_g_8).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh62 = gfx;
        gfx = gfx.offset(1);
        let mut _g_9: *mut Gfx = fresh62;
        (*_g_9).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                      7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_9).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh63 = gfx;
        gfx = gfx.offset(1);
        let mut _g_10: *mut Gfx = fresh63;
        (*_g_10).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_10).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((lrs << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((lrt << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh64 = gfx;
        gfx = gfx.offset(1);
        let mut _g_11: *mut Gfx = fresh64;
        (*_g_11).words.w0 =
            (0xe4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((lrs + 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((lrt + 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_11).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((uls << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((ult << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh65 = gfx;
        gfx = gfx.offset(1);
        let mut _g_12: *mut Gfx = fresh65;
        (*_g_12).words.w0 =
            (0xe1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_12).words.w1 =
            ((uls << 5 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                ((ult << 5 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh66 = gfx;
        gfx = gfx.offset(1);
        let mut _g_13: *mut Gfx = fresh66;
        (*_g_13).words.w0 =
            (0xf1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_13).words.w1 =
            (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        x -= dx;
        x2 += dx
    }
    let fresh67 = gfx;
    gfx = gfx.offset(1);
    let mut _g_14: *mut Gfx = fresh67;
    (*_g_14).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_14).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh68 = gfx;
    gfx = gfx.offset(1);
    let mut _g_15: *mut Gfx = fresh68;
    (*_g_15).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((*this).width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_15).words.w1 = (*this).fbuf as libc::c_uint;
    *gfxp = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C1E9C(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    LogUtils_CheckNullPointer(b"this->zbuf_save\x00" as *const u8 as
                                  *const libc::c_char,
                              (*this).zbufSave as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 481 as libc::c_int);
    LogUtils_CheckNullPointer(b"this->zbuf\x00" as *const u8 as
                                  *const libc::c_char,
                              (*this).zbuf as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 482 as libc::c_int);
    if !(*this).zbufSave.is_null() && !(*this).zbuf.is_null() {
        func_800C0F28(this, gfxp, (*this).zbuf as *mut libc::c_void,
                      (*this).zbufSave as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800C1F20(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    LogUtils_CheckNullPointer(b"this->fbuf_save\x00" as *const u8 as
                                  *const libc::c_char,
                              (*this).fbufSave as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 495 as libc::c_int);
    LogUtils_CheckNullPointer(b"this->fbuf\x00" as *const u8 as
                                  *const libc::c_char,
                              (*this).fbuf as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 496 as libc::c_int);
    if !(*this).fbufSave.is_null() && !(*this).fbuf.is_null() {
        func_800C1AE8(this, gfxp, (*this).fbuf as *mut libc::c_void,
                      (*this).fbufSave as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800C1FA4(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = *gfxp;
    let fresh69 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh69;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh70 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh70;
    (*_g_0).words.w0 =
        (0xf9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh71 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh71;
    (*_g_1).words.w0 =
        (0xee as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 =
        (-(1 as libc::c_int) as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (-(1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh72 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh72;
    (*_g_2).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((3 as libc::c_int) << 4 as libc::c_int |
                  (3 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (6 as libc::c_int) << 9 as libc::c_int |
                  (0 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (0 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int | 0x40 as libc::c_int |
             0x4000 as libc::c_int | (0 as libc::c_int) << 30 as libc::c_int |
             (3 as libc::c_int) << 26 as libc::c_int |
             (2 as libc::c_int) << 22 as libc::c_int |
             (1 as libc::c_int) << 18 as libc::c_int | 0x40 as libc::c_int |
             0x4000 as libc::c_int | (0 as libc::c_int) << 28 as libc::c_int |
             (3 as libc::c_int) << 24 as libc::c_int |
             (2 as libc::c_int) << 20 as libc::c_int |
             (1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh73 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh73;
    (*_g_3).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*this).width as libc::c_float * 4.0f32) as libc::c_int as u32_0
                 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*this).height as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh74 = gfx;
    gfx = gfx.offset(1);
    let mut _g_4: *mut Gfx = fresh74;
    (*_g_4).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*this).width as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            ((*this).height as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_4).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    let fresh75 = gfx;
    gfx = gfx.offset(1);
    let mut _g_5: *mut Gfx = fresh75;
    (*_g_5).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
    *gfxp = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C20B4(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    func_800C1FA4(this, gfxp);
    LogUtils_CheckNullPointer(b"this->cvg_save\x00" as *const u8 as
                                  *const libc::c_char,
                              (*this).cvgSave as *mut libc::c_void,
                              b"../PreRender.c\x00" as *const u8 as
                                  *const libc::c_char, 532 as libc::c_int);
    if !(*this).cvgSave.is_null() {
        func_800C1B24(this, gfxp, (*this).fbuf as *mut libc::c_void,
                      (*this).cvgSave as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800C2118(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    func_800C0F28(this, gfxp, (*this).zbufSave as *mut libc::c_void,
                  (*this).zbuf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C213C(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    let mut y: s32 = 0;
    let mut y2: s32 = 0;
    let mut dy: s32 = 0;
    let mut rtile: s32 = 1 as libc::c_int;
    if !(*this).cvgSave.is_null() {
        LogUtils_CheckNullPointer(b"this\x00" as *const u8 as
                                      *const libc::c_char,
                                  this as *mut libc::c_void,
                                  b"../PreRender.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  563 as libc::c_int);
        LogUtils_CheckNullPointer(b"glistpp\x00" as *const u8 as
                                      *const libc::c_char,
                                  gfxp as *mut libc::c_void,
                                  b"../PreRender.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  564 as libc::c_int);
        gfx = *gfxp;
        LogUtils_CheckNullPointer(b"glistp\x00" as *const u8 as
                                      *const libc::c_char,
                                  gfx as *mut libc::c_void,
                                  b"../PreRender.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  566 as libc::c_int);
        let fresh76 = gfx;
        gfx = gfx.offset(1);
        let mut _g: *mut Gfx = fresh76;
        (*_g).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh77 = gfx;
        gfx = gfx.offset(1);
        let mut _g_0: *mut Gfx = fresh77;
        (*_g_0).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_0).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (32 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh78 = gfx;
        gfx = gfx.offset(1);
        let mut _g_1: *mut Gfx = fresh78;
        (*_g_1).words.w0 =
            (0xef as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((3 as libc::c_int) << 4 as libc::c_int |
                      (3 as libc::c_int) << 6 as libc::c_int |
                      (0 as libc::c_int) << 8 as libc::c_int |
                      (6 as libc::c_int) << 9 as libc::c_int |
                      (0 as libc::c_int) << 12 as libc::c_int |
                      (0 as libc::c_int) << 14 as libc::c_int |
                      (0 as libc::c_int) << 16 as libc::c_int |
                      (0 as libc::c_int) << 17 as libc::c_int |
                      (0 as libc::c_int) << 19 as libc::c_int |
                      (1 as libc::c_int) << 20 as libc::c_int |
                      (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 =
            ((0 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int | 0x8 as libc::c_int |
                 0 as libc::c_int | 0 as libc::c_int | 0x1000 as libc::c_int |
                 (0 as libc::c_int) << 30 as libc::c_int |
                 (3 as libc::c_int) << 26 as libc::c_int |
                 (0 as libc::c_int) << 22 as libc::c_int |
                 (2 as libc::c_int) << 18 as libc::c_int |
                 (0 as libc::c_int) << 28 as libc::c_int |
                 (3 as libc::c_int) << 24 as libc::c_int |
                 (0 as libc::c_int) << 20 as libc::c_int |
                 (2 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        let fresh79 = gfx;
        gfx = gfx.offset(1);
        let mut _g_2: *mut Gfx = fresh79;
        (*_g_2).words.w0 =
            (0xfc as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((31 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 4 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      20 as libc::c_int |
                      (31 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          15 as libc::c_int |
                      (6 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          12 as libc::c_int |
                      (2 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          9 as libc::c_int |
                      ((31 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           5 as libc::c_int |
                           (31 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int)) &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_2).words.w1 =
            (31 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 28 as libc::c_int
                |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    15 as libc::c_int |
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                ((31 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 4 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     24 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         21 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         18 as libc::c_int |
                     (0 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         6 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         3 as libc::c_int |
                     (0 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         0 as libc::c_int);
        dy = 4 as libc::c_int;
        y = (*this).height;
        y2 = 0 as libc::c_int;
        while y > 0 as libc::c_int {
            let mut uls: s32 = 0 as libc::c_int;
            let mut lrs: s32 = (*this).width - 1 as libc::c_int;
            let mut ult: s32 = 0;
            let mut lrt: s32 = 0;
            dy = if dy > y { y } else { dy };
            ult = y2;
            lrt = y2 + dy - 1 as libc::c_int;
            let fresh80 = gfx;
            gfx = gfx.offset(1);
            let mut _g_3: *mut Gfx = fresh80;
            (*_g_3).words.w0 =
                (0xfd as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (((*this).width - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 = (*this).fbufSave as libc::c_uint;
            let fresh81 = gfx;
            gfx = gfx.offset(1);
            let mut _g_4: *mut Gfx = fresh81;
            (*_g_4).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                          7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh82 = gfx;
            gfx = gfx.offset(1);
            let mut _g_5: *mut Gfx = fresh82;
            (*_g_5).words.w0 =
                (0xe6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh83 = gfx;
            gfx = gfx.offset(1);
            let mut _g_6: *mut Gfx = fresh83;
            (*_g_6).words.w0 =
                (0xf4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((uls << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((ult << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_6).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((lrs << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((lrt << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh84 = gfx;
            gfx = gfx.offset(1);
            let mut _g_7: *mut Gfx = fresh84;
            (*_g_7).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_7).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh85 = gfx;
            gfx = gfx.offset(1);
            let mut _g_8: *mut Gfx = fresh85;
            (*_g_8).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (((lrs - uls + 1 as libc::c_int) * 2 as libc::c_int +
                          7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_8).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh86 = gfx;
            gfx = gfx.offset(1);
            let mut _g_9: *mut Gfx = fresh86;
            (*_g_9).words.w0 =
                (0xf2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((uls << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((ult << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_9).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((lrs << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((lrt << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh87 = gfx;
            gfx = gfx.offset(1);
            let mut _g_10: *mut Gfx = fresh87;
            (*_g_10).words.w0 =
                (0xfd as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (1 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (((*this).width - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_10).words.w1 = (*this).cvgSave as libc::c_uint;
            let fresh88 = gfx;
            gfx = gfx.offset(1);
            let mut _g_11: *mut Gfx = fresh88;
            (*_g_11).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (1 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (((lrs - uls + 1 as libc::c_int) * 1 as libc::c_int +
                          7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0x160 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_11).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh89 = gfx;
            gfx = gfx.offset(1);
            let mut _g_12: *mut Gfx = fresh89;
            (*_g_12).words.w0 =
                (0xe6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_12).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh90 = gfx;
            gfx = gfx.offset(1);
            let mut _g_13: *mut Gfx = fresh90;
            (*_g_13).words.w0 =
                (0xf4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((uls << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((ult << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_13).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((lrs << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((lrt << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh91 = gfx;
            gfx = gfx.offset(1);
            let mut _g_14: *mut Gfx = fresh91;
            (*_g_14).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_14).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh92 = gfx;
            gfx = gfx.offset(1);
            let mut _g_15: *mut Gfx = fresh92;
            (*_g_15).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (1 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (((lrs - uls + 1 as libc::c_int) * 1 as libc::c_int +
                          7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0x160 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_15).words.w1 =
                (rtile as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh93 = gfx;
            gfx = gfx.offset(1);
            let mut _g_16: *mut Gfx = fresh93;
            (*_g_16).words.w0 =
                (0xf2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((uls << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((ult << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_16).words.w1 =
                (rtile as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((lrs << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((lrt << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh94 = gfx;
            gfx = gfx.offset(1);
            let mut _g_17: *mut Gfx = fresh94;
            (*_g_17).words.w0 =
                (0xe4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((lrs + 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((lrt + 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_17).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((uls << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((ult << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh95 = gfx;
            gfx = gfx.offset(1);
            let mut _g_18: *mut Gfx = fresh95;
            (*_g_18).words.w0 =
                (0xe1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_18).words.w1 =
                ((uls << 5 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    ((ult << 5 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh96 = gfx;
            gfx = gfx.offset(1);
            let mut _g_19: *mut Gfx = fresh96;
            (*_g_19).words.w0 =
                (0xf1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_19).words.w1 =
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            y -= dy;
            y2 += dy
        }
        let fresh97 = gfx;
        gfx = gfx.offset(1);
        let mut _g_20: *mut Gfx = fresh97;
        (*_g_20).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_20).words.w1 = 0 as libc::c_int as libc::c_uint;
        *gfxp = gfx
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800C24BC(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    func_800C0F28(this, gfxp, (*this).fbufSave as *mut libc::c_void,
                  (*this).fbuf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C24E0(mut this: *mut PreRender,
                                       mut gfxp: *mut *mut Gfx) {
    func_800C1258(this, gfxp);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C2500(mut this: *mut PreRender, mut x: s32,
                                       mut y: s32) {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut buffA: [s32; 15] = [0; 15];
    let mut buffR: [s32; 15] = [0; 15];
    let mut buffG: [s32; 15] = [0; 15];
    let mut buffB: [s32; 15] = [0; 15];
    let mut x1: s32 = 0;
    let mut y1: s32 = 0;
    let mut pad: s32 = 0;
    let mut pxR: s32 = 0;
    let mut pxG: s32 = 0;
    let mut pxB: s32 = 0;
    let mut pxR2: s32 = 0;
    let mut pxG2: s32 = 0;
    let mut pxB2: s32 = 0;
    let mut pxIn: Color_RGBA16 =
        Color_RGBA16{c2rust_unnamed: C2RustUnnamed{r_g_b_a: [0; 2],},};
    let mut pxOut: Color_RGBA16 =
        Color_RGBA16{c2rust_unnamed: C2RustUnnamed{r_g_b_a: [0; 2],},};
    let mut pxR3: u32_0 = 0;
    let mut pxG3: u32_0 = 0;
    let mut pxB3: u32_0 = 0;
    /*
    Picture this as a 3x5 rectangle where the middle pixel (index 7) correspond to (x, y)
      _ _ _ _ _
    | 0 1 2 3 4 |
    | 5 6 7 8 9 |
    | A B C D E |
      ‾ ‾ ‾ ‾ ‾
    */
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int * 5 as libc::c_int {
        x1 = i % 5 as libc::c_int + x - 2 as libc::c_int;
        y1 = i / 5 as libc::c_int + y - 1 as libc::c_int;
        if x1 < 0 as libc::c_int {
            x1 = 0 as libc::c_int
        } else if x1 > (*this).width - 1 as libc::c_int {
            x1 = (*this).width - 1 as libc::c_int
        }
        if y1 < 0 as libc::c_int {
            y1 = 0 as libc::c_int
        } else if y1 > (*this).height - 1 as libc::c_int {
            y1 = (*this).height - 1 as libc::c_int
        }
        pxIn.rgba =
            *(*this).fbufSave.offset((x1 + y1 * (*this).width) as isize);
        buffR[i as usize] =
            (pxIn.c2rust_unnamed.r() as libc::c_int) << 3 as libc::c_int |
                pxIn.c2rust_unnamed.r() as libc::c_int >> 2 as libc::c_int;
        buffG[i as usize] =
            (pxIn.c2rust_unnamed.g() as libc::c_int) << 3 as libc::c_int |
                pxIn.c2rust_unnamed.g() as libc::c_int >> 2 as libc::c_int;
        buffB[i as usize] =
            (pxIn.c2rust_unnamed.b() as libc::c_int) << 3 as libc::c_int |
                pxIn.c2rust_unnamed.b() as libc::c_int >> 2 as libc::c_int;
        buffA[i as usize] =
            *(*this).cvgSave.offset((x1 + y1 * (*this).width) as isize) as
                libc::c_int >> 5 as libc::c_int;
        i += 1
        // A
    }
    if buffA[7 as libc::c_int as usize] == 7 as libc::c_int {
        osSyncPrintf(b"Error, should not be in here \n\x00" as *const u8 as
                         *const libc::c_char);
        return
    }
    pxR2 = buffR[7 as libc::c_int as usize];
    pxR = pxR2;
    pxG2 = buffG[7 as libc::c_int as usize];
    pxG = pxG2;
    pxB2 = buffB[7 as libc::c_int as usize];
    pxB = pxB2;
    i = 1 as libc::c_int;
    while i < 3 as libc::c_int * 5 as libc::c_int {
        if buffA[i as usize] == 7 as libc::c_int {
            if pxR < buffR[i as usize] {
                j = 1 as libc::c_int;
                while j < 15 as libc::c_int {
                    if i != j && buffR[j as usize] >= buffR[i as usize] &&
                           buffA[j as usize] == 7 as libc::c_int {
                        pxR = buffR[i as usize]
                    }
                    j += 2 as libc::c_int
                }
            }
            if pxG < buffG[i as usize] {
                j = 1 as libc::c_int;
                while j < 15 as libc::c_int {
                    if i != j && buffG[j as usize] >= buffG[i as usize] &&
                           buffA[j as usize] == 7 as libc::c_int {
                        pxG = buffG[i as usize]
                    }
                    j += 2 as libc::c_int
                }
            }
            if pxB < buffB[i as usize] {
                j = 1 as libc::c_int;
                while j < 15 as libc::c_int {
                    if i != j && buffB[j as usize] >= buffB[i as usize] &&
                           buffA[j as usize] == 7 as libc::c_int {
                        pxB = buffB[i as usize]
                    }
                    j += 2 as libc::c_int
                }
            }
            if pxR2 > buffR[i as usize] {
                j = 1 as libc::c_int;
                while j < 15 as libc::c_int {
                    if i != j && buffR[j as usize] <= buffR[i as usize] &&
                           buffA[j as usize] == 7 as libc::c_int {
                        pxR2 = buffR[i as usize]
                    }
                    j += 2 as libc::c_int
                }
            }
            if pxG2 > buffG[i as usize] {
                j = 1 as libc::c_int;
                while j < 15 as libc::c_int {
                    if i != j && buffG[j as usize] <= buffG[i as usize] &&
                           buffA[j as usize] == 7 as libc::c_int {
                        pxG2 = buffG[i as usize]
                    }
                    j += 2 as libc::c_int
                }
            }
            if pxB2 > buffB[i as usize] {
                j = 1 as libc::c_int;
                while j < 15 as libc::c_int {
                    if i != j && buffB[j as usize] <= buffB[i as usize] &&
                           buffA[j as usize] == 7 as libc::c_int {
                        pxB2 = buffB[i as usize]
                    }
                    j += 2 as libc::c_int
                }
            }
        }
        i += 2 as libc::c_int
    }
    pxR3 =
        (buffR[7 as libc::c_int as usize] +
             ((7 as libc::c_int - buffA[7 as libc::c_int as usize]) *
                  (pxR + pxR2 -
                       (buffR[7 as libc::c_int as usize] << 1 as libc::c_int))
                  + 4 as libc::c_int >> 3 as libc::c_int)) as u32_0;
    pxG3 =
        (buffG[7 as libc::c_int as usize] +
             ((7 as libc::c_int - buffA[7 as libc::c_int as usize]) *
                  (pxG + pxG2 -
                       (buffG[7 as libc::c_int as usize] << 1 as libc::c_int))
                  + 4 as libc::c_int >> 3 as libc::c_int)) as u32_0;
    pxB3 =
        (buffB[7 as libc::c_int as usize] +
             ((7 as libc::c_int - buffA[7 as libc::c_int as usize]) *
                  (pxB + pxB2 -
                       (buffB[7 as libc::c_int as usize] << 1 as libc::c_int))
                  + 4 as libc::c_int >> 3 as libc::c_int)) as u32_0;
    pxOut.c2rust_unnamed.set_r((pxR3 >> 3 as libc::c_int) as u16_0);
    pxOut.c2rust_unnamed.set_g((pxG3 >> 3 as libc::c_int) as u16_0);
    pxOut.c2rust_unnamed.set_b((pxB3 >> 3 as libc::c_int) as u16_0);
    pxOut.c2rust_unnamed.set_a(1 as libc::c_int as u16_0);
    *(*this).fbufSave.offset((x + y * (*this).width) as isize) = pxOut.rgba;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C2FE4(mut this: *mut PreRender) {
    let mut x: s32 = 0;
    let mut y: s32 = 0;
    let mut phi_v0: s32 = 0;
    let mut fresh98 =
        ::std::vec::from_elem(0, (*this).width as libc::c_uint as usize);
    let mut buffR: *mut u8_0 = fresh98.as_mut_ptr() as *mut u8_0;
    let mut fresh99 =
        ::std::vec::from_elem(0, (*this).width as libc::c_uint as usize);
    let mut buffG: *mut u8_0 = fresh99.as_mut_ptr() as *mut u8_0;
    let mut fresh100 =
        ::std::vec::from_elem(0, (*this).width as libc::c_uint as usize);
    let mut buffB: *mut u8_0 = fresh100.as_mut_ptr() as *mut u8_0;
    let mut pad: [s32; 3] = [0; 3];
    let mut pxR: s32 = 0;
    let mut pxG: s32 = 0;
    let mut pxB: s32 = 0;
    y = 0 as libc::c_int;
    while y < (*this).height {
        x = 0 as libc::c_int;
        while x < (*this).width {
            let mut pxIn: Color_RGBA16 =
                Color_RGBA16{c2rust_unnamed:
                                 C2RustUnnamed{r_g_b_a: [0; 2],},};
            pxIn.rgba =
                *(*this).fbufSave.offset((x + y * (*this).width) as isize);
            *buffR.offset(x as isize) = pxIn.c2rust_unnamed.r() as u8_0;
            *buffG.offset(x as isize) = pxIn.c2rust_unnamed.g() as u8_0;
            *buffB.offset(x as isize) = pxIn.c2rust_unnamed.b() as u8_0;
            x += 1
        }
        x = 1 as libc::c_int;
        while x < (*this).width - 1 as libc::c_int {
            let mut pxOut: Color_RGBA16 =
                Color_RGBA16{c2rust_unnamed:
                                 C2RustUnnamed{r_g_b_a: [0; 2],},};
            let mut a: s32 =
                *(*this).cvgSave.offset((x + y * (*this).width) as isize) as
                    s32;
            a >>= 5 as libc::c_int;
            if !(a == 7 as libc::c_int) {
                if (if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              80 as libc::c_int) as usize] as
                           libc::c_int == 0xf as libc::c_int {
                        (*gGameInfo).data[(21 as libc::c_int *
                                               6 as libc::c_int *
                                               16 as libc::c_int +
                                               81 as libc::c_int) as usize] as
                            libc::c_int
                    } else { 0 as libc::c_int }) != 0 as libc::c_int {
                    ((if (*gGameInfo).data[(21 as libc::c_int *
                                                6 as libc::c_int *
                                                16 as libc::c_int +
                                                80 as libc::c_int) as usize]
                             as libc::c_int == 0xf as libc::c_int {
                          (*gGameInfo).data[(21 as libc::c_int *
                                                 6 as libc::c_int *
                                                 16 as libc::c_int +
                                                 81 as libc::c_int) as usize]
                              as libc::c_int
                      } else { 0 as libc::c_int })) != 0 as libc::c_int;
                    if (if (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  80 as libc::c_int) as usize]
                               as libc::c_int == 0xf as libc::c_int {
                            (*gGameInfo).data[(21 as libc::c_int *
                                                   6 as libc::c_int *
                                                   16 as libc::c_int +
                                                   81 as libc::c_int) as
                                                  usize] as libc::c_int
                        } else { 0 as libc::c_int }) == 5 as libc::c_int {
                        pxR = 31 as libc::c_int;
                        pxG = 0 as libc::c_int;
                        pxB = 0 as libc::c_int
                    } else {
                        let mut temp_s0: *mut u8_0 =
                            &mut *buffR.offset((x - 1 as libc::c_int) as
                                                   isize) as *mut u8_0;
                        let mut temp_s1: *mut u8_0 =
                            &mut *buffG.offset((x - 1 as libc::c_int) as
                                                   isize) as *mut u8_0;
                        let mut temp_s2: *mut u8_0 =
                            &mut *buffB.offset((x - 1 as libc::c_int) as
                                                   isize) as *mut u8_0;
                        if (if (*gGameInfo).data[(21 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      80 as libc::c_int) as
                                                     usize] as libc::c_int ==
                                   0xf as libc::c_int {
                                (*gGameInfo).data[(21 as libc::c_int *
                                                       6 as libc::c_int *
                                                       16 as libc::c_int +
                                                       81 as libc::c_int) as
                                                      usize] as libc::c_int
                            } else { 0 as libc::c_int }) == 3 as libc::c_int {
                            osSyncPrintf(b"red=%3d %3d %3d %3d grn=%3d %3d %3d %3d blu=%3d %3d %3d %3d \n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         *temp_s0.offset(0 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         *temp_s0.offset(1 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         *temp_s0.offset(2 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         if *temp_s0.offset(1 as libc::c_int
                                                                as isize) as
                                                libc::c_int >=
                                                *temp_s0.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                                    as libc::c_int {
                                             if *temp_s0.offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                                    as libc::c_int >=
                                                    *temp_s0.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                                        as libc::c_int {
                                                 *temp_s0.offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             } else if *temp_s0.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int >=
                                                           *temp_s0.offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                                               as libc::c_int
                                              {
                                                 *temp_s0.offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             } else {
                                                 *temp_s0.offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             }
                                         } else if *temp_s0.offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                       as libc::c_int >=
                                                       *temp_s0.offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int {
                                             *temp_s0.offset(1 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         } else if *temp_s0.offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                       as libc::c_int >=
                                                       *temp_s0.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int {
                                             *temp_s0.offset(0 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         } else {
                                             *temp_s0.offset(2 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         },
                                         *temp_s1.offset(0 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         *temp_s1.offset(1 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         *temp_s1.offset(2 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         if *temp_s1.offset(1 as libc::c_int
                                                                as isize) as
                                                libc::c_int >=
                                                *temp_s1.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                                    as libc::c_int {
                                             if *temp_s1.offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                                    as libc::c_int >=
                                                    *temp_s1.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                                        as libc::c_int {
                                                 *temp_s1.offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             } else if *temp_s1.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int >=
                                                           *temp_s1.offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                                               as libc::c_int
                                              {
                                                 *temp_s1.offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             } else {
                                                 *temp_s1.offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             }
                                         } else if *temp_s1.offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                       as libc::c_int >=
                                                       *temp_s1.offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int {
                                             *temp_s1.offset(1 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         } else if *temp_s1.offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                       as libc::c_int >=
                                                       *temp_s1.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int {
                                             *temp_s1.offset(0 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         } else {
                                             *temp_s1.offset(2 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         },
                                         *temp_s2.offset(0 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         *temp_s2.offset(1 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         *temp_s2.offset(2 as libc::c_int as
                                                             isize) as
                                             libc::c_int,
                                         if *temp_s2.offset(1 as libc::c_int
                                                                as isize) as
                                                libc::c_int >=
                                                *temp_s2.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                                    as libc::c_int {
                                             if *temp_s2.offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                                    as libc::c_int >=
                                                    *temp_s2.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                                        as libc::c_int {
                                                 *temp_s2.offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             } else if *temp_s2.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int >=
                                                           *temp_s2.offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                                               as libc::c_int
                                              {
                                                 *temp_s2.offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             } else {
                                                 *temp_s2.offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                                     as libc::c_int
                                             }
                                         } else if *temp_s2.offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                       as libc::c_int >=
                                                       *temp_s2.offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int {
                                             *temp_s2.offset(1 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         } else if *temp_s2.offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                       as libc::c_int >=
                                                       *temp_s2.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                           as libc::c_int {
                                             *temp_s2.offset(0 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         } else {
                                             *temp_s2.offset(2 as libc::c_int
                                                                 as isize) as
                                                 libc::c_int
                                         });
                        }
                        if (if (*gGameInfo).data[(21 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      80 as libc::c_int) as
                                                     usize] as libc::c_int ==
                                   0xf as libc::c_int {
                                (*gGameInfo).data[(21 as libc::c_int *
                                                       6 as libc::c_int *
                                                       16 as libc::c_int +
                                                       81 as libc::c_int) as
                                                      usize] as libc::c_int
                            } else { 0 as libc::c_int }) == 1 as libc::c_int {
                            pxR =
                                if *temp_s0.offset(1 as libc::c_int as isize)
                                       as libc::c_int >=
                                       *temp_s0.offset(0 as libc::c_int as
                                                           isize) as
                                           libc::c_int {
                                    if *temp_s0.offset(2 as libc::c_int as
                                                           isize) as
                                           libc::c_int >=
                                           *temp_s0.offset(1 as libc::c_int as
                                                               isize) as
                                               libc::c_int {
                                        *temp_s0.offset(1 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else if *temp_s0.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int >=
                                                  *temp_s0.offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                      as libc::c_int {
                                        *temp_s0.offset(0 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else {
                                        *temp_s0.offset(2 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    }
                                } else if *temp_s0.offset(1 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s0.offset(2 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s0.offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                } else if *temp_s0.offset(2 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s0.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s0.offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                } else {
                                    *temp_s0.offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                };
                            pxG =
                                if *temp_s1.offset(1 as libc::c_int as isize)
                                       as libc::c_int >=
                                       *temp_s1.offset(0 as libc::c_int as
                                                           isize) as
                                           libc::c_int {
                                    if *temp_s1.offset(2 as libc::c_int as
                                                           isize) as
                                           libc::c_int >=
                                           *temp_s1.offset(1 as libc::c_int as
                                                               isize) as
                                               libc::c_int {
                                        *temp_s1.offset(1 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else if *temp_s1.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int >=
                                                  *temp_s1.offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                      as libc::c_int {
                                        *temp_s1.offset(0 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else {
                                        *temp_s1.offset(2 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    }
                                } else if *temp_s1.offset(1 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s1.offset(2 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s1.offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                } else if *temp_s1.offset(2 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s1.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s1.offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                } else {
                                    *temp_s1.offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                };
                            pxB =
                                if *temp_s2.offset(1 as libc::c_int as isize)
                                       as libc::c_int >=
                                       *temp_s2.offset(0 as libc::c_int as
                                                           isize) as
                                           libc::c_int {
                                    if *temp_s2.offset(2 as libc::c_int as
                                                           isize) as
                                           libc::c_int >=
                                           *temp_s2.offset(1 as libc::c_int as
                                                               isize) as
                                               libc::c_int {
                                        *temp_s2.offset(1 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else if *temp_s2.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int >=
                                                  *temp_s2.offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                      as libc::c_int {
                                        *temp_s2.offset(0 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else {
                                        *temp_s2.offset(2 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    }
                                } else if *temp_s2.offset(1 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s2.offset(2 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s2.offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                } else if *temp_s2.offset(2 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s2.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s2.offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                } else {
                                    *temp_s2.offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                }
                        } else {
                            pxR =
                                if *temp_s0.offset(1 as libc::c_int as isize)
                                       as libc::c_int >=
                                       *temp_s0.offset(0 as libc::c_int as
                                                           isize) as
                                           libc::c_int {
                                    if *temp_s0.offset(2 as libc::c_int as
                                                           isize) as
                                           libc::c_int >=
                                           *temp_s0.offset(1 as libc::c_int as
                                                               isize) as
                                               libc::c_int {
                                        *temp_s0.offset(1 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else if *temp_s0.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int >=
                                                  *temp_s0.offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                      as libc::c_int {
                                        *temp_s0.offset(0 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else {
                                        *temp_s0.offset(2 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    }
                                } else if *temp_s0.offset(1 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s0.offset(2 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s0.offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                } else if *temp_s0.offset(2 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s0.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s0.offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                } else {
                                    *temp_s0.offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                };
                            pxG =
                                if *temp_s1.offset(1 as libc::c_int as isize)
                                       as libc::c_int >=
                                       *temp_s1.offset(0 as libc::c_int as
                                                           isize) as
                                           libc::c_int {
                                    if *temp_s1.offset(2 as libc::c_int as
                                                           isize) as
                                           libc::c_int >=
                                           *temp_s1.offset(1 as libc::c_int as
                                                               isize) as
                                               libc::c_int {
                                        *temp_s1.offset(1 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else if *temp_s1.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int >=
                                                  *temp_s1.offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                      as libc::c_int {
                                        *temp_s1.offset(0 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else {
                                        *temp_s1.offset(2 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    }
                                } else if *temp_s1.offset(1 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s1.offset(2 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s1.offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                } else if *temp_s1.offset(2 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s1.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s1.offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                } else {
                                    *temp_s1.offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                };
                            pxB =
                                if *temp_s2.offset(1 as libc::c_int as isize)
                                       as libc::c_int >=
                                       *temp_s2.offset(0 as libc::c_int as
                                                           isize) as
                                           libc::c_int {
                                    if *temp_s2.offset(2 as libc::c_int as
                                                           isize) as
                                           libc::c_int >=
                                           *temp_s2.offset(1 as libc::c_int as
                                                               isize) as
                                               libc::c_int {
                                        *temp_s2.offset(1 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else if *temp_s2.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int >=
                                                  *temp_s2.offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                      as libc::c_int {
                                        *temp_s2.offset(0 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    } else {
                                        *temp_s2.offset(2 as libc::c_int as
                                                            isize) as
                                            libc::c_int
                                    }
                                } else if *temp_s2.offset(1 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s2.offset(2 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s2.offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                } else if *temp_s2.offset(2 as libc::c_int as
                                                              isize) as
                                              libc::c_int >=
                                              *temp_s2.offset(0 as libc::c_int
                                                                  as isize) as
                                                  libc::c_int {
                                    *temp_s2.offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                } else {
                                    *temp_s2.offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                }
                        }
                    }
                    pxOut.c2rust_unnamed.set_r(pxR as u16_0);
                    pxOut.c2rust_unnamed.set_g(pxG as u16_0);
                    pxOut.c2rust_unnamed.set_b(pxB as u16_0);
                    pxOut.c2rust_unnamed.set_a(1 as libc::c_int as u16_0)
                }
                *(*this).fbufSave.offset((x + y * (*this).width) as isize) =
                    pxOut.rgba
            }
            x += 1
        }
        y += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn PreRender_Calc(mut this: *mut PreRender) {
    let mut x: s32 = 0;
    let mut y: s32 = 0;
    if !(*this).cvgSave.is_null() && !(*this).fbufSave.is_null() {
        y = 0 as libc::c_int;
        while y < (*this).height {
            x = 0 as libc::c_int;
            while x < (*this).width {
                let mut a: s32 =
                    *(*this).cvgSave.offset((x + y * (*this).width) as isize)
                        as s32;
                a >>= 5 as libc::c_int;
                a += 1;
                if a != 8 as libc::c_int { func_800C2500(this, x, y); }
                x += 1
            }
            y += 1
        }
        if if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 80 as libc::c_int) as
                                    usize] as libc::c_int ==
                  0xf as libc::c_int {
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 81 as libc::c_int)
                                     as usize] as libc::c_int
           } else { 0 as libc::c_int } != 0 {
            func_800C2FE4(this);
        }
    };
}
