#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
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
pub struct struct_801664F0 {
    pub type_0: u32_0,
    pub setScissor: u32_0,
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
}
// Note : This file is related to z_vismono, the original name was probably z_vis<something before "mono"
// alphabetically>
#[no_mangle]
pub static mut D_8012AC00: [Gfx; 5] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xef as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((0 as libc::c_int) << 4 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              6 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              8 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              9 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              12 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              14 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              16 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              17 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              19 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              20 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              23 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((0 as libc::c_int) << 0 as libc::c_int |
                                     (1 as libc::c_int) << 2 as libc::c_int |
                                     0x40 as libc::c_int |
                                     0x4000 as libc::c_int |
                                     (0 as libc::c_int) << 30 as libc::c_int |
                                     (3 as libc::c_int) << 26 as libc::c_int |
                                     (2 as libc::c_int) << 22 as libc::c_int |
                                     (1 as libc::c_int) << 18 as libc::c_int |
                                     0x40 as libc::c_int |
                                     0x4000 as libc::c_int |
                                     (0 as libc::c_int) << 28 as libc::c_int |
                                     (3 as libc::c_int) << 24 as libc::c_int |
                                     (2 as libc::c_int) << 20 as libc::c_int |
                                     (1 as libc::c_int) << 16 as libc::c_int)
                                    as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xf6 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((320 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        14 as libc::c_int |
                                    ((240 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,
                            w1:
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           10 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 14 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xe7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xf9 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1:
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 8 as libc::c_int
                                    |
                                    (8 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
#[no_mangle]
pub static mut D_8012AC28: [Gfx; 3] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xef as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((0 as libc::c_int) << 4 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              6 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              8 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              9 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              12 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              14 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              16 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              17 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              19 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              20 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              23 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((0 as libc::c_int) << 0 as libc::c_int |
                                     (1 as libc::c_int) << 2 as libc::c_int |
                                     0x40 as libc::c_int | 0 as libc::c_int |
                                     0 as libc::c_int | 0x4000 as libc::c_int
                                     | (3 as libc::c_int) << 30 as libc::c_int
                                     | (1 as libc::c_int) << 26 as libc::c_int
                                     | (1 as libc::c_int) << 22 as libc::c_int
                                     | (1 as libc::c_int) << 18 as libc::c_int
                                     | (3 as libc::c_int) << 28 as libc::c_int
                                     | (1 as libc::c_int) << 24 as libc::c_int
                                     | (1 as libc::c_int) << 20 as libc::c_int
                                     |
                                     (1 as libc::c_int) << 16 as libc::c_int)
                                    as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xf6 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((320 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        14 as libc::c_int |
                                    ((240 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,
                            w1:
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           10 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 14 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
#[no_mangle]
pub static mut D_8012AC40: [Gfx; 3] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xef as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((0 as libc::c_int) << 4 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              6 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              8 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              9 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              12 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              14 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              16 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              17 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              19 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              20 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              23 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((0 as libc::c_int) << 0 as libc::c_int |
                                     (1 as libc::c_int) << 2 as libc::c_int |
                                     0x40 as libc::c_int | 0 as libc::c_int |
                                     0 as libc::c_int | 0x4000 as libc::c_int
                                     | (0 as libc::c_int) << 30 as libc::c_int
                                     | (3 as libc::c_int) << 26 as libc::c_int
                                     | (1 as libc::c_int) << 22 as libc::c_int
                                     | (1 as libc::c_int) << 18 as libc::c_int
                                     | (0 as libc::c_int) << 28 as libc::c_int
                                     | (3 as libc::c_int) << 24 as libc::c_int
                                     | (1 as libc::c_int) << 20 as libc::c_int
                                     |
                                     (1 as libc::c_int) << 16 as libc::c_int)
                                    as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xf6 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((320 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        14 as libc::c_int |
                                    ((240 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,
                            w1:
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           10 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 14 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
#[no_mangle]
pub static mut D_8012AC58: [Gfx; 6] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xfc as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((31 as libc::c_int as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 4 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          20 as libc::c_int |
                                          (31 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     5 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              15 as libc::c_int |
                                          (7 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     3 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              12 as libc::c_int |
                                          (7 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     3 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              9 as libc::c_int |
                                          ((31 as libc::c_int as u32_0 &
                                                (((0x1 as libc::c_int) <<
                                                      4 as libc::c_int) -
                                                     1 as libc::c_int) as
                                                    libc::c_uint) <<
                                               5 as libc::c_int |
                                               (31 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          5 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   0 as libc::c_int)) &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                (31 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           4 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 28 as libc::c_int |
                                    (3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        15 as libc::c_int |
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        12 as libc::c_int |
                                    (3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 9 as libc::c_int
                                    |
                                    ((31 as libc::c_int as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                4 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         24 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             21 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             18 as libc::c_int |
                                         (3 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             6 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             3 as libc::c_int |
                                         (3 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             0 as libc::c_int),};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xef as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((1 as libc::c_int) << 4 as libc::c_int |
                                          (3 as libc::c_int) <<
                                              6 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              8 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              9 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              12 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              14 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              16 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              17 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              19 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              20 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              23 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((0 as libc::c_int) << 0 as libc::c_int |
                                     (1 as libc::c_int) << 2 as libc::c_int |
                                     0x40 as libc::c_int |
                                     0x300 as libc::c_int |
                                     0x4000 as libc::c_int | 0 as libc::c_int
                                     | (0 as libc::c_int) << 30 as libc::c_int
                                     | (0 as libc::c_int) << 26 as libc::c_int
                                     | (1 as libc::c_int) << 22 as libc::c_int
                                     | (0 as libc::c_int) << 18 as libc::c_int
                                     | 0x40 as libc::c_int |
                                     0x300 as libc::c_int |
                                     0x4000 as libc::c_int | 0 as libc::c_int
                                     | (0 as libc::c_int) << 28 as libc::c_int
                                     | (0 as libc::c_int) << 24 as libc::c_int
                                     | (1 as libc::c_int) << 20 as libc::c_int
                                     |
                                     (0 as libc::c_int) << 16 as libc::c_int)
                                    as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xf6 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((320 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        14 as libc::c_int |
                                    ((240 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,
                            w1:
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           10 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 14 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xef as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((0 as libc::c_int) << 4 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              6 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              8 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              9 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              12 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              14 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              16 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              17 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              19 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              20 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              23 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((0 as libc::c_int) << 0 as libc::c_int |
                                     (1 as libc::c_int) << 2 as libc::c_int |
                                     0x40 as libc::c_int | 0 as libc::c_int |
                                     0 as libc::c_int | 0x4000 as libc::c_int
                                     | (0 as libc::c_int) << 30 as libc::c_int
                                     | (3 as libc::c_int) << 26 as libc::c_int
                                     | (1 as libc::c_int) << 22 as libc::c_int
                                     | (1 as libc::c_int) << 18 as libc::c_int
                                     | (0 as libc::c_int) << 28 as libc::c_int
                                     | (3 as libc::c_int) << 24 as libc::c_int
                                     | (1 as libc::c_int) << 20 as libc::c_int
                                     |
                                     (1 as libc::c_int) << 16 as libc::c_int)
                                    as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xf6 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((320 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        14 as libc::c_int |
                                    ((240 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,
                            w1:
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           10 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 14 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
// Init
#[no_mangle]
pub unsafe extern "C" fn func_800ACE70(mut this: *mut struct_801664F0) {
    (*this).type_0 = 0 as libc::c_int as u32_0;
    (*this).setScissor = 0 as libc::c_int as u32_0;
    (*this).color.c2rust_unnamed.r = 255 as libc::c_int as u8_0;
    (*this).color.c2rust_unnamed.g = 255 as libc::c_int as u8_0;
    (*this).color.c2rust_unnamed.b = 255 as libc::c_int as u8_0;
    (*this).color.c2rust_unnamed.a = 255 as libc::c_int as u8_0;
}
// Destroy
#[no_mangle]
pub unsafe extern "C" fn func_800ACE90(mut this: *mut struct_801664F0) { }
// Draw
#[no_mangle]
pub unsafe extern "C" fn func_800ACE98(mut this: *mut struct_801664F0,
                                       mut gfxp: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = *gfxp;
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
        (0xee as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        (-(1 as libc::c_int) as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (-(1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    if (*this).setScissor == 1 as libc::c_int as libc::c_uint {
        let fresh2 = gfx;
        gfx = gfx.offset(1);
        let mut _g_1: *mut Gfx = fresh2;
        (*_g_1).words.w0 =
            (0xed as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int
                     as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int
                     as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((320 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int
                     as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((240 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int
                     as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    }
    match (*this).type_0 {
        1 => {
            let fresh3 = gfx;
            gfx = gfx.offset(1);
            let mut _g_2: *mut Gfx = fresh3;
            (*_g_2).words.w0 =
                (0xde as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 = D_8012AC40.as_mut_ptr() as libc::c_uint
        }
        2 => {
            let fresh4 = gfx;
            gfx = gfx.offset(1);
            let mut _g_3: *mut Gfx = fresh4;
            (*_g_3).words.w0 =
                (0xfa as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_3).words.w1 = (*this).color.rgba;
            let fresh5 = gfx;
            gfx = gfx.offset(1);
            let mut _g_4: *mut Gfx = fresh5;
            (*_g_4).words.w0 =
                (0xde as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 = D_8012AC58.as_mut_ptr() as libc::c_uint
        }
        3 => {
            let fresh6 = gfx;
            gfx = gfx.offset(1);
            let mut _g_5: *mut Gfx = fresh6;
            (*_g_5).words.w0 =
                (0xf9 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_5).words.w1 = (*this).color.rgba;
            let fresh7 = gfx;
            gfx = gfx.offset(1);
            let mut _g_6: *mut Gfx = fresh7;
            (*_g_6).words.w0 =
                (0xde as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_6).words.w1 = D_8012AC00.as_mut_ptr() as libc::c_uint
        }
        4 => {
            let fresh8 = gfx;
            gfx = gfx.offset(1);
            let mut _g_7: *mut Gfx = fresh8;
            (*_g_7).words.w0 =
                (0xf8 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_7).words.w1 = (*this).color.rgba;
            let fresh9 = gfx;
            gfx = gfx.offset(1);
            let mut _g_8: *mut Gfx = fresh9;
            (*_g_8).words.w0 =
                (0xde as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_8).words.w1 = D_8012AC28.as_mut_ptr() as libc::c_uint
        }
        _ => { }
    }
    let fresh10 = gfx;
    gfx = gfx.offset(1);
    let mut _g_9: *mut Gfx = fresh10;
    (*_g_9).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_9).words.w1 = 0 as libc::c_int as libc::c_uint;
    *gfxp = gfx;
}
