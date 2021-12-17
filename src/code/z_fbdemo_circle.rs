#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn Gfx_BranchTexScroll(gfxp: *mut *mut Gfx, x: u32_0, y: u32_0,
                           width: s32, height: s32) -> *mut Gfx;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn guScale(m: *mut Mtx, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn guPerspective(m: *mut Mtx, perspNorm: *mut u16_0, fovy: f32_0,
                     aspect: f32_0, near: f32_0, far: f32_0, scale: f32_0);
    #[no_mangle]
    fn guLookAt(_: *mut Mtx, xEye: f32_0, yEye: f32_0, zEye: f32_0,
                xAt: f32_0, yAt: f32_0, zAt: f32_0, xUp: f32_0, yUp: f32_0,
                zUp: f32_0);
    #[no_mangle]
    fn guRotate(_: *mut Mtx, angle: f32_0, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn guTranslate(m: *mut Mtx, x: f32_0, y: f32_0, z: f32_0);
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
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
pub struct Vec3f {
    pub x: f32_0,
    pub y: f32_0,
    pub z: f32_0,
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
pub struct TransitionCircle {
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub texX: s32,
    pub texY: s32,
    pub step: s32,
    pub unk_14: u8_0,
    pub typeColor: u8_0,
    pub speed: u8_0,
    pub effect: u8_0,
    pub isDone: u8_0,
    pub frame: u8_0,
    pub normal: u16_0,
    pub projection: Mtx,
    pub lookAt: Mtx,
    pub texture: *mut libc::c_void,
    pub modelView: [[Mtx; 3]; 2],
}
// unused
#[no_mangle]
pub static mut sCircleNullDList: [Gfx; 1] =
    [Gfx{words:
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
pub static mut sCircleWipeDefaultTex: [u64_0; 128] =
    [0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x101010101010101 as libc::c_longlong as u64_0,
     0x101010101010101 as libc::c_longlong as u64_0,
     0xa0a0a0a0a0a0a0a as libc::c_longlong as u64_0,
     0xa0a0a0a0a0a0a0a as libc::c_longlong as u64_0,
     0x1313131313131313 as libc::c_longlong as u64_0,
     0x1313131313131313 as libc::c_longlong as u64_0,
     0x1c1c1c1c1c1c1c1c as libc::c_longlong as u64_0,
     0x1c1c1c1c1c1c1c1c as libc::c_longlong as u64_0,
     0x2525252525252525 as libc::c_longlong as u64_0,
     0x2525252525252525 as libc::c_longlong as u64_0,
     0x2e2e2e2e2e2e2e2e as libc::c_longlong as u64_0,
     0x2e2e2e2e2e2e2e2e as libc::c_longlong as u64_0,
     0x3838383838383838 as libc::c_longlong as u64_0,
     0x3838383838383838 as libc::c_longlong as u64_0,
     0x4040404040404040 as libc::c_longlong as u64_0,
     0x4040404040404040 as libc::c_longlong as u64_0,
     0x4a4a4a4a4a4a4a4a as libc::c_longlong as u64_0,
     0x4a4a4a4a4a4a4a4a as libc::c_longlong as u64_0,
     0x5353535353535353 as libc::c_longlong as u64_0,
     0x5353535353535353 as libc::c_longlong as u64_0,
     0x5c5c5c5c5c5c5c5c as libc::c_longlong as u64_0,
     0x5c5c5c5c5c5c5c5c as libc::c_longlong as u64_0,
     0x6565656565656565 as libc::c_longlong as u64_0,
     0x6565656565656565 as libc::c_longlong as u64_0,
     0x6e6e6e6e6e6e6e6e as libc::c_longlong as u64_0,
     0x6e6e6e6e6e6e6e6e as libc::c_longlong as u64_0,
     0x7777777777777777 as libc::c_longlong as u64_0,
     0x7777777777777777 as libc::c_longlong as u64_0,
     0x7f7f7f7f7f7f7f7f as libc::c_longlong as u64_0,
     0x7f7f7f7f7f7f7f7f as libc::c_longlong as u64_0,
     0x8989898989898989 as libc::c_ulonglong,
     0x8989898989898989 as libc::c_ulonglong,
     0x9292929292929292 as libc::c_ulonglong,
     0x9292929292929292 as libc::c_ulonglong,
     0x9b9b9b9b9b9b9b9b as libc::c_ulonglong,
     0x9b9b9b9b9b9b9b9b as libc::c_ulonglong,
     0xa4a4a4a4a4a4a4a4 as libc::c_ulonglong,
     0xa4a4a4a4a4a4a4a4 as libc::c_ulonglong,
     0xadadadadadadadad as libc::c_ulonglong,
     0xadadadadadadadad as libc::c_ulonglong,
     0xb7b7b7b7b7b7b7b7 as libc::c_ulonglong,
     0xb7b7b7b7b7b7b7b7 as libc::c_ulonglong,
     0xbfbfbfbfbfbfbfbf as libc::c_ulonglong,
     0xbfbfbfbfbfbfbfbf as libc::c_ulonglong,
     0xc9c9c9c9c9c9c9c9 as libc::c_ulonglong,
     0xc9c9c9c9c9c9c9c9 as libc::c_ulonglong,
     0xd2d2d2d2d2d2d2d2 as libc::c_ulonglong,
     0xd2d2d2d2d2d2d2d2 as libc::c_ulonglong,
     0xdbdbdbdbdbdbdbdb as libc::c_ulonglong,
     0xdbdbdbdbdbdbdbdb as libc::c_ulonglong,
     0xe4e4e4e4e4e4e4e4 as libc::c_ulonglong,
     0xe4e4e4e4e4e4e4e4 as libc::c_ulonglong,
     0xedededededededed as libc::c_ulonglong,
     0xedededededededed as libc::c_ulonglong,
     0xf7f7f7f7f7f7f7f7 as libc::c_ulonglong,
     0xf7f7f7f7f7f7f7f7 as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong];
#[no_mangle]
pub static mut sCircleWipeStarburstTex: [u64_0; 128] =
    [0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x302000000000000 as libc::c_longlong as u64_0,
     0x2000000 as libc::c_int as u64_0,
     0x1108020100000003 as libc::c_longlong as u64_0,
     0x100000c000000 as libc::c_longlong as u64_0,
     0x140a020100000006 as libc::c_longlong as u64_0,
     0x300000f000001 as libc::c_longlong as u64_0,
     0x190c040200000009 as libc::c_longlong as u64_0,
     0x5000014000003 as libc::c_longlong as u64_0,
     0x1e0d05040000010b as libc::c_longlong as u64_0,
     0x800001a000206 as libc::c_longlong as u64_0,
     0x231007070000010e as libc::c_longlong as u64_0,
     0xb000021000309 as libc::c_longlong as u64_0,
     0x2915090900000213 as libc::c_longlong as u64_0,
     0x1100002800050e as libc::c_longlong as u64_0,
     0x2e180b0a00010416 as libc::c_longlong as u64_0,
     0x11600012f000712 as libc::c_longlong as u64_0,
     0x351d0f0c0002051b as libc::c_longlong as u64_0,
     0x21c000238000a18 as libc::c_longlong as u64_0,
     0x3d23121001020821 as libc::c_longlong as u64_0,
     0x220000340000d1e as libc::c_longlong as u64_0,
     0x4729151504040d27 as libc::c_longlong as u64_0,
     0x42801044a001125 as libc::c_longlong as u64_0,
     0x55351f1f0d0d1733 as libc::c_longlong as u64_0,
     0xd36080c58051b2f as libc::c_longlong as u64_0,
     0x66422d2e181a2643 as libc::c_longlong as u64_0,
     0x1a48141768112c3d as libc::c_longlong as u64_0,
     0x7651393c24283754 as libc::c_longlong as u64_0,
     0x285c2023741c3d49 as libc::c_longlong as u64_0,
     0x8561444930384764 as libc::c_ulonglong,
     0x386f2e318029505b as libc::c_longlong as u64_0,
     0x92714e583c475774 as libc::c_ulonglong,
     0x497c3d3e8e35636e as libc::c_longlong as u64_0,
     0x9d815b6949586b82 as libc::c_ulonglong,
     0x5d894f4a9c437681 as libc::c_longlong as u64_0,
     0xa991677956698190 as libc::c_ulonglong,
     0x729a6159a7518b93 as libc::c_longlong as u64_0,
     0xb4a27388657c969f as libc::c_ulonglong,
     0x88a97466b260a2a3 as libc::c_ulonglong,
     0xbfb07e96768ea9af as libc::c_ulonglong,
     0x9eb78474bc71b7b0 as libc::c_ulonglong,
     0xcac08da889a1babc as libc::c_ulonglong,
     0xb2c69285c682c9bc as libc::c_ulonglong,
     0xd3ce9fb69eb3c9c9 as libc::c_ulonglong,
     0xc5d39e99d094d8c8 as libc::c_ulonglong,
     0xdbd8b1c7b3c5d6d2 as libc::c_ulonglong,
     0xd1deafacdba8e4d4 as libc::c_ulonglong,
     0xe2dfc4d8c8d4dfdb as libc::c_ulonglong,
     0xdbe4c4bfe5bbece0 as libc::c_ulonglong,
     0xe7e5d9e8dae1e9e7 as libc::c_ulonglong,
     0xe6ecd9d4efcff1e9 as libc::c_ulonglong,
     0xedeaebf2e6e8f2f1 as libc::c_ulonglong,
     0xeef4ebe6f7ddf6f0 as libc::c_ulonglong,
     0xf5f3f6f6f4f0f8f5 as libc::c_ulonglong,
     0xf2faf7f1fbe6fbf6 as libc::c_ulonglong,
     0xfcfdfcfbfdfbfbfb as libc::c_ulonglong,
     0xfafefdfbfef7fefb as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong];
#[no_mangle]
pub static mut sCircleWipeRippleTex: [u64_0; 128] =
    [0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x202020202020202 as libc::c_longlong as u64_0,
     0x202020202020202 as libc::c_longlong as u64_0,
     0x1010101010101010 as libc::c_longlong as u64_0,
     0x1010101010101010 as libc::c_longlong as u64_0,
     0x1010101010101010 as libc::c_longlong as u64_0,
     0x1010101010101010 as libc::c_longlong as u64_0,
     0x2929292929292929 as libc::c_longlong as u64_0,
     0x2929292929292929 as libc::c_longlong as u64_0,
     0x4a4a4a4a4a4a4a4a as libc::c_longlong as u64_0,
     0x4a4a4a4a4a4a4a4a as libc::c_longlong as u64_0,
     0x3737373737373737 as libc::c_longlong as u64_0,
     0x3737373737373737 as libc::c_longlong as u64_0,
     0x3838383838383838 as libc::c_longlong as u64_0,
     0x3838383838383838 as libc::c_longlong as u64_0,
     0x7171717171717171 as libc::c_longlong as u64_0,
     0x7171717171717171 as libc::c_longlong as u64_0,
     0x7272727272727272 as libc::c_longlong as u64_0,
     0x7272727272727272 as libc::c_longlong as u64_0,
     0x4a4a4a4a4a4a4a4a as libc::c_longlong as u64_0,
     0x4a4a4a4a4a4a4a4a as libc::c_longlong as u64_0,
     0x7a7a7a7a7a7a7a7a as libc::c_longlong as u64_0,
     0x7a7a7a7a7a7a7a7a as libc::c_longlong as u64_0,
     0xacacacacacacacac as libc::c_ulonglong,
     0xacacacacacacacac as libc::c_ulonglong,
     0x7d7d7d7d7d7d7d7d as libc::c_longlong as u64_0,
     0x7d7d7d7d7d7d7d7d as libc::c_longlong as u64_0,
     0x7777777777777777 as libc::c_longlong as u64_0,
     0x7777777777777777 as libc::c_longlong as u64_0,
     0xc0c0c0c0c0c0c0c0 as libc::c_ulonglong,
     0xc0c0c0c0c0c0c0c0 as libc::c_ulonglong,
     0xbabababababababa as libc::c_ulonglong,
     0xbabababababababa as libc::c_ulonglong,
     0x8888888888888888 as libc::c_ulonglong,
     0x8888888888888888 as libc::c_ulonglong,
     0xb9b9b9b9b9b9b9b9 as libc::c_ulonglong,
     0xb9b9b9b9b9b9b9b9 as libc::c_ulonglong,
     0xe8e8e8e8e8e8e8e8 as libc::c_ulonglong,
     0xe8e8e8e8e8e8e8e8 as libc::c_ulonglong,
     0xbbbbbbbbbbbbbbbb as libc::c_ulonglong,
     0xbbbbbbbbbbbbbbbb as libc::c_ulonglong,
     0xb6b6b6b6b6b6b6b6 as libc::c_ulonglong,
     0xb6b6b6b6b6b6b6b6 as libc::c_ulonglong,
     0xefefefefefefefef as libc::c_ulonglong,
     0xefefefefefefefef as libc::c_ulonglong,
     0xeaeaeaeaeaeaeaea as libc::c_ulonglong,
     0xeaeaeaeaeaeaeaea as libc::c_ulonglong,
     0xcccccccccccccccc as libc::c_ulonglong,
     0xcccccccccccccccc as libc::c_ulonglong,
     0xeaeaeaeaeaeaeaea as libc::c_ulonglong,
     0xeaeaeaeaeaeaeaea as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xf2f2f2f2f2f2f2f2 as libc::c_ulonglong,
     0xf2f2f2f2f2f2f2f2 as libc::c_ulonglong,
     0xf6f6f6f6f6f6f6f6 as libc::c_ulonglong,
     0xf6f6f6f6f6f6f6f6 as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong];
#[no_mangle]
pub static mut sCircleWipeWaveTex: [u64_0; 128] =
    [0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x101010101000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0,
     0x1717171712020000 as libc::c_longlong as u64_0,
     0x30808080708 as libc::c_longlong as u64_0,
     0x181818181b150706 as libc::c_longlong as u64_0,
     0x606060807070b15 as libc::c_longlong as u64_0,
     0x181818181d1f1717 as libc::c_longlong as u64_0,
     0x1a1710070a111a20 as libc::c_longlong as u64_0,
     0x1717171b20201c18 as libc::c_longlong as u64_0,
     0x1817170b19202120 as libc::c_longlong as u64_0,
     0x21211f1f1f201c18 as libc::c_longlong as u64_0,
     0x18212c2b23202020 as libc::c_longlong as u64_0,
     0x3535343026243437 as libc::c_longlong as u64_0,
     0x3135393b30201f1f as libc::c_longlong as u64_0,
     0x393939393c3d4142 as libc::c_longlong as u64_0,
     0x3f39383840403d39 as libc::c_longlong as u64_0,
     0x38393e4347444040 as libc::c_longlong as u64_0,
     0x4039384148494d4f as libc::c_longlong as u64_0,
     0x4041474847433e3d as libc::c_longlong as u64_0,
     0x3d3d4147464a5050 as libc::c_longlong as u64_0,
     0x59544c464950504c as libc::c_longlong as u64_0,
     0x4e535151504f4f4f as libc::c_longlong as u64_0,
     0x6a6961595f676b69 as libc::c_longlong as u64_0,
     0x6a6b6460625b5757 as libc::c_longlong as u64_0,
     0x6868696e716a6868 as libc::c_longlong as u64_0,
     0x686868615f666c6c as libc::c_longlong as u64_0,
     0x68686970706c6868 as libc::c_longlong as u64_0,
     0x68686864666f7171 as libc::c_longlong as u64_0,
     0x67676b6f6f6e6b68 as libc::c_longlong as u64_0,
     0x686767656d707070 as libc::c_longlong as u64_0,
     0x6c777e7774757f7d as libc::c_longlong as u64_0,
     0x796b6a6a706d6e6e as libc::c_longlong as u64_0,
     0x8e98989895969898 as libc::c_ulonglong,
     0x988d8d8c897d7a78 as libc::c_ulonglong,
     0x9c96979797979797 as libc::c_ulonglong,
     0x9798a6a9aaa9a4a2 as libc::c_ulonglong,
     0xaa9c969796969596 as libc::c_ulonglong,
     0x989ea7a7a7a7abb0 as libc::c_ulonglong,
     0xb0ac9e969ca5a7a5 as libc::c_ulonglong,
     0xa4aca6a7a7a7acaf as libc::c_ulonglong,
     0xafb0b1a8acb0b1b0 as libc::c_ulonglong,
     0xb3bdb0a7a7a8afaf as libc::c_ulonglong,
     0xb4b9bdc2bbafafb0 as libc::c_ulonglong,
     0xbdc0beb2a8aaaeae as libc::c_ulonglong,
     0xbfbfc1c7c7bcafb7 as libc::c_ulonglong,
     0xbebfbfc0c6bdb3b1 as libc::c_ulonglong,
     0xbebec5c7c7c7c4c8 as libc::c_ulonglong,
     0xc2bec1ced8d8d3d0 as libc::c_ulonglong,
     0xc3c6c3c5c5d0e0df as libc::c_ulonglong,
     0xd6cbd0d8d7d7dce0 as libc::c_ulonglong,
     0xe2e3dad1cedddfdf as libc::c_ulonglong,
     0xe0ded7d5d6d6dedf as libc::c_ulonglong,
     0xf2f1f2ebe6e1dfdd as libc::c_ulonglong,
     0xdfe8ebe3dfdfe1de as libc::c_ulonglong,
     0xf2efeff0eeeeeff1 as libc::c_ulonglong,
     0xeff6f8f5f4f4f3ed as libc::c_ulonglong,
     0xf7f3f1f0f3f8f8f9 as libc::c_ulonglong,
     0xf8f8f8f8f8f8f8f8 as libc::c_ulonglong,
     0xfffffefeffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong];
#[no_mangle]
pub static mut sCircleWipeVtx: [Vtx; 34] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(25 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                               [-(23 as libc::c_int) as libc::c_short,
                                10 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [384 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [192 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(18 as libc::c_int) as libc::c_short,
                                18 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [576 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(10 as libc::c_int) as libc::c_short,
                                23 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1152 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [960 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                                25 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1344 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [10 as libc::c_int as libc::c_short,
                                23 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1920 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1728 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [18 as libc::c_int as libc::c_short,
                                18 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2304 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2112 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [23 as libc::c_int as libc::c_short,
                                10 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2688 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2496 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [25 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [3072 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2880 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [23 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [3456 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [3264 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [18 as libc::c_int as libc::c_short,
                                -(18 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [3840 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [3648 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [10 as libc::c_int as libc::c_short,
                                -(23 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4224 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4032 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                                -(25 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4608 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4416 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(10 as libc::c_int) as libc::c_short,
                                -(23 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4992 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4800 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(18 as libc::c_int) as libc::c_short,
                                -(18 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [5376 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [5184 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(23 as libc::c_int) as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [5760 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [5568 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(23 as libc::c_int) as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [5760 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                               [-(25 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [6144 as libc::c_int as libc::c_short,
                                4096 as libc::c_int as libc::c_short],
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
                                0 as libc::c_int as libc::c_short,
                                -(10 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [5952 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut sCircleDList: [Gfx; 26] =
    [Gfx{words: Gwords{w0: 0, w1: 0,},}; 26];
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_Start(mut thisx:
                                                    *mut libc::c_void) {
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    (*this).isDone = 0 as libc::c_int as u8_0;
    match (*this).effect as libc::c_int {
        1 => {
            (*this).texture =
                sCircleWipeWaveTex.as_mut_ptr() as *mut libc::c_void
        }
        2 => {
            (*this).texture =
                sCircleWipeRippleTex.as_mut_ptr() as *mut libc::c_void
        }
        3 => {
            (*this).texture =
                sCircleWipeStarburstTex.as_mut_ptr() as *mut libc::c_void
        }
        _ => {
            (*this).texture =
                sCircleWipeDefaultTex.as_mut_ptr() as *mut libc::c_void
        }
    }
    if (*this).speed as libc::c_int == 0 as libc::c_int {
        (*this).step = 0x14 as libc::c_int
    } else { (*this).step = 0xa as libc::c_int }
    if (*this).typeColor as libc::c_int == 0 as libc::c_int {
        (*this).color.rgba =
            ((0 as libc::c_int & 0xff as libc::c_int) << 24 as libc::c_int |
                 (0 as libc::c_int & 0xff as libc::c_int) << 16 as libc::c_int
                 |
                 (0 as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
                 |
                 (255 as libc::c_int & 0xff as libc::c_int) <<
                     0 as libc::c_int) as u32_0
    } else if (*this).typeColor as libc::c_int == 1 as libc::c_int {
        (*this).color.rgba =
            ((160 as libc::c_int & 0xff as libc::c_int) << 24 as libc::c_int |
                 (160 as libc::c_int & 0xff as libc::c_int) <<
                     16 as libc::c_int |
                 (160 as libc::c_int & 0xff as libc::c_int) <<
                     8 as libc::c_int |
                 (255 as libc::c_int & 0xff as libc::c_int) <<
                     0 as libc::c_int) as u32_0
    } else if (*this).typeColor as libc::c_int == 2 as libc::c_int {
        // yes, really.
        (*this).color.c2rust_unnamed.r = 100 as libc::c_int as u8_0;
        (*this).color.c2rust_unnamed.g = 100 as libc::c_int as u8_0;
        (*this).color.c2rust_unnamed.b = 100 as libc::c_int as u8_0;
        (*this).color.c2rust_unnamed.a = 255 as libc::c_int as u8_0
    } else {
        (*this).step = 0x28 as libc::c_int;
        (*this).color.rgba =
            if (*this).effect as libc::c_int == 1 as libc::c_int {
                ((0 as libc::c_int & 0xff as libc::c_int) << 24 as libc::c_int
                     |
                     (0 as libc::c_int & 0xff as libc::c_int) <<
                         16 as libc::c_int |
                     (0 as libc::c_int & 0xff as libc::c_int) <<
                         8 as libc::c_int) |
                    (255 as libc::c_int & 0xff as libc::c_int) <<
                        0 as libc::c_int
            } else {
                ((160 as libc::c_int & 0xff as libc::c_int) <<
                     24 as libc::c_int |
                     (160 as libc::c_int & 0xff as libc::c_int) <<
                         16 as libc::c_int |
                     (160 as libc::c_int & 0xff as libc::c_int) <<
                         8 as libc::c_int) |
                    (255 as libc::c_int & 0xff as libc::c_int) <<
                        0 as libc::c_int
            } as u32_0
    }
    if (*this).unk_14 as libc::c_int != 0 as libc::c_int {
        (*this).texY = 0 as libc::c_int;
        if (*this).typeColor as libc::c_int == 3 as libc::c_int {
            (*this).texY = 0xfa as libc::c_int
        }
    } else {
        (*this).texY = 0x1f4 as libc::c_int;
        if (*this).effect as libc::c_int == 2 as libc::c_int {
            Audio_PlaySoundGeneral(0x5804 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
        }
    }
    guPerspective(&mut (*this).projection, &mut (*this).normal, 60.0f32,
                  4.0f32 / 3.0f32, 10.0f32, 12800.0f32, 1.0f32);
    guLookAt(&mut (*this).lookAt, 0.0f32, 0.0f32, 400.0f32, 0.0f32, 0.0f32,
             0.0f32, 0.0f32, 1.0f32, 0.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_Init(mut thisx: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    bzero(this as *mut libc::c_void,
          ::std::mem::size_of::<TransitionCircle>() as libc::c_ulong);
    return this as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_Destroy(mut thisx:
                                                      *mut libc::c_void) {
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_Update(mut thisx: *mut libc::c_void,
                                                 mut updateRate: s32) {
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    let mut temp_t2: s32 = 0;
    let mut temp_t3: s32 = 0;
    if (*this).unk_14 as libc::c_int != 0 as libc::c_int {
        if (*this).texY == 0 as libc::c_int {
            if (*this).effect as libc::c_int == 2 as libc::c_int {
                Audio_PlaySoundGeneral(0x5803 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
        }
        (*this).texY += (*this).step * 3 as libc::c_int / updateRate;
        if (*this).texY >= 0x1f4 as libc::c_int {
            (*this).texY = 0x1f4 as libc::c_int;
            (*this).isDone = 1 as libc::c_int as u8_0
        }
    } else {
        (*this).texY -= (*this).step * 3 as libc::c_int / updateRate;
        if (*this).typeColor as libc::c_int != 3 as libc::c_int {
            if (*this).texY <= 0 as libc::c_int {
                (*this).texY = 0 as libc::c_int;
                (*this).isDone = 1 as libc::c_int as u8_0
            }
        } else if (*this).texY < 0xfb as libc::c_int {
            (*this).texY = 0xfa as libc::c_int;
            (*this).isDone = 1 as libc::c_int as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_Draw(mut thisx: *mut libc::c_void,
                                               mut gfxP: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = *gfxP;
    let mut modelView: *mut Mtx = 0 as *mut Mtx;
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    let mut texScroll: *mut Gfx = 0 as *mut Gfx;
    // These variables are a best guess based on the other transition types.
    let mut tPos: f32_0 = 0.0f32;
    let mut rot: f32_0 = 0.0f32;
    let mut scale: f32_0 = 14.8f32;
    modelView = (*this).modelView[(*this).frame as usize].as_mut_ptr();
    (*this).frame = ((*this).frame as libc::c_int ^ 1 as libc::c_int) as u8_0;
    let fresh0 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    texScroll =
        Gfx_BranchTexScroll(&mut gfx, (*this).texX as u32_0,
                            (*this).texY as u32_0, 0x10 as libc::c_int,
                            0x40 as libc::c_int);
    let fresh1 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = texScroll as libc::c_uint;
    let fresh2 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = (*this).texture as libc::c_uint;
    let fresh3 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_2).words.w1 = (*this).color.rgba;
    let fresh4 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh4;
    (*_g_3).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_3).words.w1 = (*this).color.rgba;
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
            (((0x4 as libc::c_int | 0x2 as libc::c_int) ^ 0x1 as libc::c_int)
                 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 = &mut (*this).projection as *mut Mtx as libc::c_uint;
    let fresh6 = gfx;
    gfx = gfx.offset(1);
    let mut _g_5: *mut Gfx = fresh6;
    (*_g_5).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xe as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 = (*this).normal as libc::c_uint;
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
            (((0x4 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 = &mut (*this).lookAt as *mut Mtx as libc::c_uint;
    if scale != 1.0f32 {
        guScale(&mut *modelView.offset(0 as libc::c_int as isize), scale,
                scale, 1.0f32);
        let fresh8 = gfx;
        gfx = gfx.offset(1);
        let mut _g_7: *mut Gfx = fresh8;
        (*_g_7).words.w0 =
            (0xda as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((::std::mem::size_of::<Mtx>() as
                      libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint).wrapping_div(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                     &
                     (((0x1 as libc::c_int) << 5 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_7).words.w1 =
            &mut *modelView.offset(0 as libc::c_int as isize) as *mut Mtx as
                libc::c_uint
    }
    if rot != 0.0f32 {
        guRotate(&mut *modelView.offset(1 as libc::c_int as isize), rot,
                 0.0f32, 0.0f32, 1.0f32);
        let fresh9 = gfx;
        gfx = gfx.offset(1);
        let mut _g_8: *mut Gfx = fresh9;
        (*_g_8).words.w0 =
            (0xda as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((::std::mem::size_of::<Mtx>() as
                      libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint).wrapping_div(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                     &
                     (((0x1 as libc::c_int) << 5 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_8).words.w1 =
            &mut *modelView.offset(1 as libc::c_int as isize) as *mut Mtx as
                libc::c_uint
    }
    if tPos != 0.0f32 || tPos != 0.0f32 {
        guTranslate(&mut *modelView.offset(2 as libc::c_int as isize), tPos,
                    tPos, 0.0f32);
        let fresh10 = gfx;
        gfx = gfx.offset(1);
        let mut _g_9: *mut Gfx = fresh10;
        (*_g_9).words.w0 =
            (0xda as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((::std::mem::size_of::<Mtx>() as
                      libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint).wrapping_div(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                     &
                     (((0x1 as libc::c_int) << 5 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_9).words.w1 =
            &mut *modelView.offset(2 as libc::c_int as isize) as *mut Mtx as
                libc::c_uint
    }
    let fresh11 = gfx;
    gfx = gfx.offset(1);
    let mut _g_10: *mut Gfx = fresh11;
    (*_g_10).words.w0 =
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
    (*_g_10).words.w1 = sCircleDList.as_mut_ptr() as libc::c_uint;
    let fresh12 = gfx;
    gfx = gfx.offset(1);
    let mut _g_11: *mut Gfx = fresh12;
    (*_g_11).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_11).words.w1 = 0 as libc::c_int as libc::c_uint;
    *gfxP = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_IsDone(mut thisx: *mut libc::c_void)
 -> s32 {
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    return (*this).isDone as s32;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_SetType(mut thisx:
                                                      *mut libc::c_void,
                                                  mut type_0: s32) {
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    if type_0 & 0x80 as libc::c_int != 0 {
        (*this).unk_14 =
            (type_0 >> 5 as libc::c_int & 0x1 as libc::c_int) as u8_0;
        (*this).typeColor =
            (type_0 >> 3 as libc::c_int & 0x3 as libc::c_int) as u8_0;
        (*this).speed = (type_0 & 0x1 as libc::c_int) as u8_0;
        (*this).effect =
            (type_0 >> 1 as libc::c_int & 0x3 as libc::c_int) as u8_0
    } else if type_0 == 1 as libc::c_int {
        (*this).unk_14 = 1 as libc::c_int as u8_0
    } else { (*this).unk_14 = 0 as libc::c_int as u8_0 };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_SetColor(mut thisx:
                                                       *mut libc::c_void,
                                                   mut color: u32_0) {
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    (*this).color.rgba = color;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionCircle_SetEnvColor(mut thisx:
                                                          *mut libc::c_void,
                                                      mut envColor: u32_0) {
    let mut this: *mut TransitionCircle = thisx as *mut TransitionCircle;
    (*this).envColor.rgba = envColor;
}
unsafe extern "C" fn run_static_initializers() {
    sCircleDList =
        [Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd9 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (!((0x1 as libc::c_int |
                                                0x4 as libc::c_int |
                                                0x600 as libc::c_int |
                                                0x10000 as libc::c_int |
                                                0x20000 as libc::c_int |
                                                0x40000 as libc::c_int |
                                                0x80000 as libc::c_int |
                                                0x100000 as libc::c_int |
                                                0x200000 as libc::c_int) as
                                               u32_0) &
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
                                    (0xd9 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (!(0 as libc::c_int as u32_0) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0x4 as libc::c_int |
                                         0x200000 as libc::c_int) as u32_0,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xef as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((3 as libc::c_int) <<
                                              4 as libc::c_int |
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
                                              (0 as libc::c_int) <<
                                                  23 as libc::c_int) as u32_0
                                             &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ((0 as libc::c_int) << 0 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             2 as libc::c_int |
                                         0x40 as libc::c_int |
                                         0x200 as libc::c_int |
                                         0x4000 as libc::c_int |
                                         0 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             30 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             26 as libc::c_int |
                                         (1 as libc::c_int) <<
                                             22 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             18 as libc::c_int |
                                         0x40 as libc::c_int |
                                         0x200 as libc::c_int |
                                         0x4000 as libc::c_int |
                                         0 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             28 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             24 as libc::c_int |
                                         (1 as libc::c_int) <<
                                             20 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             16 as libc::c_int) as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfc as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((3 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     4 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              20 as libc::c_int |
                                              (1 as libc::c_int as u32_0 &
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
                                              ((3 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          4 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   5 as libc::c_int |
                                                   (1 as libc::c_int as u32_0
                                                        &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              5 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 0 as libc::c_int)) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               4 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        28 as libc::c_int |
                                        (5 as libc::c_int as u32_0 &
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
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        ((5 as libc::c_int as u32_0 &
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
                                             (5 as libc::c_int as u32_0 &
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
                                             (1 as libc::c_int as u32_0 &
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
                                    (0xd7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
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
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    (0xffff as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        (0xffff as libc::c_int as u32_0 &
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
                                    (0xfd as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
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
                                        ((1 as libc::c_int - 1 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    0x8000000 as libc::c_int as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
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
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((if ((16 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (16 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int
                                          } else { 2047 as libc::c_int }) as
                                             u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((((1 as libc::c_int) <<
                                                11 as libc::c_int) +
                                               (if 1 as libc::c_int >
                                                       16 as libc::c_int *
                                                           1 as libc::c_int /
                                                           8 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (16 as libc::c_int *
                                                         1 as libc::c_int) /
                                                        8 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      16 as libc::c_int *
                                                          1 as libc::c_int /
                                                          8 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (16 as libc::c_int *
                                                        1 as libc::c_int) /
                                                       8 as libc::c_int
                                               })) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((16 as libc::c_int * 1 as libc::c_int
                                              + 7 as libc::c_int >>
                                              3 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((16 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((64 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xde as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    0x9000000 as libc::c_int as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (32 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int +
                                              32 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    sCircleWipeVtx.as_mut_ptr() as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((0 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((1 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((2 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((1 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((2 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((0 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((2 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((0 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((1 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((1 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((3 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((4 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((3 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((4 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((1 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((4 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((1 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((3 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((3 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((5 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((6 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((5 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((6 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((3 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((6 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((3 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((5 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((5 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((7 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((8 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((7 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((8 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((5 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((8 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((5 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
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
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((7 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((9 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((10 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((9 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((10 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((7 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((10 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((7 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((9 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((9 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((11 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((12 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((11 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((12 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((9 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((12 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((9 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((11 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((11 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((13 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((14 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((13 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((14 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((11 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((14 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((11 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((13 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((13 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((15 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((16 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((15 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((16 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((13 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((16 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((13 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((15 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((15 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((17 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((18 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((17 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((18 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((15 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((18 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((15 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((17 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((17 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((19 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((20 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((19 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((20 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((17 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((20 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((17 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((19 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((19 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((21 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((22 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((21 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((22 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((19 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((22 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((19 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((21 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((21 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((23 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((24 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((23 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((24 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((21 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((24 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((21 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((23 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((23 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((25 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((26 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((25 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((26 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((23 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((26 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((23 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((25 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((25 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((27 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((28 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((27 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((28 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((25 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((28 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((25 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((27 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((27 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((29 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((30 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((29 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((30 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((27 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((30 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((27 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((29 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (3 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int + 3 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    &mut *sCircleWipeVtx.as_mut_ptr().offset(31
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
                                        as *mut Vtx as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((0 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((1 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((2 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((1 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((2 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((0 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((2 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((0 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((1 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
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
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },}]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
