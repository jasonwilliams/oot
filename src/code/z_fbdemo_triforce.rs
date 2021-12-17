#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn guOrtho(_: *mut Mtx, _: f32_0, _: f32_0, _: f32_0, _: f32_0, _: f32_0,
               _: f32_0, _: f32_0);
    #[no_mangle]
    fn guTranslate(m: *mut Mtx, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn guRotate(_: *mut Mtx, angle: f32_0, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn guScale(m: *mut Mtx, x: f32_0, y: f32_0, z: f32_0);
}
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type f32_0 = libc::c_float;
pub type Mtx_t = [[libc::c_long; 4]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub union Mtx {
    pub m: Mtx_t,
    pub c2rust_unnamed: C2RustUnnamed,
    pub forc_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub intPart: [[u16_0; 4]; 4],
    pub fracPart: [[u16_0; 4]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vtx_t {
    pub ob: [libc::c_short; 3],
    pub flag: libc::c_ushort,
    pub tc: [libc::c_short; 2],
    pub cn: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vtx_tn {
    pub ob: [libc::c_short; 3],
    pub flag: libc::c_ushort,
    pub tc: [libc::c_short; 2],
    pub n: [libc::c_schar; 3],
    pub a: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vtx {
    pub v: Vtx_t,
    pub n: Vtx_tn,
    pub force_structure_alignment: libc::c_longlong,
}
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
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionTriforce {
    pub color: Color_RGBA8_u32,
    pub transPos: f32_0,
    pub step: f32_0,
    pub state: s32,
    pub fadeDirection: s32,
    pub projection: Mtx,
    pub frame: s32,
    pub modelView: [[Mtx; 3]; 2],
}
#[no_mangle]
pub static mut sTriforceWipeDL: [Gfx; 6] =
    [Gfx{words:
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
                                (0xd7 as libc::c_int as u32_0 &
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
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        11 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 8 as libc::c_int
                                    |
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               7 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        1 as libc::c_int,
                            w1:
                                (0x8000 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           16 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 16 as libc::c_int |
                                    (0x8000 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xd9 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (!((0x1 as libc::c_int |
                                            0x4 as libc::c_int |
                                            0x600 as libc::c_int |
                                            0x10000 as libc::c_int |
                                            0x20000 as libc::c_int |
                                            0x40000 as libc::c_int |
                                            0x80000 as libc::c_int |
                                            0x100000 as libc::c_int |
                                            0x200000 as libc::c_int) as u32_0)
                                         &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1: 0 as libc::c_int as u32_0,};
                 init
             },},
     Gfx{words:
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
                                    (1 as libc::c_int as u32_0 &
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
                                    (4 as libc::c_int as u32_0 &
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
                                         (1 as libc::c_int as u32_0 &
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
                                         (4 as libc::c_int as u32_0 &
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
                                    (((3 as libc::c_int) << 4 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              6 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              8 as libc::c_int |
                                          (6 as libc::c_int) <<
                                              9 as libc::c_int |
                                          (2 as libc::c_int) <<
                                              12 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              14 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              16 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              17 as libc::c_int |
                                          (1 as libc::c_int) <<
                                              19 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              20 as libc::c_int |
                                          (1 as libc::c_int) <<
                                              23 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((0 as libc::c_int) << 0 as libc::c_int |
                                     (0 as libc::c_int) << 2 as libc::c_int |
                                     0x8 as libc::c_int | 0x40 as libc::c_int
                                     | 0 as libc::c_int | 0 as libc::c_int |
                                     0x2000 as libc::c_int |
                                     (0 as libc::c_int) << 30 as libc::c_int |
                                     (0 as libc::c_int) << 26 as libc::c_int |
                                     (1 as libc::c_int) << 22 as libc::c_int |
                                     (1 as libc::c_int) << 18 as libc::c_int |
                                     0x8 as libc::c_int | 0x40 as libc::c_int
                                     | 0 as libc::c_int | 0 as libc::c_int |
                                     0x2000 as libc::c_int |
                                     (0 as libc::c_int) << 28 as libc::c_int |
                                     (0 as libc::c_int) << 24 as libc::c_int |
                                     (1 as libc::c_int) << 20 as libc::c_int |
                                     (1 as libc::c_int) << 16 as libc::c_int)
                                    as libc::c_uint,};
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
pub static mut sTriforceWipeVtx: [Vtx; 10] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                577 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [1000 as libc::c_int as libc::c_short,
                                -(1154 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(1000 as libc::c_int) as libc::c_short,
                                -(1154 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(1154 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [500 as libc::c_int as libc::c_short,
                                -(288 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(500 as libc::c_int) as libc::c_short,
                                -(288 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(32000 as libc::c_int) as libc::c_short,
                                32000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [32000 as libc::c_int as libc::c_short,
                                32000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [32000 as libc::c_int as libc::c_short,
                                -(32000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(32000 as libc::c_int) as libc::c_short,
                                -(32000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_Start(mut thisx:
                                                      *mut libc::c_void) {
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    match (*this).state {
        1 | 2 => { (*this).transPos = 1.0f32; return }
        _ => { }
    }
    (*this).transPos = 0.03f32;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_Init(mut thisx: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    bzero(this as *mut libc::c_void,
          ::std::mem::size_of::<TransitionTriforce>() as libc::c_ulong);
    guOrtho(&mut (*this).projection, -160.0f32, 160.0f32, -120.0f32, 120.0f32,
            -1000.0f32, 1000.0f32, 1.0f32);
    (*this).transPos = 1.0f32;
    (*this).state = 2 as libc::c_int;
    (*this).step = 0.015f32;
    (*this).fadeDirection = 1 as libc::c_int;
    return this as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_Destroy(mut thisx:
                                                        *mut libc::c_void) {
}
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_Update(mut thisx:
                                                       *mut libc::c_void,
                                                   mut updateRate: s32) {
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    let mut temp_f0: f32_0 = 0.;
    let mut i: s32 = 0;
    i = updateRate;
    while i > 0 as libc::c_int {
        if (*this).state == 1 as libc::c_int {
            (*this).transPos =
                if (*this).transPos * (1.0f32 - (*this).step) < 0.03f32 {
                    0.03f32
                } else { ((*this).transPos) * (1.0f32 - (*this).step) }
        } else if (*this).state == 2 as libc::c_int {
            (*this).transPos =
                if (*this).transPos - (*this).step < 0.03f32 {
                    0.03f32
                } else { ((*this).transPos) - (*this).step }
        } else if (*this).state == 3 as libc::c_int {
            (*this).transPos =
                if (*this).transPos / (1.0f32 - (*this).step) > 1.0f32 {
                    1.0f32
                } else { ((*this).transPos) / (1.0f32 - (*this).step) }
        } else if (*this).state == 4 as libc::c_int {
            (*this).transPos =
                if (*this).transPos + (*this).step > 1.0f32 {
                    1.0f32
                } else { ((*this).transPos) + (*this).step }
        }
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_SetColor(mut thisx:
                                                         *mut libc::c_void,
                                                     mut color: u32_0) {
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    (*this).color.rgba = color;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_SetType(mut thisx:
                                                        *mut libc::c_void,
                                                    mut type_0: s32) {
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    (*this).fadeDirection = type_0;
}
// unused
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_SetState(mut thisx:
                                                         *mut libc::c_void,
                                                     mut state: s32) {
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    (*this).state = state;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_Draw(mut thisx: *mut libc::c_void,
                                                 mut gfxP: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = *gfxP;
    let mut modelView: *mut Mtx = 0 as *mut Mtx;
    let mut scale: f32_0 = 0.;
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    let mut pad: s32 = 0;
    let mut rotation: f32_0 = (*this).transPos * 360.0f32;
    modelView = (*this).modelView[(*this).frame as usize].as_mut_ptr();
    scale = (*this).transPos * 0.625f32;
    (*this).frame ^= 1 as libc::c_int;
    osSyncPrintf(b"rate=%f tx=%f ty=%f rotate=%f\n\x00" as *const u8 as
                     *const libc::c_char, (*this).transPos as libc::c_double,
                 0.0f32 as libc::c_double, 0.0f32 as libc::c_double,
                 rotation as libc::c_double);
    guScale(&mut *modelView.offset(0 as libc::c_int as isize), scale, scale,
            1.0f32);
    guRotate(&mut *modelView.offset(1 as libc::c_int as isize), rotation,
             0.0f32, 0.0f32, 1.0f32);
    guTranslate(&mut *modelView.offset(2 as libc::c_int as isize), 0.0f32,
                0.0f32, 0.0f32);
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
        (0xde as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = sTriforceWipeDL.as_mut_ptr() as libc::c_uint;
    let fresh2 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 = (*this).color.rgba;
    let fresh3 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
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
            (3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (3 as libc::c_int as u32_0 &
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
                 (3 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (3 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh4 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh4;
    (*_g_3).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0x2 as libc::c_int | 0x4 as libc::c_int) ^ 0x1 as libc::c_int)
                 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = &mut (*this).projection as *mut Mtx as libc::c_uint;
    let fresh5 = gfx;
    gfx = gfx.offset(1);
    let mut _g_4: *mut Gfx = fresh5;
    (*_g_4).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 =
        &mut *modelView.offset(0 as libc::c_int as isize) as *mut Mtx as
            libc::c_uint;
    let fresh6 = gfx;
    gfx = gfx.offset(1);
    let mut _g_5: *mut Gfx = fresh6;
    (*_g_5).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        &mut *modelView.offset(1 as libc::c_int as isize) as *mut Mtx as
            libc::c_uint;
    let fresh7 = gfx;
    gfx = gfx.offset(1);
    let mut _g_6: *mut Gfx = fresh7;
    (*_g_6).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        &mut *modelView.offset(2 as libc::c_int as isize) as *mut Mtx as
            libc::c_uint;
    let fresh8 = gfx;
    gfx = gfx.offset(1);
    let mut _g_7: *mut Gfx = fresh8;
    (*_g_7).words.w0 =
        (0x1 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (10 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int + 10 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 7 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 1 as libc::c_int;
    (*_g_7).words.w1 = sTriforceWipeVtx.as_mut_ptr() as libc::c_uint;
    if TransitionTriforce_IsDone(this as *mut libc::c_void) == 0 {
        match (*this).fadeDirection {
            1 => {
                let fresh9 = gfx;
                gfx = gfx.offset(1);
                let mut _g_8: *mut Gfx = fresh9;
                (*_g_8).words.w0 =
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (if 0 as libc::c_int == 0 as libc::c_int {
                             (((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 16 as libc::c_int |
                                  ((4 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int) |
                                 ((5 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int
                         } else {
                             (if 0 as libc::c_int == 1 as libc::c_int {
                                  (((4 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((5 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((0 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              } else {
                                  (((5 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((0 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((4 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              })
                         });
                (*_g_8).words.w1 =
                    if 0 as libc::c_int == 0 as libc::c_int {
                        (((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else if 0 as libc::c_int == 1 as libc::c_int {
                        (((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else {
                        (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    };
                let fresh10 = gfx;
                gfx = gfx.offset(1);
                let mut _g_9: *mut Gfx = fresh10;
                (*_g_9).words.w0 =
                    (0x5 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (if 0 as libc::c_int == 0 as libc::c_int {
                             (((5 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 16 as libc::c_int |
                                  ((3 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int) |
                                 ((2 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int
                         } else {
                             (if 0 as libc::c_int == 1 as libc::c_int {
                                  (((3 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((2 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((5 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              } else {
                                  (((2 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((5 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((3 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              })
                         });
                (*_g_9).words.w1 = 0 as libc::c_int as libc::c_uint
            }
            2 => {
                let fresh11 = gfx;
                gfx = gfx.offset(1);
                let mut _g_10: *mut Gfx = fresh11;
                (*_g_10).words.w0 =
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (if 0 as libc::c_int == 0 as libc::c_int {
                             (((3 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 16 as libc::c_int |
                                  ((4 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int) |
                                 ((5 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int
                         } else {
                             (if 0 as libc::c_int == 1 as libc::c_int {
                                  (((4 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((5 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((3 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              } else {
                                  (((5 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((3 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((4 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              })
                         });
                (*_g_10).words.w1 =
                    if 0 as libc::c_int == 0 as libc::c_int {
                        (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else if 0 as libc::c_int == 1 as libc::c_int {
                        (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else {
                        (((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    };
                let fresh12 = gfx;
                gfx = gfx.offset(1);
                let mut _g_11: *mut Gfx = fresh12;
                (*_g_11).words.w0 =
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (if 0 as libc::c_int == 0 as libc::c_int {
                             (((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 16 as libc::c_int |
                                  ((6 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int) |
                                 ((7 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int
                         } else {
                             (if 0 as libc::c_int == 1 as libc::c_int {
                                  (((6 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((7 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((0 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              } else {
                                  (((7 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((0 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((6 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              })
                         });
                (*_g_11).words.w1 =
                    if 0 as libc::c_int == 0 as libc::c_int {
                        (((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else if 0 as libc::c_int == 1 as libc::c_int {
                        (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else {
                        (((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    };
                let fresh13 = gfx;
                gfx = gfx.offset(1);
                let mut _g_12: *mut Gfx = fresh13;
                (*_g_12).words.w0 =
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (if 0 as libc::c_int == 0 as libc::c_int {
                             (((1 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 16 as libc::c_int |
                                  ((7 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int) |
                                 ((8 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int
                         } else {
                             (if 0 as libc::c_int == 1 as libc::c_int {
                                  (((7 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((8 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((1 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              } else {
                                  (((8 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((1 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((7 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              })
                         });
                (*_g_12).words.w1 =
                    if 0 as libc::c_int == 0 as libc::c_int {
                        (((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else if 0 as libc::c_int == 1 as libc::c_int {
                        (((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else {
                        (((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    };
                let fresh14 = gfx;
                gfx = gfx.offset(1);
                let mut _g_13: *mut Gfx = fresh14;
                (*_g_13).words.w0 =
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (if 0 as libc::c_int == 0 as libc::c_int {
                             (((1 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 16 as libc::c_int |
                                  ((9 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int) |
                                 ((2 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int
                         } else {
                             (if 0 as libc::c_int == 1 as libc::c_int {
                                  (((9 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((2 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((1 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              } else {
                                  (((2 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((1 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((9 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              })
                         });
                (*_g_13).words.w1 =
                    if 0 as libc::c_int == 0 as libc::c_int {
                        (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else if 0 as libc::c_int == 1 as libc::c_int {
                        (((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else {
                        (((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    }
            }
            _ => { }
        }
    } else {
        match (*this).fadeDirection {
            2 => {
                let fresh15 = gfx;
                gfx = gfx.offset(1);
                let mut _g_14: *mut Gfx = fresh15;
                (*_g_14).words.w0 =
                    (0x7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (if 0 as libc::c_int == 0 as libc::c_int {
                             (((6 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 16 as libc::c_int |
                                  ((7 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int) |
                                 ((8 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int
                         } else {
                             (if 0 as libc::c_int == 1 as libc::c_int {
                                  (((7 as libc::c_int * 2 as libc::c_int) as
                                        u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 16 as libc::c_int
                                       |
                                       ((8 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           8 as libc::c_int) |
                                      ((9 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          0 as libc::c_int
                              } else {
                                  (if 0 as libc::c_int == 2 as libc::c_int {
                                       (((8 as libc::c_int * 2 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                            ((9 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                8 as libc::c_int) |
                                           ((6 as libc::c_int *
                                                 2 as libc::c_int) as u32_0 &
                                                (((0x1 as libc::c_int) <<
                                                      8 as libc::c_int) -
                                                     1 as libc::c_int) as
                                                    libc::c_uint) <<
                                               0 as libc::c_int
                                   } else {
                                       (((9 as libc::c_int * 2 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                            ((6 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                8 as libc::c_int) |
                                           ((7 as libc::c_int *
                                                 2 as libc::c_int) as u32_0 &
                                                (((0x1 as libc::c_int) <<
                                                      8 as libc::c_int) -
                                                     1 as libc::c_int) as
                                                    libc::c_uint) <<
                                               0 as libc::c_int
                                   })
                              })
                         });
                (*_g_14).words.w1 =
                    if 0 as libc::c_int == 0 as libc::c_int {
                        (((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else if 0 as libc::c_int == 1 as libc::c_int {
                        (((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else if 0 as libc::c_int == 2 as libc::c_int {
                        (((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    } else {
                        (((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             16 as libc::c_int |
                             ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int) |
                            ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int
                    }
            }
            1 | _ => { }
        }
    }
    let fresh16 = gfx;
    gfx = gfx.offset(1);
    let mut _g_15: *mut Gfx = fresh16;
    (*_g_15).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_15).words.w1 = 0 as libc::c_int as libc::c_uint;
    *gfxP = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionTriforce_IsDone(mut thisx:
                                                       *mut libc::c_void)
 -> s32 {
    let mut this: *mut TransitionTriforce = thisx as *mut TransitionTriforce;
    let mut ret: s32 = 0 as libc::c_int;
    if (*this).state == 1 as libc::c_int || (*this).state == 2 as libc::c_int
       {
        return ((*this).transPos <= 0.03f32) as libc::c_int
    } else {
        if (*this).state == 3 as libc::c_int ||
               (*this).state == 4 as libc::c_int {
            return ((*this).transPos >= 1.0f32) as libc::c_int
        }
    }
    return ret;
}
