#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn SystemArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn SystemArena_FreeDebug(ptr: *mut libc::c_void,
                             file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn guOrtho(_: *mut Mtx, _: f32_0, _: f32_0, _: f32_0, _: f32_0, _: f32_0,
               _: f32_0, _: f32_0);
    #[no_mangle]
    fn guMtxIdent(_: *mut Mtx);
    #[no_mangle]
    fn Sleep_Msec(ms: u32_0);
}
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
pub struct TransitionUnkData {
    pub unk_0: f32_0,
    pub unk_4: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionUnk {
    pub row: s32,
    pub col: s32,
    pub frame: s32,
    pub unk_0C: *mut TransitionUnkData,
    pub vtxFrame1: *mut Vtx,
    pub vtxFrame2: *mut Vtx,
    pub projection: Mtx,
    pub modelView: Mtx,
    pub unk_98: Mtx,
    pub gfx: *mut Gfx,
    pub zBuffer: *mut u16_0,
}
#[no_mangle]
pub static mut D_8012AFB0: [Gfx; 10] =
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
                                (0xe3 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((32 as libc::c_int - 20 as libc::c_int -
                                          2 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 8 as libc::c_int
                                    |
                                    ((2 as libc::c_int - 1 as libc::c_int) as
                                         u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((3 as libc::c_int) << 20 as libc::c_int) as
                                    libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xff as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        21 as libc::c_int |
                                    (2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               2 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        19 as libc::c_int |
                                    ((320 as libc::c_int - 1 as libc::c_int)
                                         as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               12 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1: 0xf000000 as libc::c_int as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xf7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1:
                                (((65 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (65 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      65 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     << 16 as libc::c_int |
                                     ((65 as libc::c_int) << 8 as libc::c_int
                                          & 0xf800 as libc::c_int |
                                          (65 as libc::c_int) <<
                                              3 as libc::c_int &
                                              0x7c0 as libc::c_int |
                                          65 as libc::c_int >>
                                              2 as libc::c_int &
                                              0x3e as libc::c_int |
                                          1 as libc::c_int &
                                              0x1 as libc::c_int)) as
                                    libc::c_uint,};
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
                                    (319 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        14 as libc::c_int |
                                    (239 as libc::c_int as u32_0 &
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
                                (0xf7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1:
                                (((65 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (65 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      255 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     << 16 as libc::c_int |
                                     ((65 as libc::c_int) << 8 as libc::c_int
                                          & 0xf800 as libc::c_int |
                                          (65 as libc::c_int) <<
                                              3 as libc::c_int &
                                              0x7c0 as libc::c_int |
                                          255 as libc::c_int >>
                                              2 as libc::c_int &
                                              0x3e as libc::c_int |
                                          1 as libc::c_int &
                                              0x1 as libc::c_int)) as
                                    libc::c_uint,};
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
                                    (300 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        14 as libc::c_int |
                                    (220 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               10 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        2 as libc::c_int,
                            w1:
                                (20 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           10 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 14 as libc::c_int |
                                    (20 as libc::c_int as u32_0 &
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
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
#[no_mangle]
pub static mut D_8012B000: [Gfx; 6] =
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
pub unsafe extern "C" fn TransitionUnk_InitGraphics(mut this:
                                                        *mut TransitionUnk) {
    let mut row2: s32 = 0;
    let mut pad2: s32 = 0;
    let mut pad3: s32 = 0;
    let mut vtx2: *mut Vtx_t = 0 as *mut Vtx_t;
    let mut frame: s32 = 0;
    let mut rowTex: s32 = 0;
    let mut row: s32 = 0;
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    let mut vtx: *mut Vtx = 0 as *mut Vtx;
    let mut col: s32 = 0;
    let mut colTex: s32 = 0;
    guMtxIdent(&mut (*this).modelView);
    guMtxIdent(&mut (*this).unk_98);
    guOrtho(&mut (*this).projection, 0.0f32, 320 as libc::c_int as f32_0,
            240 as libc::c_int as f32_0, 0.0f32, -1000.0f32, 1000.0f32,
            1.0f32);
    frame = 0 as libc::c_int;
    while frame < 2 as libc::c_int {
        (*this).frame = frame;
        vtx =
            if (*this).frame == 0 as libc::c_int {
                (*this).vtxFrame1
            } else { (*this).vtxFrame2 };
        colTex = 0 as libc::c_int;
        col = 0 as libc::c_int;
        while col < (*this).col + 1 as libc::c_int {
            rowTex = 0 as libc::c_int;
            row = 0 as libc::c_int;
            while row < (*this).row + 1 as libc::c_int {
                vtx2 = &mut (*vtx).v;
                vtx = vtx.offset(1);
                (*vtx2).tc[0 as libc::c_int as usize] =
                    (rowTex << 6 as libc::c_int) as libc::c_short;
                (*vtx2).ob[0 as libc::c_int as usize] =
                    (row * 0x20 as libc::c_int) as libc::c_short;
                (*vtx2).ob[1 as libc::c_int as usize] =
                    (col * 0x20 as libc::c_int) as libc::c_short;
                (*vtx2).ob[2 as libc::c_int as usize] =
                    -(5 as libc::c_int) as libc::c_short;
                (*vtx2).flag = 0 as libc::c_int as libc::c_ushort;
                (*vtx2).tc[1 as libc::c_int as usize] =
                    (colTex << 6 as libc::c_int) as libc::c_short;
                (*vtx2).cn[0 as libc::c_int as usize] =
                    0 as libc::c_int as libc::c_uchar;
                (*vtx2).cn[1 as libc::c_int as usize] =
                    0 as libc::c_int as libc::c_uchar;
                (*vtx2).cn[2 as libc::c_int as usize] =
                    120 as libc::c_int as libc::c_uchar;
                (*vtx2).cn[3 as libc::c_int as usize] =
                    255 as libc::c_int as libc::c_uchar;
                rowTex += 0x20 as libc::c_int;
                row += 1
            }
            colTex += 0x20 as libc::c_int;
            col += 1
        }
        frame += 1
    }
    gfx = (*this).gfx;
    colTex = 0 as libc::c_int;
    col = 0 as libc::c_int;
    while col < (*this).col {
        let fresh0 = gfx;
        gfx = gfx.offset(1);
        let mut _g: *mut Gfx = fresh0;
        (*_g).words.w0 =
            (0x1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((2 as libc::c_int * ((*this).row + 1 as libc::c_int)) as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((0 as libc::c_int +
                      2 as libc::c_int * ((*this).row + 1 as libc::c_int)) as
                     u32_0 &
                     (((0x1 as libc::c_int) << 7 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    1 as libc::c_int;
        (*_g).words.w1 =
            (((0xa as libc::c_int) << 24 as libc::c_int) as
                 libc::c_uint).wrapping_add((col as
                                                 u32_0).wrapping_mul(((*this).row
                                                                          +
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         libc::c_uint).wrapping_mul(::std::mem::size_of::<Vtx>()
                                                                                                        as
                                                                                                        libc::c_ulong));
        rowTex = 0 as libc::c_int;
        row = 0 as libc::c_int;
        row2 = 0 as libc::c_int;
        while row < (*this).row {
            let fresh1 = gfx;
            gfx = gfx.offset(1);
            let mut _g_0: *mut Gfx = fresh1;
            (*_g_0).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh2 = gfx;
            gfx = gfx.offset(1);
            let mut _g_1: *mut Gfx = fresh2;
            (*_g_1).words.w0 =
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
                    ((320 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 =
                (((0xb as libc::c_int) << 24 as libc::c_int) +
                     0 as libc::c_int) as libc::c_uint;
            let fresh3 = gfx;
            gfx = gfx.offset(1);
            let mut _g_2: *mut Gfx = fresh3;
            (*_g_2).words.w0 =
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
                    (((rowTex + 0x20 as libc::c_int - rowTex +
                           1 as libc::c_int) * 2 as libc::c_int +
                          7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 =
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
            let fresh4 = gfx;
            gfx = gfx.offset(1);
            let mut _g_3: *mut Gfx = fresh4;
            (*_g_3).words.w0 =
                (0xe6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_3).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh5 = gfx;
            gfx = gfx.offset(1);
            let mut _g_4: *mut Gfx = fresh5;
            (*_g_4).words.w0 =
                (0xf4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((rowTex << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((colTex << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((rowTex + 0x20 as libc::c_int) << 2 as libc::c_int) as
                         u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((colTex + 0x20 as libc::c_int) << 2 as libc::c_int) as
                         u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh6 = gfx;
            gfx = gfx.offset(1);
            let mut _g_5: *mut Gfx = fresh6;
            (*_g_5).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh7 = gfx;
            gfx = gfx.offset(1);
            let mut _g_6: *mut Gfx = fresh7;
            (*_g_6).words.w0 =
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
                    (((rowTex + 0x20 as libc::c_int - rowTex +
                           1 as libc::c_int) * 2 as libc::c_int +
                          7 as libc::c_int >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_6).words.w1 =
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
            let fresh8 = gfx;
            gfx = gfx.offset(1);
            let mut _g_7: *mut Gfx = fresh8;
            (*_g_7).words.w0 =
                (0xf2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((rowTex << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((colTex << 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_7).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((rowTex + 0x20 as libc::c_int) << 2 as libc::c_int) as
                         u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((colTex + 0x20 as libc::c_int) << 2 as libc::c_int) as
                         u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh9 = gfx;
            gfx = gfx.offset(1);
            let mut _g_8: *mut Gfx = fresh9;
            (*_g_8).words.w0 =
                (0x7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (if 0 as libc::c_int == 0 as libc::c_int {
                         (((row * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              16 as libc::c_int |
                              (((row + 1 as libc::c_int) * 2 as libc::c_int)
                                   as u32_0 &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 8 as libc::c_int) |
                             (((row2 + (*this).row + 2 as libc::c_int) *
                                   2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int
                     } else {
                         (if 0 as libc::c_int == 1 as libc::c_int {
                              ((((row + 1 as libc::c_int) * 2 as libc::c_int)
                                    as u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   16 as libc::c_int |
                                   (((row2 + (*this).row + 2 as libc::c_int) *
                                         2 as libc::c_int) as u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              8 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) << 8 as libc::c_int)
                                  |
                                  ((((*this).row + row2 + 1 as libc::c_int) *
                                        2 as libc::c_int) as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 0 as libc::c_int
                          } else {
                              (if 0 as libc::c_int == 2 as libc::c_int {
                                   ((((row2 + (*this).row + 2 as libc::c_int)
                                          * 2 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        ((((*this).row + row2 +
                                               1 as libc::c_int) *
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int) |
                                       ((row * 2 as libc::c_int) as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           0 as libc::c_int
                               } else {
                                   (((((*this).row + row2 + 1 as libc::c_int)
                                          * 2 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        ((row * 2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int) |
                                       (((row + 1 as libc::c_int) *
                                             2 as libc::c_int) as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           0 as libc::c_int
                               })
                          })
                     });
            (*_g_8).words.w1 =
                if 0 as libc::c_int == 0 as libc::c_int {
                    (((row * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         16 as libc::c_int |
                         (((row2 + (*this).row + 2 as libc::c_int) *
                               2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             8 as libc::c_int) |
                        ((((*this).row + row2 + 1 as libc::c_int) *
                              2 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
                } else if 0 as libc::c_int == 1 as libc::c_int {
                    ((((row + 1 as libc::c_int) * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         16 as libc::c_int |
                         ((((*this).row + row2 + 1 as libc::c_int) *
                               2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             8 as libc::c_int) |
                        ((row * 2 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
                } else if 0 as libc::c_int == 2 as libc::c_int {
                    ((((row2 + (*this).row + 2 as libc::c_int) *
                           2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         16 as libc::c_int |
                         ((row * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             8 as libc::c_int) |
                        (((row + 1 as libc::c_int) * 2 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
                } else {
                    (((((*this).row + row2 + 1 as libc::c_int) *
                           2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         16 as libc::c_int |
                         (((row + 1 as libc::c_int) * 2 as libc::c_int) as
                              u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             8 as libc::c_int) |
                        (((row2 + (*this).row + 2 as libc::c_int) *
                              2 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
                };
            rowTex += 0x20 as libc::c_int;
            row2 += 1;
            row += 1
        }
        colTex += 0x20 as libc::c_int;
        col += 1
    }
    let fresh10 = gfx;
    gfx = gfx.offset(1);
    let mut _g_9: *mut Gfx = fresh10;
    (*_g_9).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_9).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh11 = gfx;
    gfx = gfx.offset(1);
    let mut _g_10: *mut Gfx = fresh11;
    (*_g_10).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_10).words.w1 = 0 as libc::c_int as libc::c_uint;
    LogUtils_LogThreadId(b"../z_fbdemo.c\x00" as *const u8 as
                             *const libc::c_char, 144 as libc::c_int);
    osSyncPrintf(b"this->col * (1 + this->row * (1 + 7 + 1)) + 1 + 1 = %d\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*this).col *
                     (1 as libc::c_int + (*this).row * 9 as libc::c_int) +
                     2 as libc::c_int);
    LogUtils_LogThreadId(b"../z_fbdemo.c\x00" as *const u8 as
                             *const libc::c_char, 145 as libc::c_int);
    osSyncPrintf(b"gp - this->gfxtbl = %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 gfx.wrapping_offset_from((*this).gfx) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn TransitionUnk_InitData(mut this:
                                                    *mut TransitionUnk) {
    let mut col: s32 = 0;
    let mut row: s32 = 0;
    col = 0 as libc::c_int;
    while col < (*this).col + 1 as libc::c_int {
        row = 0 as libc::c_int;
        while row < (*this).row + 1 as libc::c_int {
            (*(*this).unk_0C.offset(row as
                                        isize).offset((col *
                                                           ((*this).row +
                                                                1 as
                                                                    libc::c_int))
                                                          as isize)).unk_0 =
                (row * 32 as libc::c_int) as f32_0;
            (*(*this).unk_0C.offset(row as
                                        isize).offset((col *
                                                           ((*this).row +
                                                                1 as
                                                                    libc::c_int))
                                                          as isize)).unk_4 =
                (col * 32 as libc::c_int) as f32_0;
            row += 1
        }
        col += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionUnk_Destroy(mut this: *mut TransitionUnk) {
    osSyncPrintf(b"fbdemo_cleanup(%08x)\n\x00" as *const u8 as
                     *const libc::c_char, this);
    osSyncPrintf(b"msleep(100);\n\x00" as *const u8 as *const libc::c_char);
    Sleep_Msec(100 as libc::c_int as u32_0);
    if !(*this).unk_0C.is_null() {
        SystemArena_FreeDebug((*this).unk_0C as *mut libc::c_void,
                              b"../z_fbdemo.c\x00" as *const u8 as
                                  *const libc::c_char, 180 as libc::c_int);
        (*this).unk_0C = 0 as *mut TransitionUnkData
    }
    if !(*this).vtxFrame1.is_null() {
        SystemArena_FreeDebug((*this).vtxFrame1 as *mut libc::c_void,
                              b"../z_fbdemo.c\x00" as *const u8 as
                                  *const libc::c_char, 181 as libc::c_int);
        (*this).vtxFrame1 = 0 as *mut Vtx
    }
    if !(*this).vtxFrame2.is_null() {
        SystemArena_FreeDebug((*this).vtxFrame2 as *mut libc::c_void,
                              b"../z_fbdemo.c\x00" as *const u8 as
                                  *const libc::c_char, 182 as libc::c_int);
        (*this).vtxFrame2 = 0 as *mut Vtx
    }
    if !(*this).gfx.is_null() {
        SystemArena_FreeDebug((*this).gfx as *mut libc::c_void,
                              b"../z_fbdemo.c\x00" as *const u8 as
                                  *const libc::c_char, 183 as libc::c_int);
        (*this).gfx = 0 as *mut Gfx
    };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionUnk_Init(mut this: *mut TransitionUnk,
                                            mut row: s32, mut col: s32)
 -> *mut TransitionUnk {
    osSyncPrintf(b"fbdemo_init(%08x, %d, %d)\n\x00" as *const u8 as
                     *const libc::c_char, this, row, col);
    bzero(this as *mut libc::c_void,
          ::std::mem::size_of::<TransitionUnk>() as libc::c_ulong);
    (*this).frame = 0 as libc::c_int;
    (*this).row = row;
    (*this).col = col;
    (*this).unk_0C =
        SystemArena_MallocDebug(((row + 1 as libc::c_int) as
                                     libc::c_uint).wrapping_mul(::std::mem::size_of::<TransitionUnkData>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul((col
                                                                                                     +
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                                    as
                                                                                                    libc::c_uint),
                                b"../z_fbdemo.c\x00" as *const u8 as
                                    *const libc::c_char, 195 as libc::c_int)
            as *mut TransitionUnkData;
    (*this).vtxFrame1 =
        SystemArena_MallocDebug(((row + 1 as libc::c_int) as
                                     libc::c_uint).wrapping_mul(::std::mem::size_of::<Vtx>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul((col
                                                                                                     +
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                                    as
                                                                                                    libc::c_uint),
                                b"../z_fbdemo.c\x00" as *const u8 as
                                    *const libc::c_char, 196 as libc::c_int)
            as *mut Vtx;
    (*this).vtxFrame2 =
        SystemArena_MallocDebug(((row + 1 as libc::c_int) as
                                     libc::c_uint).wrapping_mul(::std::mem::size_of::<Vtx>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul((col
                                                                                                     +
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                                    as
                                                                                                    libc::c_uint),
                                b"../z_fbdemo.c\x00" as *const u8 as
                                    *const libc::c_char, 197 as libc::c_int)
            as *mut Vtx;
    (*this).gfx =
        SystemArena_MallocDebug((((*this).col *
                                      (1 as libc::c_int +
                                           (*this).row * 9 as libc::c_int) +
                                      2 as libc::c_int) as
                                     libc::c_uint).wrapping_mul(::std::mem::size_of::<Gfx>()
                                                                    as
                                                                    libc::c_ulong),
                                b"../z_fbdemo.c\x00" as *const u8 as
                                    *const libc::c_char, 198 as libc::c_int)
            as *mut Gfx;
    if (*this).unk_0C.is_null() || (*this).vtxFrame1.is_null() ||
           (*this).vtxFrame2.is_null() || (*this).gfx.is_null() {
        osSyncPrintf(b"fbdemo_init allocation error\n\x00" as *const u8 as
                         *const libc::c_char);
        if !(*this).unk_0C.is_null() {
            SystemArena_FreeDebug((*this).unk_0C as *mut libc::c_void,
                                  b"../z_fbdemo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  202 as libc::c_int);
            (*this).unk_0C = 0 as *mut TransitionUnkData
        }
        if !(*this).vtxFrame1.is_null() {
            SystemArena_FreeDebug((*this).vtxFrame1 as *mut libc::c_void,
                                  b"../z_fbdemo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  203 as libc::c_int);
            (*this).vtxFrame1 = 0 as *mut Vtx
        }
        if !(*this).vtxFrame2.is_null() {
            SystemArena_FreeDebug((*this).vtxFrame2 as *mut libc::c_void,
                                  b"../z_fbdemo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  204 as libc::c_int);
            (*this).vtxFrame2 = 0 as *mut Vtx
        }
        if !(*this).gfx.is_null() {
            SystemArena_FreeDebug((*this).gfx as *mut libc::c_void,
                                  b"../z_fbdemo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  205 as libc::c_int);
            (*this).gfx = 0 as *mut Gfx
        }
        return 0 as *mut TransitionUnk
    }
    TransitionUnk_InitGraphics(this);
    TransitionUnk_InitData(this);
    (*this).frame = 0 as libc::c_int;
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionUnk_SetData(mut this: *mut TransitionUnk) {
    let mut col: s32 = 0;
    let mut vtx: *mut Vtx = 0 as *mut Vtx;
    let mut row: s32 = 0;
    col = 0 as libc::c_int;
    while col < (*this).col + 1 as libc::c_int {
        row = 0 as libc::c_int;
        while row < (*this).row + 1 as libc::c_int {
            vtx =
                if (*this).frame == 0 as libc::c_int {
                    (*this).vtxFrame1
                } else { (*this).vtxFrame2 };
            (*vtx.offset(row as
                             isize).offset((col *
                                                ((*this).row +
                                                     1 as libc::c_int)) as
                                               isize)).v.ob[0 as libc::c_int
                                                                as usize] =
                (*(*this).unk_0C.offset(row as
                                            isize).offset((col *
                                                               ((*this).row +
                                                                    1 as
                                                                        libc::c_int))
                                                              as isize)).unk_0
                    as libc::c_short;
            vtx =
                if (*this).frame == 0 as libc::c_int {
                    (*this).vtxFrame1
                } else { (*this).vtxFrame2 };
            (*vtx.offset(row as
                             isize).offset((col *
                                                ((*this).row +
                                                     1 as libc::c_int)) as
                                               isize)).v.ob[1 as libc::c_int
                                                                as usize] =
                (*(*this).unk_0C.offset(row as
                                            isize).offset((col *
                                                               ((*this).row +
                                                                    1 as
                                                                        libc::c_int))
                                                              as isize)).unk_4
                    as libc::c_short;
            row += 1
        }
        col += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionUnk_Draw(mut this: *mut TransitionUnk,
                                            mut gfxP: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = *gfxP;
    let fresh12 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh12;
    (*_g).words.w0 =
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
    (*_g).words.w1 = D_8012B000.as_mut_ptr() as libc::c_uint;
    TransitionUnk_SetData(this);
    let fresh13 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh13;
    (*_g_0).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = &mut (*this).projection as *mut Mtx as libc::c_uint;
    let fresh14 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh14;
    (*_g_1).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = &mut (*this).modelView as *mut Mtx as libc::c_uint;
    let fresh15 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh15;
    (*_g_2).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xa as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        if (*this).frame == 0 as libc::c_int {
            (*this).vtxFrame1
        } else { (*this).vtxFrame2 } as libc::c_uint;
    let fresh16 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh16;
    (*_g_3).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xb as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = (*this).zBuffer as libc::c_uint;
    let fresh17 = gfx;
    gfx = gfx.offset(1);
    let mut _g_4: *mut Gfx = fresh17;
    (*_g_4).words.w0 =
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
    (*_g_4).words.w1 = D_8012B000.as_mut_ptr() as libc::c_uint;
    let fresh18 = gfx;
    gfx = gfx.offset(1);
    let mut _g_5: *mut Gfx = fresh18;
    (*_g_5).words.w0 =
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
    (*_g_5).words.w1 = (*this).gfx as libc::c_uint;
    let fresh19 = gfx;
    gfx = gfx.offset(1);
    let mut _g_6: *mut Gfx = fresh19;
    (*_g_6).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_6).words.w1 = 0 as libc::c_int as libc::c_uint;
    (*this).frame ^= 1 as libc::c_int;
    *gfxP = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionUnk_Update(mut this: *mut TransitionUnk) {
    let mut temp_f00: f32_0 = 0.;
    let mut temp_f12: f32_0 = 0.;
    let mut col: s32 = 0;
    let mut phi_f14: f32_0 = 0.;
    let mut row: s32 = 0;
    col = 0 as libc::c_int;
    while col < (*this).col + 1 as libc::c_int {
        row = 0 as libc::c_int;
        while row < (*this).row + 1 as libc::c_int {
            temp_f00 =
                (*(*this).unk_0C.offset(row as
                                            isize).offset((col *
                                                               ((*this).row +
                                                                    1 as
                                                                        libc::c_int))
                                                              as isize)).unk_0
                    -
                    (*(*this).unk_0C.offset(5 as libc::c_int as
                                                isize).offset((4 as
                                                                   libc::c_int
                                                                   *
                                                                   ((*this).row
                                                                        +
                                                                        1 as
                                                                            libc::c_int))
                                                                  as
                                                                  isize)).unk_0;
            temp_f12 =
                (*(*this).unk_0C.offset(row as
                                            isize).offset((col *
                                                               ((*this).row +
                                                                    1 as
                                                                        libc::c_int))
                                                              as isize)).unk_4
                    -
                    (*(*this).unk_0C.offset(5 as libc::c_int as
                                                isize).offset((4 as
                                                                   libc::c_int
                                                                   *
                                                                   ((*this).row
                                                                        +
                                                                        1 as
                                                                            libc::c_int))
                                                                  as
                                                                  isize)).unk_4;
            phi_f14 = (temp_f00 * temp_f00 + temp_f12 * temp_f12) / 100.0f32;
            if phi_f14 != 0.0f32 {
                if phi_f14 < 1.0f32 { phi_f14 = 1.0f32 }
                let ref mut fresh20 =
                    (*(*this).unk_0C.offset(row as
                                                isize).offset((col *
                                                                   ((*this).row
                                                                        +
                                                                        1 as
                                                                            libc::c_int))
                                                                  as
                                                                  isize)).unk_0;
                *fresh20 -= temp_f00 / phi_f14;
                let ref mut fresh21 =
                    (*(*this).unk_0C.offset(row as
                                                isize).offset((col *
                                                                   ((*this).row
                                                                        +
                                                                        1 as
                                                                            libc::c_int))
                                                                  as
                                                                  isize)).unk_4;
                *fresh21 -= temp_f12 / phi_f14
            }
            row += 1
        }
        col += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800B23E8(mut this: *mut TransitionUnk) { }
#[no_mangle]
pub unsafe extern "C" fn func_800B23F0(mut this: *mut TransitionUnk) -> s32 {
    return 0 as libc::c_int;
}
