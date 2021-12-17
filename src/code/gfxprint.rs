#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn PrintUtils_VPrintf(pfn: *mut PrintCallback, fmt: *const libc::c_char,
                          args: __builtin_va_list) -> s32;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
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
pub union Color_RGBA8_u32 {
    pub c2rust_unnamed: C2RustUnnamed,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GfxPrint {
    pub callback: PrintCallback,
    pub dList: *mut Gfx,
    pub posX: u16_0,
    pub posY: u16_0,
    pub baseX: u16_0,
    pub baseY: u8_0,
    pub flags: u8_0,
    pub color: Color_RGBA8_u32,
    pub unk_14: [libc::c_char; 28],
}
#[no_mangle]
pub static mut sGfxPrintFontTLUT: [u16_0; 64] =
    [0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0,
     0xffff as libc::c_int as u16_0, 0xffff as libc::c_int as u16_0];
#[no_mangle]
pub static mut sGfxPrintRainbowTLUT: [u16_0; 16] =
    [0xf801 as libc::c_int as u16_0, 0xfbc1 as libc::c_int as u16_0,
     0xffc1 as libc::c_int as u16_0, 0x7c1 as libc::c_int as u16_0,
     0x421 as libc::c_int as u16_0, 0x3f as libc::c_int as u16_0,
     0x803f as libc::c_int as u16_0, 0xf83f as libc::c_int as u16_0,
     0xf801 as libc::c_int as u16_0, 0xfbc1 as libc::c_int as u16_0,
     0xffc1 as libc::c_int as u16_0, 0x7c1 as libc::c_int as u16_0,
     0x421 as libc::c_int as u16_0, 0x3f as libc::c_int as u16_0,
     0x803f as libc::c_int as u16_0, 0xf83f as libc::c_int as u16_0];
#[no_mangle]
pub static mut sGfxPrintRainbowData: [u8_0; 8] =
    [0 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0x22 as libc::c_int as u8_0, 0x33 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0x55 as libc::c_int as u8_0,
     0x66 as libc::c_int as u8_0, 0x77 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sGfxPrintFontData: [u8_0; 2048] =
    [0 as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xfd as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0xee as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xf2 as libc::c_int as u8_0,
     0x2d as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0x1d as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x12 as libc::c_int as u8_0,
     0x2d as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x71 as libc::c_int as u8_0,
     0x99 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1e as libc::c_int as u8_0,
     0xed as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x7e as libc::c_int as u8_0,
     0xf7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x56 as libc::c_int as u8_0,
     0x29 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x58 as libc::c_int as u8_0,
     0x97 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xd2 as libc::c_int as u8_0,
     0x29 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x59 as libc::c_int as u8_0,
     0x97 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xfd as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x6e as libc::c_int as u8_0,
     0xf7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0xbf as libc::c_int as u8_0,
     0xfb as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0xff as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xb0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xfb as libc::c_int as u8_0,
     0xbf as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xfc as libc::c_int as u8_0,
     0xce as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xd4 as libc::c_int as u8_0,
     0x4f as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xfb as libc::c_int as u8_0,
     0xbf as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0xf7 as libc::c_int as u8_0,
     0x77 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xfd as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0xee as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xf2 as libc::c_int as u8_0,
     0x2d as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0x1d as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x12 as libc::c_int as u8_0,
     0x2d as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x71 as libc::c_int as u8_0,
     0x99 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1e as libc::c_int as u8_0,
     0xed as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x7e as libc::c_int as u8_0,
     0xf7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x56 as libc::c_int as u8_0,
     0x29 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x58 as libc::c_int as u8_0,
     0x97 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xd2 as libc::c_int as u8_0,
     0x29 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x59 as libc::c_int as u8_0,
     0x97 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xfd as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x6e as libc::c_int as u8_0,
     0xf7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0xbf as libc::c_int as u8_0,
     0xfb as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xd as libc::c_int as u8_0,
     0xe0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xb0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x5d as libc::c_int as u8_0,
     0xe6 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x5c as libc::c_int as u8_0,
     0xc6 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xfb as libc::c_int as u8_0,
     0xbf as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x77 as libc::c_int as u8_0, 0x3f as libc::c_int as u8_0,
     0xf3 as libc::c_int as u8_0, 0x77 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xd4 as libc::c_int as u8_0,
     0x4f as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xbb as libc::c_int as u8_0, 0x3f as libc::c_int as u8_0,
     0xf3 as libc::c_int as u8_0, 0xbb as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x9c as libc::c_int as u8_0,
     0xca as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xfb as libc::c_int as u8_0,
     0xbf as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x9d as libc::c_int as u8_0,
     0xea as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xd as libc::c_int as u8_0,
     0xe0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xc2 as libc::c_int as u8_0,
     0x2c as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x8d as libc::c_int as u8_0,
     0x50 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xca as libc::c_int as u8_0,
     0xac as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0xf9 as libc::c_int as u8_0,
     0x17 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xc2 as libc::c_int as u8_0,
     0x2c as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x12 as libc::c_int as u8_0, 0x49 as libc::c_int as u8_0,
     0x34 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x82 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x97 as libc::c_int as u8_0,
     0x51 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x8a as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0x52 as libc::c_int as u8_0, 0x41 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x43 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0x75 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xa2 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x60 as libc::c_int as u8_0, 0x5 as libc::c_int as u8_0,
     0x56 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x22 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xf as libc::c_int as u8_0,
     0xb0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xd as libc::c_int as u8_0,
     0xa4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0xcd as libc::c_int as u8_0,
     0xe8 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2a as libc::c_int as u8_0,
     0xa2 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0xcd as libc::c_int as u8_0,
     0xe8 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0xaa as libc::c_int as u8_0,
     0x22 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xd as libc::c_int as u8_0,
     0xa4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xd1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xf as libc::c_int as u8_0,
     0xb0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8c as libc::c_int as u8_0, 0x51 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x22 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x81 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xfd as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0xee as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xf2 as libc::c_int as u8_0,
     0x2d as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0x1d as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x12 as libc::c_int as u8_0,
     0x2d as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x71 as libc::c_int as u8_0,
     0x99 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1e as libc::c_int as u8_0,
     0xed as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x7e as libc::c_int as u8_0,
     0xf7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x56 as libc::c_int as u8_0,
     0x29 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x58 as libc::c_int as u8_0,
     0x97 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xd2 as libc::c_int as u8_0,
     0x29 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x59 as libc::c_int as u8_0,
     0x97 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xfd as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x6e as libc::c_int as u8_0,
     0xf7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x33 as libc::c_int as u8_0,
     0x33 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x48 as libc::c_int as u8_0,
     0x99 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x3c as libc::c_int as u8_0,
     0xc3 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xcd as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x3c as libc::c_int as u8_0,
     0xc3 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0xbf as libc::c_int as u8_0,
     0x62 as libc::c_int as u8_0, 0xa8 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x33 as libc::c_int as u8_0,
     0x33 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x4c as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x15 as libc::c_int as u8_0,
     0xc8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x3c as libc::c_int as u8_0,
     0xc3 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x67 as libc::c_int as u8_0,
     0x32 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x3f as libc::c_int as u8_0,
     0xf3 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x99 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xfd as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0xff as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x1c as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0xc1 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x1e as libc::c_int as u8_0, 0xe1 as libc::c_int as u8_0,
     0x16 as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x1e as libc::c_int as u8_0, 0xf4 as libc::c_int as u8_0,
     0x56 as libc::c_int as u8_0, 0x21 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf6 as libc::c_int as u8_0,
     0x67 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x1e as libc::c_int as u8_0, 0xf2 as libc::c_int as u8_0,
     0x36 as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x89 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x1e as libc::c_int as u8_0, 0xf1 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xe1 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x16 as libc::c_int as u8_0, 0xec as libc::c_int as u8_0,
     0xce as libc::c_int as u8_0, 0x21 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0xfb as libc::c_int as u8_0,
     0xbb as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0xb6 as libc::c_int as u8_0,
     0x6f as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x27 as libc::c_int as u8_0, 0xd8 as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x92 as libc::c_int as u8_0,
     0xed as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x2e as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x9a as libc::c_int as u8_0,
     0xe5 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xf6 as libc::c_int as u8_0,
     0x2e as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x9b as libc::c_int as u8_0,
     0x75 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xd6 as libc::c_int as u8_0,
     0x4e as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xda as libc::c_int as u8_0,
     0xe5 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x4e as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xd2 as libc::c_int as u8_0,
     0xed as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0xf6 as libc::c_int as u8_0,
     0x6f as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x27 as libc::c_int as u8_0, 0xd9 as libc::c_int as u8_0,
     0x9f as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0xff as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8f as libc::c_int as u8_0, 0x71 as libc::c_int as u8_0,
     0x1f as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x8f as libc::c_int as u8_0, 0x71 as libc::c_int as u8_0,
     0x1f as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0x1e as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x27 as libc::c_int as u8_0, 0xdd as libc::c_int as u8_0,
     0xdf as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0x69 as libc::c_int as u8_0,
     0x1e as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x27 as libc::c_int as u8_0, 0x76 as libc::c_int as u8_0,
     0x4a as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0xe9 as libc::c_int as u8_0,
     0x9e as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x2f as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x6e as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x8a as libc::c_int as u8_0, 0xe7 as libc::c_int as u8_0,
     0xfe as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0xfa as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0x27 as libc::c_int as u8_0,
     0x7a as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x7c as libc::c_int as u8_0,
     0xcf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x13 as libc::c_int as u8_0, 0x26 as libc::c_int as u8_0,
     0x60 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x7c as libc::c_int as u8_0,
     0xcf as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x76 as libc::c_int as u8_0,
     0x65 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x39 as libc::c_int as u8_0,
     0xd7 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x53 as libc::c_int as u8_0,
     0x35 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x2f as libc::c_int as u8_0,
     0xf2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x13 as libc::c_int as u8_0,
     0x31 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x5f as libc::c_int as u8_0,
     0xb1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x3 as libc::c_int as u8_0,
     0x30 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x5e as libc::c_int as u8_0,
     0xe5 as libc::c_int as u8_0, 0x50 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x13 as libc::c_int as u8_0,
     0x31 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x5e as libc::c_int as u8_0,
     0xed as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x23 as libc::c_int as u8_0,
     0x30 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x8 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x8a as libc::c_int as u8_0, 0xab as libc::c_int as u8_0,
     0xb8 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x45 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x62 as libc::c_int as u8_0,
     0x33 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x44 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xc8 as libc::c_int as u8_0,
     0x9a as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xee as libc::c_int as u8_0,
     0xab as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xe6 as libc::c_int as u8_0,
     0x67 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0xf5 as libc::c_int as u8_0,
     0x5f as libc::c_int as u8_0, 0xb0 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0xf6 as libc::c_int as u8_0,
     0x2b as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0xfc as libc::c_int as u8_0,
     0x89 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xee as libc::c_int as u8_0,
     0xee as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x77 as libc::c_int as u8_0,
     0x3b as libc::c_int as u8_0, 0xb0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x92 as libc::c_int as u8_0,
     0x24 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x26 as libc::c_int as u8_0, 0xef as libc::c_int as u8_0,
     0xde as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x9b as libc::c_int as u8_0,
     0xb5 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2e as libc::c_int as u8_0, 0xc3 as libc::c_int as u8_0,
     0x3c as libc::c_int as u8_0, 0xe2 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0x9a as libc::c_int as u8_0,
     0x25 as libc::c_int as u8_0, 0x50 as libc::c_int as u8_0,
     0x2e as libc::c_int as u8_0, 0xc3 as libc::c_int as u8_0,
     0x3c as libc::c_int as u8_0, 0xe2 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xda as libc::c_int as u8_0,
     0xa5 as libc::c_int as u8_0, 0x50 as libc::c_int as u8_0,
     0x2e as libc::c_int as u8_0, 0xc3 as libc::c_int as u8_0,
     0x3c as libc::c_int as u8_0, 0xe2 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0xd6 as libc::c_int as u8_0,
     0xed as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x26 as libc::c_int as u8_0, 0xcb as libc::c_int as u8_0,
     0xbc as libc::c_int as u8_0, 0x62 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0xfb as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0x16 as libc::c_int as u8_0, 0xe8 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xf4 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x8f as libc::c_int as u8_0, 0x71 as libc::c_int as u8_0,
     0x17 as libc::c_int as u8_0, 0xf8 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0xfc as libc::c_int as u8_0,
     0x8b as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0x69 as libc::c_int as u8_0,
     0x96 as libc::c_int as u8_0, 0xe8 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x73 as libc::c_int as u8_0,
     0x3b as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x8a as libc::c_int as u8_0, 0x6d as libc::c_int as u8_0,
     0xd6 as libc::c_int as u8_0, 0xa8 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xd8 as libc::c_int as u8_0,
     0x8a as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0xa7 as libc::c_int as u8_0,
     0x79 as libc::c_int as u8_0, 0xb2 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x8a as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0xa1 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x74 as libc::c_int as u8_0,
     0x4f as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0xa9 as libc::c_int as u8_0,
     0x90 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x31 as libc::c_int as u8_0,
     0xdf as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x84 as libc::c_int as u8_0, 0xe6 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x27 as libc::c_int as u8_0,
     0xda as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xc8 as libc::c_int as u8_0, 0xaa as libc::c_int as u8_0,
     0x4c as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x57 as libc::c_int as u8_0,
     0x3b as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xa1 as libc::c_int as u8_0,
     0x18 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0x54 as libc::c_int as u8_0,
     0x6f as libc::c_int as u8_0, 0x50 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xa9 as libc::c_int as u8_0,
     0x98 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x22 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x18 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x44 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xc as libc::c_int as u8_0,
     0xc0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x46 as libc::c_int as u8_0,
     0xa4 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xc as libc::c_int as u8_0,
     0xc0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x8e as libc::c_int as u8_0,
     0xe0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x8 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0xa8 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0xcd as libc::c_int as u8_0,
     0x4c as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x6 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x56 as libc::c_int as u8_0, 0xe7 as libc::c_int as u8_0,
     0x50 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x1f as libc::c_int as u8_0,
     0xf1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x38 as libc::c_int as u8_0, 0x8c as libc::c_int as u8_0,
     0xb8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0xf6 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x94 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x7 as libc::c_int as u8_0,
     0x6a as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xcb as libc::c_int as u8_0, 0xa6 as libc::c_int as u8_0,
     0xc8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x47 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xa as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x39 as libc::c_int as u8_0,
     0x14 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x22 as libc::c_int as u8_0,
     0x24 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0xae as libc::c_int as u8_0,
     0xa8 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x28 as libc::c_int as u8_0,
     0x99 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x75 as libc::c_int as u8_0,
     0xd1 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xb3 as libc::c_int as u8_0,
     0x33 as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xae as libc::c_int as u8_0,
     0xbe as libc::c_int as u8_0, 0xa4 as libc::c_int as u8_0,
     0x25 as libc::c_int as u8_0, 0x15 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x61 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0x42 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x2c as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x14 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x28 as libc::c_int as u8_0,
     0x82 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0xac as libc::c_int as u8_0,
     0xc1 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x8 as libc::c_int as u8_0,
     0x12 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0xcf as libc::c_int as u8_0,
     0xee as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0x62 as libc::c_int as u8_0,
     0x2e as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x82 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x44 as libc::c_int as u8_0,
     0xe4 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8d as libc::c_int as u8_0, 0xea as libc::c_int as u8_0,
     0xac as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x24 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x21 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x42 as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xcc as libc::c_int as u8_0,
     0xf4 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0xbf as libc::c_int as u8_0,
     0xd4 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xcc as libc::c_int as u8_0,
     0xc4 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xc as libc::c_int as u8_0,
     0xc0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0xa0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xc as libc::c_int as u8_0,
     0xc0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xce as libc::c_int as u8_0,
     0x64 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x8 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0xa8 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x44 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x29 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x54 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xee as libc::c_int as u8_0, 0xfe as libc::c_int as u8_0,
     0xe0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x3b as libc::c_int as u8_0,
     0x3f as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0xd8 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x54 as libc::c_int as u8_0,
     0x4f as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x18 as libc::c_int as u8_0, 0x58 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x86 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc6 as libc::c_int as u8_0, 0x7e as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xef as libc::c_int as u8_0,
     0x66 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xaa as libc::c_int as u8_0, 0xaa as libc::c_int as u8_0,
     0xea as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0xef as libc::c_int as u8_0, 0xff as libc::c_int as u8_0,
     0xff as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0x44 as libc::c_int as u8_0,
     0x19 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x49 as libc::c_int as u8_0,
     0x24 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc5 as libc::c_int as u8_0, 0x35 as libc::c_int as u8_0,
     0x1b as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4b as libc::c_int as u8_0,
     0x24 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x35 as libc::c_int as u8_0,
     0xa0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8c as libc::c_int as u8_0, 0xa9 as libc::c_int as u8_0,
     0xac as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x2c as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x21 as libc::c_int as u8_0,
     0xa4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2a as libc::c_int as u8_0, 0x84 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x73 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0xf1 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0x19 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8f as libc::c_int as u8_0, 0xee as libc::c_int as u8_0,
     0xef as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0x76 as libc::c_int as u8_0,
     0x66 as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x1a as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x4c as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1a as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0x2c as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x38 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0xa0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x42 as libc::c_int as u8_0,
     0x83 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0xfe as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0xfd as libc::c_int as u8_0,
     0x54 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0x99 as libc::c_int as u8_0,
     0x9b as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0x26 as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x2a as libc::c_int as u8_0, 0xfe as libc::c_int as u8_0,
     0xee as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x8d as libc::c_int as u8_0, 0x8c as libc::c_int as u8_0,
     0xa9 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x32 as libc::c_int as u8_0, 0x33 as libc::c_int as u8_0,
     0xb3 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x19 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xa1 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0xb1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x8 as libc::c_int as u8_0,
     0x34 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1a as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0xf7 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8e as libc::c_int as u8_0, 0xf4 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x14 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1d as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0xdb as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xdd as libc::c_int as u8_0, 0xfd as libc::c_int as u8_0,
     0xdd as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x6 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x90 as libc::c_int as u8_0,
     0x48 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x34 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2c as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x2c as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x48 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x84 as libc::c_int as u8_0,
     0x83 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x59 as libc::c_int as u8_0, 0x3 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x50 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0xc as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x42 as libc::c_int as u8_0, 0xa9 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0x15 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x98 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0xf9 as libc::c_int as u8_0,
     0x98 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0x77 as libc::c_int as u8_0,
     0x75 as libc::c_int as u8_0, 0x50 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0xc as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x19 as libc::c_int as u8_0, 0x98 as libc::c_int as u8_0,
     0xa8 as libc::c_int as u8_0, 0xd0 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0x99 as libc::c_int as u8_0,
     0xca as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x54 as libc::c_int as u8_0,
     0x65 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0x8 as libc::c_int as u8_0,
     0x50 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0x31 as libc::c_int as u8_0, 0x1c as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x28 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x26 as libc::c_int as u8_0, 0x63 as libc::c_int as u8_0,
     0xbb as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x26 as libc::c_int as u8_0, 0xef as libc::c_int as u8_0,
     0xe6 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc8 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0x8a as libc::c_int as u8_0,
     0x89 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0xc3 as libc::c_int as u8_0, 0xf3 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0xc9 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x5 as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0xe as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x52 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2c as libc::c_int as u8_0, 0xdf as libc::c_int as u8_0,
     0xa8 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x33 as libc::c_int as u8_0,
     0x30 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x3 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0xff as libc::c_int as u8_0,
     0xf7 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0x26 as libc::c_int as u8_0,
     0xe4 as libc::c_int as u8_0, 0x72 as libc::c_int as u8_0,
     0xcc as libc::c_int as u8_0, 0x38 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x38 as libc::c_int as u8_0,
     0x99 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0xa as libc::c_int as u8_0,
     0x31 as libc::c_int as u8_0, 0x50 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xb1 as libc::c_int as u8_0,
     0x82 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x28 as libc::c_int as u8_0,
     0x6 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x87 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0x2a as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x5 as libc::c_int as u8_0,
     0xc2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x85 as libc::c_int as u8_0, 0x82 as libc::c_int as u8_0,
     0xc2 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x39 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0x51 as libc::c_int as u8_0,
     0xbf as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x48 as libc::c_int as u8_0, 0x9d as libc::c_int as u8_0,
     0xcc as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0xc9 as libc::c_int as u8_0, 0xe6 as libc::c_int as u8_0,
     0x7f as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x94 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x5b as libc::c_int as u8_0, 0x21 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0xb0 as libc::c_int as u8_0,
     0x48 as libc::c_int as u8_0, 0xae as libc::c_int as u8_0,
     0xcc as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0xe1 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x43 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0xa4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xe1 as libc::c_int as u8_0, 0x24 as libc::c_int as u8_0,
     0x5d as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x78 as libc::c_int as u8_0, 0x8c as libc::c_int as u8_0,
     0xd6 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0xf1 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x94 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0xd0 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x9c as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
     0xb as libc::c_int as u8_0, 0x8c as libc::c_int as u8_0,
     0x53 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc as libc::c_int as u8_0, 0x9d as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x39 as libc::c_int as u8_0,
     0x50 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0xf0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2e as libc::c_int as u8_0, 0xaf as libc::c_int as u8_0,
     0xc6 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x77 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0x41 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x3 as libc::c_int as u8_0, 0x92 as libc::c_int as u8_0,
     0xf8 as libc::c_int as u8_0, 0x12 as libc::c_int as u8_0,
     0xf as libc::c_int as u8_0, 0xbd as libc::c_int as u8_0,
     0x91 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x1b as libc::c_int as u8_0, 0x28 as libc::c_int as u8_0,
     0x60 as libc::c_int as u8_0, 0x92 as libc::c_int as u8_0,
     0x70 as libc::c_int as u8_0, 0xf4 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0xf0 as libc::c_int as u8_0,
     0xa as libc::c_int as u8_0, 0xd4 as libc::c_int as u8_0,
     0x65 as libc::c_int as u8_0, 0x82 as libc::c_int as u8_0,
     0x53 as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0xe0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x68 as libc::c_int as u8_0, 0x60 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x2a as libc::c_int as u8_0,
     0xbe as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x4f as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x3a as libc::c_int as u8_0,
     0xee as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc8 as libc::c_int as u8_0, 0xc0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0x84 as libc::c_int as u8_0,
     0xa5 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xc1 as libc::c_int as u8_0, 0xc2 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x45 as libc::c_int as u8_0, 0xe as libc::c_int as u8_0,
     0x27 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0xd9 as libc::c_int as u8_0, 0xc3 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x7 as libc::c_int as u8_0, 0xf8 as libc::c_int as u8_0,
     0x8d as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0xac as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x25 as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x22 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0x44 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x16 as libc::c_int as u8_0, 0xa0 as libc::c_int as u8_0,
     0x13 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x30 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x1b as libc::c_int as u8_0,
     0xaa as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x23 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0];
// Can be used to set GFXP_FLAG_ENLARGE by default
static mut sDefaultSpecialFlags: u8_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_Setup(mut this: *mut GfxPrint) {
    let mut width: s32 = 16 as libc::c_int;
    let mut height: s32 = 256 as libc::c_int;
    let mut i: s32 = 0;
    let fresh0 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh1 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((3 as libc::c_int) << 4 as libc::c_int |
                  (3 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (6 as libc::c_int) << 9 as libc::c_int |
                  (2 as libc::c_int) << 12 as libc::c_int |
                  (3 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (0 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int | 0x40 as libc::c_int |
             0x200 as libc::c_int | 0x4000 as libc::c_int | 0 as libc::c_int |
             (0 as libc::c_int) << 30 as libc::c_int |
             (0 as libc::c_int) << 26 as libc::c_int |
             (1 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int | 0x40 as libc::c_int |
             0x200 as libc::c_int | 0x4000 as libc::c_int | 0 as libc::c_int |
             (0 as libc::c_int) << 28 as libc::c_int |
             (0 as libc::c_int) << 24 as libc::c_int |
             (1 as libc::c_int) << 20 as libc::c_int |
             (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh2 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
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
            (1 as libc::c_int as u32_0 &
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
                 (1 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh3 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xfd as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = sGfxPrintFontData.as_mut_ptr() as libc::c_uint;
    let fresh4 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_3: *mut Gfx = fresh4;
    (*_g_3).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh5 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_4: *mut Gfx = fresh5;
    (*_g_4).words.w0 =
        (0xe6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh6 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_5: *mut Gfx = fresh6;
    (*_g_5).words.w0 =
        (0xf3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((if ((width * height + 3 as libc::c_int >> 2 as libc::c_int) -
                      1 as libc::c_int) < 2047 as libc::c_int {
                  (width * height + 3 as libc::c_int >> 2 as libc::c_int) -
                      1 as libc::c_int
              } else { 2047 as libc::c_int }) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((((1 as libc::c_int) << 11 as libc::c_int) +
                   (if 1 as libc::c_int > width / 16 as libc::c_int {
                        1 as libc::c_int
                    } else { (width) / 16 as libc::c_int }) -
                   1 as libc::c_int) /
                  (if 1 as libc::c_int > width / 16 as libc::c_int {
                       1 as libc::c_int
                   } else { (width) / 16 as libc::c_int })) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh7 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_6: *mut Gfx = fresh7;
    (*_g_6).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_6).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh8 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_7: *mut Gfx = fresh8;
    (*_g_7).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((width >> 1 as libc::c_int) + 7 as libc::c_int >>
                  3 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh9 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_8: *mut Gfx = fresh9;
    (*_g_8).words.w0 =
        (0xf2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_8).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((width - 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((height - 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh10 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_9: *mut Gfx = fresh10;
    (*_g_9).words.w0 =
        (0xfd as libc::c_int as u32_0 &
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
            ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_9).words.w1 = sGfxPrintFontTLUT.as_mut_ptr() as libc::c_uint;
    let fresh11 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_10: *mut Gfx = fresh11;
    (*_g_10).words.w0 =
        (0xe8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_10).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh12 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_11: *mut Gfx = fresh12;
    (*_g_11).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (256 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_11).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh13 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_12: *mut Gfx = fresh13;
    (*_g_12).words.w0 =
        (0xe6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_12).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh14 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_13: *mut Gfx = fresh14;
    (*_g_13).words.w0 =
        (0xf0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_13).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((64 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int;
    let fresh15 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_14: *mut Gfx = fresh15;
    (*_g_14).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_14).words.w1 = 0 as libc::c_int as libc::c_uint;
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        let fresh16 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_15: *mut Gfx = fresh16;
        (*_g_15).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_15).words.w1 =
            ((i * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (i as u32_0 &
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
        let fresh17 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_16: *mut Gfx = fresh17;
        (*_g_16).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_16).words.w1 =
            ((i * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (60 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (1020 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        i += 1
    }
    let fresh18 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_17: *mut Gfx = fresh18;
    (*_g_17).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_17).words.w1 = (*this).color.rgba;
    let fresh19 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_18: *mut Gfx = fresh19;
    (*_g_18).words.w0 =
        (0xfd as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((2 as libc::c_int >> 1 as libc::c_int) - 1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_18).words.w1 = sGfxPrintRainbowData.as_mut_ptr() as libc::c_uint;
    let fresh20 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_19: *mut Gfx = fresh20;
    (*_g_19).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((1 as libc::c_int - 0 as libc::c_int + 1 as libc::c_int >>
                   1 as libc::c_int) + 7 as libc::c_int >> 3 as libc::c_int)
                 as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_19).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh21 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_20: *mut Gfx = fresh21;
    (*_g_20).words.w0 =
        (0xe6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_20).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh22 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_21: *mut Gfx = fresh22;
    (*_g_21).words.w0 =
        (0xf4 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((0 as libc::c_int) << 2 as libc::c_int - 1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((0 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_21).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((1 as libc::c_int) << 2 as libc::c_int - 1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((7 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh23 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_22: *mut Gfx = fresh23;
    (*_g_22).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_22).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh24 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_23: *mut Gfx = fresh24;
    (*_g_23).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (((1 as libc::c_int - 0 as libc::c_int + 1 as libc::c_int >>
                   1 as libc::c_int) + 7 as libc::c_int >> 3 as libc::c_int)
                 as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_23).words.w1 =
        (1 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh25 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_24: *mut Gfx = fresh25;
    (*_g_24).words.w0 =
        (0xf2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((0 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((0 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_24).words.w1 =
        (1 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((7 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh26 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_25: *mut Gfx = fresh26;
    (*_g_25).words.w0 =
        (0xfd as libc::c_int as u32_0 &
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
            ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_25).words.w1 = sGfxPrintRainbowTLUT.as_mut_ptr() as libc::c_uint;
    let fresh27 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_26: *mut Gfx = fresh27;
    (*_g_26).words.w0 =
        (0xe8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_26).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh28 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_27: *mut Gfx = fresh28;
    (*_g_27).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (320 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_27).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh29 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_28: *mut Gfx = fresh29;
    (*_g_28).words.w0 =
        (0xe6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_28).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh30 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_29: *mut Gfx = fresh30;
    (*_g_29).words.w0 =
        (0xf0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_29).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((16 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int;
    let fresh31 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_30: *mut Gfx = fresh31;
    (*_g_30).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_30).words.w1 = 0 as libc::c_int as libc::c_uint;
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        let fresh32 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_31: *mut Gfx = fresh32;
        (*_g_31).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_31).words.w1 =
            ((i * 2 as libc::c_int + 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (3 as libc::c_int as u32_0 &
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
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh33 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_32: *mut Gfx = fresh33;
        (*_g_32).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_32).words.w1 =
            ((i * 2 as libc::c_int + 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (28 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_SetColor(mut this: *mut GfxPrint,
                                           mut r: u32_0, mut g: u32_0,
                                           mut b: u32_0, mut a: u32_0) {
    (*this).color.c2rust_unnamed.r = r as u8_0;
    (*this).color.c2rust_unnamed.g = g as u8_0;
    (*this).color.c2rust_unnamed.b = b as u8_0;
    (*this).color.c2rust_unnamed.a = a as u8_0;
    let fresh34 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g: *mut Gfx = fresh34;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh35 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g_0: *mut Gfx = fresh35;
    (*_g_0).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 = (*this).color.rgba;
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_SetPosPx(mut this: *mut GfxPrint,
                                           mut x: s32, mut y: s32) {
    (*this).posX =
        ((*this).baseX as libc::c_int + x * 4 as libc::c_int) as u16_0;
    (*this).posY =
        ((*this).baseY as libc::c_int + y * 4 as libc::c_int) as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_SetPos(mut this: *mut GfxPrint, mut x: s32,
                                         mut y: s32) {
    GfxPrint_SetPosPx(this, x * 8 as libc::c_int, y * 8 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_SetBasePosPx(mut this: *mut GfxPrint,
                                               mut x: s32, mut y: s32) {
    (*this).baseX = (x * 4 as libc::c_int) as u16_0;
    (*this).baseY = (y * 4 as libc::c_int) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_PrintCharImpl(mut this: *mut GfxPrint,
                                                mut c: u8_0) {
    let mut tile: u32_0 =
        ((c as libc::c_int & 0xff as libc::c_int) * 2 as libc::c_int) as
            u32_0;
    if (*this).flags as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int
           != 0 {
        (*this).flags =
            ((*this).flags as libc::c_int &
                 !((1 as libc::c_int) << 3 as libc::c_int)) as u8_0;
        let fresh36 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g: *mut Gfx = fresh36;
        (*_g).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
        if (*this).flags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            let fresh37 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_0: *mut Gfx = fresh37;
            (*_g_0).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 14 as libc::c_int -
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 =
                ((2 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
            let fresh38 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_1: *mut Gfx = fresh38;
            (*_g_1).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 20 as libc::c_int -
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 =
                ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint;
            let fresh39 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_2: *mut Gfx = fresh39;
            (*_g_2).words.w0 =
                (0xe2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 3 as libc::c_int -
                          29 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((29 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 =
                (0 as libc::c_int | 0 as libc::c_int |
                     (0 as libc::c_int) << 30 as libc::c_int |
                     (3 as libc::c_int) << 26 as libc::c_int |
                     (0 as libc::c_int) << 22 as libc::c_int |
                     (2 as libc::c_int) << 18 as libc::c_int |
                     (0x40 as libc::c_int | 0x200 as libc::c_int |
                          0x4000 as libc::c_int | 0 as libc::c_int |
                          (0 as libc::c_int) << 28 as libc::c_int |
                          (0 as libc::c_int) << 24 as libc::c_int |
                          (1 as libc::c_int) << 20 as libc::c_int |
                          (0 as libc::c_int) << 16 as libc::c_int)) as
                    libc::c_uint;
            let fresh40 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_3: *mut Gfx = fresh40;
            (*_g_3).words.w0 =
                (0xfc as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((1 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          20 as libc::c_int |
                          (2 as libc::c_int as u32_0 &
                               (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              15 as libc::c_int |
                          (1 as libc::c_int as u32_0 &
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
                                    (((0x1 as libc::c_int) <<
                                          5 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   0 as libc::c_int)) &
                         (((0x1 as libc::c_int) << 24 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 =
                (31 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    28 as libc::c_int |
                    (31 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
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
                             0 as libc::c_int)
        } else {
            let fresh41 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_4: *mut Gfx = fresh41;
            (*_g_4).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 14 as libc::c_int -
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 =
                ((3 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
            let fresh42 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_5: *mut Gfx = fresh42;
            (*_g_5).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 20 as libc::c_int -
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_5).words.w1 =
                ((0 as libc::c_int) << 20 as libc::c_int) as libc::c_uint;
            let fresh43 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_6: *mut Gfx = fresh43;
            (*_g_6).words.w0 =
                (0xe2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 3 as libc::c_int -
                          29 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((29 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_6).words.w1 =
                (0x40 as libc::c_int | 0x200 as libc::c_int |
                     0x4000 as libc::c_int | 0 as libc::c_int |
                     (0 as libc::c_int) << 30 as libc::c_int |
                     (0 as libc::c_int) << 26 as libc::c_int |
                     (1 as libc::c_int) << 22 as libc::c_int |
                     (0 as libc::c_int) << 18 as libc::c_int |
                     (0x40 as libc::c_int | 0x200 as libc::c_int |
                          0x4000 as libc::c_int | 0 as libc::c_int |
                          (0 as libc::c_int) << 28 as libc::c_int |
                          (0 as libc::c_int) << 24 as libc::c_int |
                          (1 as libc::c_int) << 20 as libc::c_int |
                          (0 as libc::c_int) << 16 as libc::c_int)) as
                    libc::c_uint;
            let fresh44 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_7: *mut Gfx = fresh44;
            (*_g_7).words.w0 =
                (0xfc as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((1 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          20 as libc::c_int |
                          (3 as libc::c_int as u32_0 &
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
                               (3 as libc::c_int as u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          5 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   0 as libc::c_int)) &
                         (((0x1 as libc::c_int) << 24 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_7).words.w1 =
                (31 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    28 as libc::c_int |
                    (31 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        15 as libc::c_int |
                    (7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (1 as libc::c_int as u32_0 &
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
                         (31 as libc::c_int as u32_0 &
                              (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             6 as libc::c_int |
                         (7 as libc::c_int as u32_0 &
                              (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             3 as libc::c_int |
                         (1 as libc::c_int as u32_0 &
                              (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int)
        }
    }
    if (*this).flags as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int
           != 0 {
        let fresh45 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_8: *mut Gfx = fresh45;
        (*_g_8).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
        if (*this).flags as libc::c_int &
               (1 as libc::c_int) << 6 as libc::c_int != 0 {
            let fresh46 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_9: *mut Gfx = fresh46;
            (*_g_9).words.w0 =
                (0xe4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((((*this).posX as libc::c_int + 4 as libc::c_int +
                           32 as libc::c_int) << 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((((*this).posY as libc::c_int + 4 as libc::c_int +
                           32 as libc::c_int) << 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_9).words.w1 =
                (tile &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((((*this).posX as libc::c_int + 4 as libc::c_int) <<
                          1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((((*this).posY as libc::c_int + 4 as libc::c_int) <<
                          1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh47 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_10: *mut Gfx = fresh47;
            (*_g_10).words.w0 =
                (0xe1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_10).words.w1 =
                (((c as libc::c_int & 4 as libc::c_int) as u16_0 as
                      libc::c_int * 64 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (((c as libc::c_int >> 3 as libc::c_int) as u16_0 as
                          libc::c_int * 256 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh48 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_11: *mut Gfx = fresh48;
            (*_g_11).words.w0 =
                (0xf1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_11).words.w1 =
                (((1 as libc::c_int) << 9 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (((1 as libc::c_int) << 9 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        } else {
            let fresh49 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_12: *mut Gfx = fresh49;
            (*_g_12).words.w0 =
                (0xe4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((*this).posX as libc::c_int + 4 as libc::c_int +
                          32 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((*this).posY as libc::c_int + 4 as libc::c_int +
                          32 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_12).words.w1 =
                (tile &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((*this).posX as libc::c_int + 4 as libc::c_int) as u32_0
                         &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((*this).posY as libc::c_int + 4 as libc::c_int) as u32_0
                         &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh50 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_13: *mut Gfx = fresh50;
            (*_g_13).words.w0 =
                (0xe1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_13).words.w1 =
                (((c as libc::c_int & 4 as libc::c_int) as u16_0 as
                      libc::c_int * 64 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (((c as libc::c_int >> 3 as libc::c_int) as u16_0 as
                          libc::c_int * 256 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh51 = (*this).dList;
            (*this).dList = (*this).dList.offset(1);
            let mut _g_14: *mut Gfx = fresh51;
            (*_g_14).words.w0 =
                (0xf1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_14).words.w1 =
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
        let fresh52 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_15: *mut Gfx = fresh52;
        (*_g_15).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_15).words.w1 = (*this).color.rgba
    }
    if (*this).flags as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int
           != 0 {
        let fresh53 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_16: *mut Gfx = fresh53;
        (*_g_16).words.w0 =
            (0xe4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((((*this).posX as libc::c_int + 32 as libc::c_int) <<
                      1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((((*this).posY as libc::c_int + 32 as libc::c_int) <<
                      1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_16).words.w1 =
            (tile &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((((*this).posX as libc::c_int) << 1 as libc::c_int) as u32_0
                     &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((((*this).posY as libc::c_int) << 1 as libc::c_int) as u32_0
                     &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh54 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_17: *mut Gfx = fresh54;
        (*_g_17).words.w0 =
            (0xe1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_17).words.w1 =
            (((c as libc::c_int & 4 as libc::c_int) as u16_0 as libc::c_int *
                  64 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((c as libc::c_int >> 3 as libc::c_int) as u16_0 as
                      libc::c_int * 256 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh55 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_18: *mut Gfx = fresh55;
        (*_g_18).words.w0 =
            (0xf1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_18).words.w1 =
            (((1 as libc::c_int) << 9 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((1 as libc::c_int) << 9 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    } else {
        let fresh56 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_19: *mut Gfx = fresh56;
        (*_g_19).words.w0 =
            (0xe4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*this).posX as libc::c_int + 32 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((*this).posY as libc::c_int + 32 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_19).words.w1 =
            (tile &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*this).posX as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((*this).posY as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh57 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_20: *mut Gfx = fresh57;
        (*_g_20).words.w0 =
            (0xe1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_20).words.w1 =
            (((c as libc::c_int & 4 as libc::c_int) as u16_0 as libc::c_int *
                  64 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((c as libc::c_int >> 3 as libc::c_int) as u16_0 as
                      libc::c_int * 256 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh58 = (*this).dList;
        (*this).dList = (*this).dList.offset(1);
        let mut _g_21: *mut Gfx = fresh58;
        (*_g_21).words.w0 =
            (0xf1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_21).words.w1 =
            (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    }
    (*this).posX = ((*this).posX as libc::c_int + 32 as libc::c_int) as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_PrintChar(mut this: *mut GfxPrint,
                                            mut c: u8_0) {
    let mut charParam: u8_0 = c;
    if c as libc::c_int == ' ' as i32 {
        (*this).posX =
            ((*this).posX as libc::c_int + 32 as libc::c_int) as u16_0
    } else if c as libc::c_int > ' ' as i32 &&
                  (c as libc::c_int) < 0x7f as libc::c_int {
        GfxPrint_PrintCharImpl(this, charParam);
    } else if c as libc::c_int >= 0xa0 as libc::c_int &&
                  (c as libc::c_int) < 0xe0 as libc::c_int {
        if (*this).flags as libc::c_int &
               (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if (c as libc::c_int) < 0xc0 as libc::c_int {
                charParam = (c as libc::c_int - 0x20 as libc::c_int) as u8_0
            } else {
                charParam = (c as libc::c_int + 0x20 as libc::c_int) as u8_0
            }
        }
        GfxPrint_PrintCharImpl(this, charParam);
    } else {
        let mut current_block_23: u64;
        match c as libc::c_int {
            10 => {
                (*this).posY =
                    ((*this).posY as libc::c_int + 32 as libc::c_int) as
                        u16_0;
                current_block_23 = 13196124132740782928;
            }
            13 => { current_block_23 = 13196124132740782928; }
            9 => {
                loop  {
                    GfxPrint_PrintCharImpl(this, ' ' as i32 as u8_0);
                    if !(((*this).posX as libc::c_int -
                              (*this).baseX as libc::c_int) %
                             256 as libc::c_int != 0) {
                        break ;
                    }
                }
                current_block_23 = 2569451025026770673;
            }
            141 => {
                (*this).flags =
                    ((*this).flags as libc::c_int |
                         (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
                current_block_23 = 2569451025026770673;
            }
            140 => {
                (*this).flags =
                    ((*this).flags as libc::c_int &
                         !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
                current_block_23 = 2569451025026770673;
            }
            139 => {
                (*this).flags =
                    ((*this).flags as libc::c_int |
                         (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
                (*this).flags =
                    ((*this).flags as libc::c_int |
                         (1 as libc::c_int) << 3 as libc::c_int) as u8_0;
                current_block_23 = 2569451025026770673;
            }
            138 => {
                (*this).flags =
                    ((*this).flags as libc::c_int &
                         !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
                (*this).flags =
                    ((*this).flags as libc::c_int |
                         (1 as libc::c_int) << 3 as libc::c_int) as u8_0;
                current_block_23 = 2569451025026770673;
            }
            0 | 142 | _ => { current_block_23 = 2569451025026770673; }
        }
        match current_block_23 {
            13196124132740782928 => { (*this).posX = (*this).baseX }
            _ => { }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_PrintStringWithSize(mut this: *mut GfxPrint,
                                                      mut buffer:
                                                          *const libc::c_void,
                                                      mut charSize: u32_0,
                                                      mut charCount: u32_0) {
    let mut str: *const libc::c_char = buffer as *const libc::c_char;
    let mut count: u32_0 = charSize.wrapping_mul(charCount);
    while count != 0 as libc::c_int as libc::c_uint {
        let fresh59 = str;
        str = str.offset(1);
        GfxPrint_PrintChar(this, *fresh59 as u8_0);
        count = count.wrapping_sub(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_PrintString(mut this: *mut GfxPrint,
                                              mut str: *const libc::c_char) {
    while *str as libc::c_int != '\u{0}' as i32 {
        let fresh60 = str;
        str = str.offset(1);
        GfxPrint_PrintChar(this, *fresh60 as u8_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_Callback(mut arg: *mut libc::c_void,
                                           mut str: *const libc::c_char,
                                           mut size: u32_0)
 -> *mut libc::c_void {
    let mut this: *mut GfxPrint = arg as *mut GfxPrint;
    GfxPrint_PrintStringWithSize(this, str as *const libc::c_void,
                                 ::std::mem::size_of::<libc::c_char>() as
                                     libc::c_ulong, size);
    return this as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_Init(mut this: *mut GfxPrint) {
    (*this).flags =
        ((*this).flags as libc::c_int &
             !((1 as libc::c_int) << 7 as libc::c_int)) as u8_0;
    (*this).callback =
        Some(GfxPrint_Callback as
                 unsafe extern "C" fn(_: *mut libc::c_void,
                                      _: *const libc::c_char, _: u32_0)
                     -> *mut libc::c_void);
    (*this).dList = 0 as *mut Gfx;
    (*this).posX = 0 as libc::c_int as u16_0;
    (*this).posY = 0 as libc::c_int as u16_0;
    (*this).baseX = 0 as libc::c_int as u16_0;
    (*this).baseY = 0 as libc::c_int as u8_0;
    (*this).color.rgba = 0 as libc::c_int as u32_0;
    (*this).flags =
        ((*this).flags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).flags =
        ((*this).flags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*this).flags =
        ((*this).flags as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int) as u8_0;
    (*this).flags =
        ((*this).flags as libc::c_int |
             (1 as libc::c_int) << 3 as libc::c_int) as u8_0;
    if sDefaultSpecialFlags as libc::c_int &
           (1 as libc::c_int) << 6 as libc::c_int != 0 {
        (*this).flags =
            ((*this).flags as libc::c_int |
                 (1 as libc::c_int) << 6 as libc::c_int) as u8_0
    } else {
        (*this).flags =
            ((*this).flags as libc::c_int &
                 !((1 as libc::c_int) << 6 as libc::c_int)) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_Destroy(mut this: *mut GfxPrint) { }
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_Open(mut this: *mut GfxPrint,
                                       mut dList: *mut Gfx) {
    if (*this).flags as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int
           == 0 {
        (*this).flags =
            ((*this).flags as libc::c_int |
                 (1 as libc::c_int) << 7 as libc::c_int) as u8_0;
        (*this).dList = dList;
        GfxPrint_Setup(this);
    } else {
        osSyncPrintf(b"gfxprint_open:\xef\xbc\x92\xe9\x87\x8d\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x97\xe3\x83\xb3\xe3\x81\xa7\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_Close(mut this: *mut GfxPrint) -> *mut Gfx {
    let mut ret: *mut Gfx = 0 as *mut Gfx;
    (*this).flags =
        ((*this).flags as libc::c_int &
             !((1 as libc::c_int) << 7 as libc::c_int)) as u8_0;
    let fresh61 = (*this).dList;
    (*this).dList = (*this).dList.offset(1);
    let mut _g: *mut Gfx = fresh61;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    ret = (*this).dList;
    (*this).dList = 0 as *mut Gfx;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_VPrintf(mut this: *mut GfxPrint,
                                          mut fmt: *const libc::c_char,
                                          mut args: __builtin_va_list)
 -> s32 {
    return PrintUtils_VPrintf(&mut (*this).callback, fmt, args);
}
#[no_mangle]
pub unsafe extern "C" fn GfxPrint_Printf(mut this: *mut GfxPrint,
                                         mut fmt: *const libc::c_char,
                                         mut args: ...) -> s32 {
    let mut ret: s32 = 0;
    let mut args_0: __builtin_va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    ret = GfxPrint_VPrintf(this, fmt, args_0);
    return ret;
}
