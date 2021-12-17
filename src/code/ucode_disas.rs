#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn MtxConv_L2F(m1: *mut MtxF, m2: *mut Mtx);
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
}
pub type s8 = libc::c_schar;
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
pub type MtxF_t = [[libc::c_float; 4]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub union MtxF {
    pub mf: MtxF_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub xx: libc::c_float,
    pub yx: libc::c_float,
    pub zx: libc::c_float,
    pub wx: libc::c_float,
    pub xy: libc::c_float,
    pub yy: libc::c_float,
    pub zy: libc::c_float,
    pub wy: libc::c_float,
    pub xz: libc::c_float,
    pub yz: libc::c_float,
    pub zz: libc::c_float,
    pub wz: libc::c_float,
    pub xw: libc::c_float,
    pub yw: libc::c_float,
    pub zw: libc::c_float,
    pub ww: libc::c_float,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp_t {
    pub vscale: [libc::c_short; 4],
    pub vtrans: [libc::c_short; 4],
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gtexrect {
    #[bitfield(name = "cmd", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "xl", ty = "libc::c_uint", bits = "8..=19")]
    #[bitfield(name = "yl", ty = "libc::c_uint", bits = "20..=31")]
    #[bitfield(name = "pad1", ty = "libc::c_uint", bits = "32..=36")]
    #[bitfield(name = "tile", ty = "libc::c_uint", bits = "37..=39")]
    #[bitfield(name = "xh", ty = "libc::c_uint", bits = "40..=51")]
    #[bitfield(name = "yh", ty = "libc::c_uint", bits = "52..=63")]
    #[bitfield(name = "s", ty = "libc::c_uint", bits = "64..=79")]
    #[bitfield(name = "t", ty = "libc::c_uint", bits = "80..=95")]
    #[bitfield(name = "dsdx", ty = "libc::c_uint", bits = "96..=111")]
    #[bitfield(name = "dtdy", ty = "libc::c_uint", bits = "112..=127")]
    pub cmd_xl_yl_pad1_tile_xh_yh_s_t_dsdx_dtdy: [u8; 16],
}
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
pub struct UCodeInfo {
    pub type_0: u32_0,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UCodeDisas {
    pub segments: [u32_0; 16],
    pub dlStack: [*mut Gfx; 18],
    pub dlDepth: s32,
    pub dlCnt: u32_0,
    pub vtxCnt: u32_0,
    pub spvtxCnt: u32_0,
    pub tri1Cnt: u32_0,
    pub tri2Cnt: u32_0,
    pub quadCnt: u32_0,
    pub lineCnt: u32_0,
    pub loaducodeCnt: u32_0,
    pub pipeSyncRequired: u32_0,
    pub tileSyncRequired: u32_0,
    pub loadSyncRequired: u32_0,
    pub syncErr: u32_0,
    pub enableLog: s32,
    pub ucodeType: s32,
    pub ucodeInfoCount: s32,
    pub ucodeInfo: *mut UCodeInfo,
    pub modeH: u32_0,
    pub modeL: u32_0,
    pub geometryMode: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F3dzexConst {
    pub value: u32_0,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F3dzexFlag {
    pub value: u32_0,
    pub setName: *const libc::c_char,
    pub unsetName: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F3dzexRenderMode {
    pub name: *const libc::c_char,
    pub value: u32_0,
    pub mask: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F3dzexSetModeMacroValue {
    pub name: *const libc::c_char,
    pub value: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F3dzexSetModeMacro {
    pub name: *const libc::c_char,
    pub shift: u32_0,
    pub len: u32_0,
    pub values: [F3dzexSetModeMacroValue; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gline3DFix {
    pub cmd: s8,
    pub v0: u8_0,
    pub v1: u8_0,
    pub wd: u8_0,
    pub pad: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gvtx {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "u32_0", bits = "8..=11")]
    #[bitfield(name = "numv", ty = "u32_0", bits = "12..=19")]
    #[bitfield(name = "pad2", ty = "s32", bits = "20..=23")]
    pub cmd_pad_numv_pad2: [u8; 3],
    pub vbidx: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gtrimod {
    #[bitfield(name = "pad", ty = "u8_0", bits = "0..=7")]
    #[bitfield(name = "v0", ty = "u8_0", bits = "8..=15")]
    #[bitfield(name = "v1", ty = "u8_0", bits = "16..=23")]
    #[bitfield(name = "v2", ty = "u8_0", bits = "24..=31")]
    pub pad_v0_v1_v2: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gtri1 {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad: [u8; 4],
    pub tri: Gtrimod,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gtri2 {
    pub tri1: Gtrimod,
    pub tri2: Gtrimod,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gquadmod {
    #[bitfield(name = "pad", ty = "u8_0", bits = "0..=7")]
    #[bitfield(name = "v0", ty = "u8_0", bits = "8..=15")]
    #[bitfield(name = "v1", ty = "u8_0", bits = "16..=23")]
    #[bitfield(name = "v2", ty = "u8_0", bits = "24..=31")]
    #[bitfield(name = "pad1", ty = "u8_0", bits = "32..=39")]
    #[bitfield(name = "pad2", ty = "u8_0", bits = "40..=47")]
    #[bitfield(name = "pad3", ty = "u8_0", bits = "48..=55")]
    #[bitfield(name = "v3", ty = "u8_0", bits = "56..=63")]
    pub pad_v0_v1_v2_pad1_pad2_pad3_v3: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gcull {
    pub pad: u16_0,
    pub vstart: u16_0,
    pub pad2: u16_0,
    pub vend: u16_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetcolorMod {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    pub cmd: [u8; 1],
    pub pad: u8_0,
    pub prim_min_level: u8_0,
    pub prim_level: u8_0,
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gsetprimdepth {
    pub cmd: u8_0,
    pub pad: [libc::c_char; 3],
    pub z: u16_0,
    pub d: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gnoop {
    pub cmd: u8_0,
    pub type_0: u8_0,
    pub len: u16_0,
    pub value: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub u32_0: u32_0,
    pub f32_0: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gmatrix {
    pub cmd: u8_0,
    pub pad: [u8_0; 2],
    pub params: u8_0,
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetcombineMod {
    pub cmd: u8_0,
    #[bitfield(name = "a", ty = "u32_0", bits = "0..=3")]
    #[bitfield(name = "c", ty = "u32_0", bits = "4..=8")]
    #[bitfield(name = "z", ty = "u32_0", bits = "9..=11")]
    #[bitfield(name = "x", ty = "u32_0", bits = "12..=14")]
    #[bitfield(name = "e", ty = "u32_0", bits = "15..=18")]
    #[bitfield(name = "g", ty = "u32_0", bits = "19..=23")]
    #[bitfield(name = "b", ty = "u32_0", bits = "24..=27")]
    #[bitfield(name = "f", ty = "u32_0", bits = "28..=31")]
    #[bitfield(name = "v", ty = "u32_0", bits = "32..=34")]
    #[bitfield(name = "t", ty = "u32_0", bits = "35..=37")]
    #[bitfield(name = "d", ty = "u32_0", bits = "38..=40")]
    #[bitfield(name = "y", ty = "u32_0", bits = "41..=43")]
    #[bitfield(name = "w", ty = "u32_0", bits = "44..=46")]
    #[bitfield(name = "h", ty = "u32_0", bits = "47..=49")]
    #[bitfield(name = "u", ty = "u32_0", bits = "50..=52")]
    #[bitfield(name = "s", ty = "u32_0", bits = "53..=55")]
    pub a_c_z_x_e_g_b_f_v_t_d_y_w_h_u_s: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetothermodeMod {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "sft", ty = "u32_0", bits = "16..=23")]
    #[bitfield(name = "len", ty = "u32_0", bits = "24..=31")]
    #[bitfield(name = "data", ty = "u32_0", bits = "32..=63")]
    pub cmd_pad0_sft_len_data: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gmovewd {
    #[bitfield(name = "cmd", ty = "s32", bits = "0..=7")]
    #[bitfield(name = "offset", ty = "u32_0", bits = "8..=23")]
    #[bitfield(name = "index", ty = "u32_0", bits = "24..=31")]
    pub cmd_offset_index: [u8; 4],
    pub data: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gmovemem {
    #[bitfield(name = "cmd", ty = "s32", bits = "0..=7")]
    #[bitfield(name = "size", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "offset", ty = "u32_0", bits = "16..=23")]
    #[bitfield(name = "index", ty = "u32_0", bits = "24..=31")]
    pub cmd_size_offset_index: [u8; 4],
    pub data: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gtexturemod {
    #[bitfield(name = "cmd", ty = "u8_0", bits = "0..=7")]
    #[bitfield(name = "lodscale", ty = "u8_0", bits = "8..=15")]
    #[bitfield(name = "pad", ty = "u8_0", bits = "16..=17")]
    #[bitfield(name = "level", ty = "u8_0", bits = "18..=20")]
    #[bitfield(name = "tile", ty = "u8_0", bits = "21..=23")]
    pub cmd_lodscale_pad_level_tile: [u8; 3],
    pub on: libc::c_uchar,
    pub s: libc::c_ushort,
    pub t: libc::c_ushort,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gpopmtxmod {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "8..=31")]
    #[bitfield(name = "param", ty = "u32_0", bits = "32..=57")]
    #[bitfield(name = "pad3", ty = "libc::c_uchar", bits = "58..=63")]
    pub cmd_pad1_param_pad3: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union GfxMod {
    pub words: Gwords,
    pub noop: Gnoop,
    pub matrix: Gmatrix,
    pub dma: Gdma,
    pub tri1: Gtri1,
    pub tri2: Gtri2,
    pub quad: Gquadmod,
    pub cull: Gcull,
    pub line: Gline3D,
    pub linefix: Gline3DFix,
    pub movewd: Gmovewd,
    pub movemem: Gmovemem,
    pub popmtx: Gpopmtx,
    pub popmtxmod: Gpopmtxmod,
    pub segment: Gsegment,
    pub setothermodeH: GsetothermodeH,
    pub setothermodeL: GsetothermodeL,
    pub setothermode: GsetothermodeMod,
    pub texture: Gtexture,
    pub texmod: Gtexturemod,
    pub perspnorm: Gperspnorm,
    pub setimg: Gsetimg,
    pub setcombine: GsetcombineMod,
    pub setcolor: GsetcolorMod,
    pub fillrect: Gfillrect,
    pub settile: Gsettile,
    pub loadtile: Gloadtile,
    pub settilesize: Gsettilesize,
    pub loadtlut: Gloadtlut,
    pub setprimdepth: Gsetprimdepth,
    pub vtx: Gvtx,
    pub force_structure_alignment: libc::c_longlong,
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_TranslateAddr(mut this: *mut UCodeDisas,
                                                  mut addr: u32_0) -> u32_0 {
    let mut physical: u32_0 =
        (*this).segments[(addr << 4 as libc::c_int >> 28 as libc::c_int) as
                             usize].wrapping_add(addr &
                                                     0xffffff as libc::c_int
                                                         as
                                                         libc::c_uint); // "Microcode did not match"
    return physical.wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as u32_0;
}
#[no_mangle]
pub static mut sUCodeDisasGeometryModes: [F3dzexConst; 11] =
    [{
         let mut init =
             F3dzexConst{value: 0x1 as libc::c_int as u32_0,
                         name:
                             b"G_ZBUFFER\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0 as libc::c_int as u32_0,
                         name:
                             b"G_TEXTURE_ENABLE\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x4 as libc::c_int as u32_0,
                         name:
                             b"G_SHADE\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x200000 as libc::c_int as u32_0,
                         name:
                             b"G_SHADING_SMOOTH\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x200 as libc::c_int as u32_0,
                         name:
                             b"G_CULL_FRONT\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x400 as libc::c_int as u32_0,
                         name:
                             b"G_CULL_BACK\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x10000 as libc::c_int as u32_0,
                         name:
                             b"G_FOG\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x20000 as libc::c_int as u32_0,
                         name:
                             b"G_LIGHTING\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x40000 as libc::c_int as u32_0,
                         name:
                             b"G_TEXTURE_GEN\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x80000 as libc::c_int as u32_0,
                         name:
                             b"G_TEXTURE_GEN_LINEAR\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexConst{value: 0x100000 as libc::c_int as u32_0,
                         name:
                             b"G_LOD\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     }];
#[no_mangle]
pub static mut sUCodeDisasMtxFlags: [F3dzexFlag; 3] =
    [{
         let mut init =
             F3dzexFlag{value: 0x4 as libc::c_int as u32_0,
                        setName:
                            b"G_MTX_PROJECTION\x00" as *const u8 as
                                *const libc::c_char,
                        unsetName:
                            b"G_MTX_MODELVIEW\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexFlag{value: 0x2 as libc::c_int as u32_0,
                        setName:
                            b"G_MTX_LOAD\x00" as *const u8 as
                                *const libc::c_char,
                        unsetName:
                            b"G_MTX_MUL\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             F3dzexFlag{value: 0x1 as libc::c_int as u32_0,
                        setName:
                            b"G_MTX_PUSH\x00" as *const u8 as
                                *const libc::c_char,
                        unsetName:
                            b"G_MTX_NOPUSH\x00" as *const u8 as
                                *const libc::c_char,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_ParseCombineColor(mut value: u32_0,
                                                      mut idx: u32_0)
 -> *const libc::c_char {
    let mut ret: *const libc::c_char =
        b"?\x00" as *const u8 as *const libc::c_char;
    match value {
        0 => { ret = b"COMBINED\x00" as *const u8 as *const libc::c_char }
        1 => { ret = b"TEXEL0\x00" as *const u8 as *const libc::c_char }
        2 => { ret = b"TEXEL1\x00" as *const u8 as *const libc::c_char }
        3 => { ret = b"PRIMITIVE\x00" as *const u8 as *const libc::c_char }
        4 => { ret = b"SHADE\x00" as *const u8 as *const libc::c_char }
        5 => { ret = b"ENVIRONMENT\x00" as *const u8 as *const libc::c_char }
        6 => {
            ret =
                if idx == 2 as libc::c_int as libc::c_uint {
                    b"CENTER\x00" as *const u8 as *const libc::c_char
                } else if idx == 3 as libc::c_int as libc::c_uint {
                    b"SCALE\x00" as *const u8 as *const libc::c_char
                } else { b"1\x00" as *const u8 as *const libc::c_char }
        }
        7 => {
            ret =
                if idx == 1 as libc::c_int as libc::c_uint {
                    b"NOISE\x00" as *const u8 as *const libc::c_char
                } else if idx == 2 as libc::c_int as libc::c_uint {
                    b"K4\x00" as *const u8 as *const libc::c_char
                } else if idx == 3 as libc::c_int as libc::c_uint {
                    b"COMBINED_ALPHA\x00" as *const u8 as *const libc::c_char
                } else { b"0\x00" as *const u8 as *const libc::c_char }
        }
        _ => {
            if idx == 3 as libc::c_int as libc::c_uint {
                match value {
                    8 => {
                        ret =
                            b"TEXEL0_ALPHA\x00" as *const u8 as
                                *const libc::c_char
                    }
                    9 => {
                        ret =
                            b"TEXEL1_ALPHA\x00" as *const u8 as
                                *const libc::c_char
                    }
                    10 => {
                        ret =
                            b"PRIMITIVE_ALPHA\x00" as *const u8 as
                                *const libc::c_char
                    }
                    11 => {
                        ret =
                            b"SHADE_ALPHA\x00" as *const u8 as
                                *const libc::c_char
                    }
                    12 => {
                        ret =
                            b"ENV_ALPHA\x00" as *const u8 as
                                *const libc::c_char
                    }
                    13 => {
                        ret =
                            b"LOD_FRACTION\x00" as *const u8 as
                                *const libc::c_char
                    }
                    14 => {
                        ret =
                            b"PRIM_LOD_FRAC\x00" as *const u8 as
                                *const libc::c_char
                    }
                    15 => {
                        ret = b"K5\x00" as *const u8 as *const libc::c_char
                    }
                    _ => {
                        ret = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else { ret = b"0\x00" as *const u8 as *const libc::c_char }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_ParseCombineAlpha(mut value: u32_0,
                                                      mut idx: u32_0)
 -> *const libc::c_char {
    let mut ret: *const libc::c_char =
        b"?\x00" as *const u8 as *const libc::c_char;
    match value {
        0 => {
            ret =
                if idx == 3 as libc::c_int as libc::c_uint {
                    b"LOD_FRACTION\x00" as *const u8 as *const libc::c_char
                } else { b"COMBINED\x00" as *const u8 as *const libc::c_char }
        }
        1 => { ret = b"TEXEL0\x00" as *const u8 as *const libc::c_char }
        2 => { ret = b"TEXEL1\x00" as *const u8 as *const libc::c_char }
        3 => { ret = b"PRIMITIVE\x00" as *const u8 as *const libc::c_char }
        4 => { ret = b"SHADE\x00" as *const u8 as *const libc::c_char }
        5 => { ret = b"ENVIRONMENT\x00" as *const u8 as *const libc::c_char }
        6 => {
            ret =
                if idx == 3 as libc::c_int as libc::c_uint {
                    b"PRIM_LOD_FRAC\x00" as *const u8 as *const libc::c_char
                } else { b"1\x00" as *const u8 as *const libc::c_char }
        }
        7 => { ret = b"0\x00" as *const u8 as *const libc::c_char }
        _ => { }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_Init(mut this: *mut UCodeDisas) {
    let mut i: u32_0 = 0;
    bzero(this as *mut libc::c_void,
          ::std::mem::size_of::<UCodeDisas>() as libc::c_ulong);
    i = 0 as libc::c_int as u32_0;
    while i < 16 as libc::c_int as libc::c_uint {
        (*this).segments[i as usize] = gSegments[i as usize];
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_Destroy(mut this: *mut UCodeDisas) { }
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_SetCurUCodeImpl(mut this: *mut UCodeDisas,
                                                    mut ptr:
                                                        *mut libc::c_void) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*this).ucodeInfoCount {
        if ptr == (*(*this).ucodeInfo.offset(i as isize)).ptr {
            (*this).ucodeType =
                (*(*this).ucodeInfo.offset(i as isize)).type_0 as s32;
            break ;
        } else { i += 1 }
    }
    if i >= (*this).ucodeInfoCount {
        if (*this).enableLog != 0 {
            osSyncPrintf(b"\xe3\x83\x9e\xe3\x82\xa4\xe3\x82\xaf\xe3\x83\xad\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x89\xe3\x81\x8c\xe4\xb8\x80\xe8\x87\xb4\xe3\x81\x97\xe3\x81\xaa\xe3\x81\x8b\xe3\x81\xa3\xe3\x81\x9f\n\x00"
                             as *const u8 as *const libc::c_char);
        }
        (*this).ucodeType = 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_ParseGeometryMode(mut this:
                                                          *mut UCodeDisas,
                                                      mut mode: u32_0) {
    let mut first: u32_0 = 1 as libc::c_int as u32_0;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[F3dzexConst; 11]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<F3dzexConst>()
                                                   as libc::c_ulong) as s32 {
        if !(sUCodeDisasGeometryModes[i as usize].value & mode ==
                 0 as libc::c_int as libc::c_uint) {
            if !(first != 0) {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"|\x00" as *const u8 as
                                     *const libc::c_char);
                }
            }
            first = 0 as libc::c_int as u32_0;
            if (*this).enableLog != 0 {
                osSyncPrintf(b"%s\x00" as *const u8 as *const libc::c_char,
                             sUCodeDisasGeometryModes[i as usize].name);
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_ParseRenderMode(mut this: *mut UCodeDisas,
                                                    mut mode: u32_0) {
    static mut sUCodeDisasRenderModeFlags: [F3dzexRenderMode; 16] =
        [{
             let mut init =
                 F3dzexRenderMode{name:
                                      b"AA_EN\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x8 as libc::c_int as u32_0,
                                  mask: 0x8 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"Z_CMP\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x10 as libc::c_int as u32_0,
                                  mask: 0x10 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"Z_UPD\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x20 as libc::c_int as u32_0,
                                  mask: 0x20 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"IM_RD\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x40 as libc::c_int as u32_0,
                                  mask: 0x40 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"CLR_ON_CVG\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x80 as libc::c_int as u32_0,
                                  mask: 0x80 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"CVG_DST_CLAMP\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0 as libc::c_int as u32_0,
                                  mask: 0x300 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"CVG_DST_WRAP\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x100 as libc::c_int as u32_0,
                                  mask: 0x300 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"CVG_DST_FULL\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x200 as libc::c_int as u32_0,
                                  mask: 0x300 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"CVG_DST_SAVE\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x300 as libc::c_int as u32_0,
                                  mask: 0x300 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"ZMODE_OPA\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0 as libc::c_int as u32_0,
                                  mask: 0xc00 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"ZMODE_INTER\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x400 as libc::c_int as u32_0,
                                  mask: 0xc00 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"ZMODE_XLU\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x800 as libc::c_int as u32_0,
                                  mask: 0xc00 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"ZMODE_DEC\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0xc00 as libc::c_int as u32_0,
                                  mask: 0xc00 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"CVG_X_ALPHA\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x1000 as libc::c_int as u32_0,
                                  mask: 0x1000 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"ALPHA_CVG_SEL\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x2000 as libc::c_int as u32_0,
                                  mask: 0x2000 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 F3dzexRenderMode{name:
                                      b"FORCE_BL\x00" as *const u8 as
                                          *const libc::c_char,
                                  value: 0x4000 as libc::c_int as u32_0,
                                  mask: 0x4000 as libc::c_int as u32_0,};
             init
         }];
    static mut D_8012DDDC: [[*const libc::c_char; 4]; 4] =
        [[b"G_BL_CLR_IN\x00" as *const u8 as *const libc::c_char,
          b"G_BL_CLR_MEM\x00" as *const u8 as *const libc::c_char,
          b"G_BL_CLR_BL\x00" as *const u8 as *const libc::c_char,
          b"G_BL_CLR_FOG\x00" as *const u8 as *const libc::c_char],
         [b"G_BL_A_IN\x00" as *const u8 as *const libc::c_char,
          b"G_BL_A_FOG\x00" as *const u8 as *const libc::c_char,
          b"G_BL_A_SHADE\x00" as *const u8 as *const libc::c_char,
          b"G_BL_0\x00" as *const u8 as *const libc::c_char],
         [b"G_BL_CLR_IN\x00" as *const u8 as *const libc::c_char,
          b"G_BL_CLR_MEM\x00" as *const u8 as *const libc::c_char,
          b"G_BL_CLR_BL\x00" as *const u8 as *const libc::c_char,
          b"G_BL_CLR_FOG\x00" as *const u8 as *const libc::c_char],
         [b"G_BL_1MA\x00" as *const u8 as *const libc::c_char,
          b"G_BL_A_MEM\x00" as *const u8 as *const libc::c_char,
          b"G_BL_1\x00" as *const u8 as *const libc::c_char,
          b"G_BL_0\x00" as *const u8 as *const libc::c_char]];
    let mut i: s32 = 0;
    let mut a: s32 = 0;
    let mut b: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[F3dzexRenderMode; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<F3dzexRenderMode>()
                                                   as libc::c_ulong) as s32 {
        if !(mode & sUCodeDisasRenderModeFlags[i as usize].mask !=
                 sUCodeDisasRenderModeFlags[i as usize].value) {
            if (*this).enableLog != 0 {
                osSyncPrintf(b"%s|\x00" as *const u8 as *const libc::c_char,
                             sUCodeDisasRenderModeFlags[i as usize].name);
            }
        }
        i += 1
    }
    a =
        (mode >> 18 as libc::c_int & 0x3333 as libc::c_int as libc::c_uint) as
            s32;
    b =
        (mode >> 16 as libc::c_int & 0x3333 as libc::c_int as libc::c_uint) as
            s32;
    // clang-format off
    if !((*this).enableLog == 0 as libc::c_int) {
        osSyncPrintf(b"\nGBL_c1(%s, %s, %s, %s)|\x00" as *const u8 as
                         *const libc::c_char,
                     D_8012DDDC[0 as libc::c_int as
                                    usize][(a >> 12 as libc::c_int &
                                                3 as libc::c_int) as usize],
                     D_8012DDDC[1 as libc::c_int as
                                    usize][(a >> 8 as libc::c_int &
                                                3 as libc::c_int) as usize],
                     D_8012DDDC[2 as libc::c_int as
                                    usize][(a >> 4 as libc::c_int &
                                                3 as libc::c_int) as usize],
                     D_8012DDDC[3 as libc::c_int as
                                    usize][(a >> 0 as libc::c_int &
                                                3 as libc::c_int) as usize]);
    }
    // clang-format on
    if (*this).enableLog != 0 {
        osSyncPrintf(b"\nGBL_c2(%s, %s, %s, %s)\x00" as *const u8 as
                         *const libc::c_char,
                     D_8012DDDC[0 as libc::c_int as
                                    usize][(b >> 12 as libc::c_int &
                                                3 as libc::c_int) as usize],
                     D_8012DDDC[1 as libc::c_int as
                                    usize][(b >> 8 as libc::c_int &
                                                3 as libc::c_int) as usize],
                     D_8012DDDC[2 as libc::c_int as
                                    usize][(b >> 4 as libc::c_int &
                                                3 as libc::c_int) as usize],
                     D_8012DDDC[3 as libc::c_int as
                                    usize][(b >> 0 as libc::c_int &
                                                3 as libc::c_int) as usize]);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_PrintVertices(mut this: *mut UCodeDisas,
                                                  mut vtx: *mut Vtx,
                                                  mut count: s32,
                                                  mut start: s32) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < count {
        if (*this).geometryMode & 0x20000 as libc::c_int as libc::c_uint != 0
           {
            if (*this).enableLog != 0 {
                osSyncPrintf(b"\n{{%6d, %6d, %6d, %d, %6d, %6d, %3d, %3d, %3d, %3d}}, /* vc%d */\x00"
                                 as *const u8 as *const libc::c_char,
                             (*vtx).n.ob[0 as libc::c_int as usize] as
                                 libc::c_int,
                             (*vtx).n.ob[1 as libc::c_int as usize] as
                                 libc::c_int,
                             (*vtx).n.ob[2 as libc::c_int as usize] as
                                 libc::c_int, (*vtx).n.flag as libc::c_int,
                             (*vtx).n.tc[0 as libc::c_int as usize] as
                                 libc::c_int,
                             (*vtx).n.tc[1 as libc::c_int as usize] as
                                 libc::c_int,
                             (*vtx).n.n[0 as libc::c_int as usize] as
                                 libc::c_int,
                             (*vtx).n.n[1 as libc::c_int as usize] as
                                 libc::c_int,
                             (*vtx).n.n[2 as libc::c_int as usize] as
                                 libc::c_int, (*vtx).n.a as libc::c_int,
                             start + i);
            }
        } else if (*this).enableLog != 0 {
            osSyncPrintf(b"\n{{%6d, %6d, %6d, %d, %6d, %6d, %3d, %3d, %3d, %3d}}, /* vn%d */\x00"
                             as *const u8 as *const libc::c_char,
                         (*vtx).v.ob[0 as libc::c_int as usize] as
                             libc::c_int,
                         (*vtx).v.ob[1 as libc::c_int as usize] as
                             libc::c_int,
                         (*vtx).v.ob[2 as libc::c_int as usize] as
                             libc::c_int, (*vtx).v.flag as libc::c_int,
                         (*vtx).v.tc[0 as libc::c_int as usize] as
                             libc::c_int,
                         (*vtx).v.tc[1 as libc::c_int as usize] as
                             libc::c_int,
                         (*vtx).v.cn[0 as libc::c_int as usize] as
                             libc::c_int,
                         (*vtx).v.cn[1 as libc::c_int as usize] as
                             libc::c_int,
                         (*vtx).v.cn[2 as libc::c_int as usize] as
                             libc::c_int,
                         (*vtx).v.cn[3 as libc::c_int as usize] as
                             libc::c_int, start + i);
        }
        vtx = vtx.offset(1);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_Disassemble(mut this: *mut UCodeDisas,
                                                mut ptr: *mut GfxMod) {
    let mut pad: u32_0 = 0;
    let mut addr: u32_0 = 0;
    let mut rdpHalf: u32_0 = 0;
    let mut linkDlLow: u16_0 = 0;
    let mut sid: u8_0 = 0;
    let mut cmd: u8_0 = 0;
    let mut i0: s32 = 0;
    let mut exit: u32_0 = 0;
    let mut curGfx: [GfxMod; 1] = [GfxMod{words: Gwords{w0: 0, w1: 0,},}; 1];
    exit = 0 as libc::c_int as u32_0;
    while exit == 0 {
        (*this).dlCnt = (*this).dlCnt.wrapping_add(1);
        ptr = UCodeDisas_TranslateAddr(this, ptr as u32_0) as *mut GfxMod;
        if (*this).enableLog != 0 {
            osSyncPrintf(b"%08x:\x00" as *const u8 as *const libc::c_char,
                         ptr);
        }
        *curGfx.as_mut_ptr() = *ptr;
        cmd = (*curGfx.as_mut_ptr()).dma.cmd() as u8_0;
        addr =
            UCodeDisas_TranslateAddr(this, (*curGfx.as_mut_ptr()).dma.addr);
        if (*this).enableLog != 0 {
            osSyncPrintf(b"%08x-%08x:\x00" as *const u8 as
                             *const libc::c_char,
                         (*curGfx.as_mut_ptr()).words.w0,
                         (*curGfx.as_mut_ptr()).words.w1);
        }
        i0 = 0 as libc::c_int;
        while i0 < (*this).dlDepth {
            if (*this).enableLog != 0 {
                osSyncPrintf(b" \x00" as *const u8 as *const libc::c_char);
            }
            i0 += 1
        }
        match cmd as libc::c_int {
            224 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsSPNoOp(),\x00" as *const u8 as
                                     *const libc::c_char);
                }
            }
            222 => {
                let mut dma: Gdma = (*ptr).dma;
                match dma.par() as libc::c_int {
                    0 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"gsSPDisplayList(0x%08x),\x00" as
                                             *const u8 as *const libc::c_char,
                                         dma.addr);
                        }
                        let fresh0 = (*this).dlDepth;
                        (*this).dlDepth = (*this).dlDepth + 1;
                        (*this).dlStack[fresh0 as usize] =
                            ptr.offset(1 as libc::c_int as isize) as u32_0 as
                                *mut Gfx;
                        ptr =
                            (addr as
                                 *mut GfxMod).offset(-(1 as libc::c_int as
                                                           isize))
                    }
                    1 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"gsSPBranchList(0x%08x),\x00" as
                                             *const u8 as *const libc::c_char,
                                         dma.addr);
                        }
                        ptr =
                            (addr as
                                 *mut GfxMod).offset(-(1 as libc::c_int as
                                                           isize))
                    }
                    _ => { }
                }
            }
            225 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"RDPHALF_1(0x%08x),\x00" as *const u8 as
                                     *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).dma.addr);
                }
                rdpHalf = (*curGfx.as_mut_ptr()).dma.addr
            }
            228 => {
                let mut rect: Gtexrect = *(ptr as *mut Gtexrect);
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsSPTextureRectangle(%d,%d,%d,%d,%d,%d,%d,%d,%d),\x00"
                                     as *const u8 as *const libc::c_char,
                                 rect.xl() as libc::c_int,
                                 rect.yl() as libc::c_int,
                                 rect.xh() as libc::c_int,
                                 rect.yh() as libc::c_int,
                                 rect.tile() as libc::c_int,
                                 (*ptr.offset(1 as libc::c_int as
                                                  isize)).words.w1 >>
                                     16 as libc::c_int,
                                 (*ptr.offset(1 as libc::c_int as
                                                  isize)).words.w1 &
                                     0xffff as libc::c_int as libc::c_uint,
                                 (*ptr.offset(2 as libc::c_int as
                                                  isize)).words.w1 >>
                                     16 as libc::c_int,
                                 (*ptr.offset(2 as libc::c_int as
                                                  isize)).words.w1 &
                                     0xffff as libc::c_int as libc::c_uint);
                }
                ptr =
                    ptr.offset((3 as libc::c_int - 1 as libc::c_int) as
                                   isize);
                (*this).pipeSyncRequired = 1 as libc::c_int as u32_0
            }
            221 => {
                if (*curGfx.as_mut_ptr()).dma.len() as libc::c_int ==
                       0x7ff as libc::c_int {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"gsSPLoadUcode(0x%08x, 0x%08x),\x00" as
                                         *const u8 as *const libc::c_char,
                                     (*curGfx.as_mut_ptr()).dma.addr,
                                     rdpHalf);
                    }
                } else if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsSPLoadUcodeEx(0x%08x, 0x%08x, 0x%05x),\x00"
                                     as *const u8 as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).dma.addr, rdpHalf,
                                 (*curGfx.as_mut_ptr()).dma.len() as
                                     libc::c_int + 1 as libc::c_int);
                }
                UCodeDisas_SetCurUCodeImpl(this,
                                           UCodeDisas_TranslateAddr(this,
                                                                    (*curGfx.as_mut_ptr()).dma.addr)
                                               as *mut libc::c_void);
                (*this).loaducodeCnt = (*this).loaducodeCnt.wrapping_add(1)
            }
            223 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsSPEndDisplayList(),\x00" as *const u8 as
                                     *const libc::c_char);
                }
                if (*this).dlDepth <= 0 as libc::c_int {
                    exit = 1 as libc::c_int as u32_0
                } else {
                    (*this).dlDepth -= 1;
                    ptr =
                        ((*this).dlStack[(*this).dlDepth as usize] as
                             *mut GfxMod).offset(-(1 as libc::c_int as isize))
                }
            }
            245 => {
                let mut settile: Gsettile = (*ptr).settile;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetTile(%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d),\x00"
                                     as *const u8 as *const libc::c_char,
                                 settile.fmt() as libc::c_int,
                                 settile.siz() as libc::c_int,
                                 settile.line() as libc::c_int,
                                 settile.tmem() as libc::c_int,
                                 settile.tile() as libc::c_int,
                                 settile.palette() as libc::c_int,
                                 ((settile.ct() as libc::c_int) <<
                                      1 as libc::c_int) +
                                     settile.mt() as libc::c_int,
                                 settile.maskt() as libc::c_int,
                                 settile.shiftt() as libc::c_int,
                                 ((settile.cs() as libc::c_int) <<
                                      1 as libc::c_int) +
                                     settile.ms() as libc::c_int,
                                 settile.masks() as libc::c_int,
                                 settile.shifts() as libc::c_int);
                }
                if (*this).tileSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### TileSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            244 => {
                let mut loadtile: Gloadtile = (*ptr).loadtile;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPLoadTile(%d,%d,%d,%d,%d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 loadtile.tile() as libc::c_int,
                                 loadtile.sl() as libc::c_int,
                                 loadtile.tl() as libc::c_int,
                                 loadtile.sh() as libc::c_int,
                                 loadtile.th() as libc::c_int);
                }
            }
            243 => {
                let mut loadtile_0: Gloadtile = (*ptr).loadtile;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPLoadBlock(%d,%d,%d,%d,%d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 loadtile_0.tile() as libc::c_int,
                                 loadtile_0.sl() as libc::c_int,
                                 loadtile_0.tl() as libc::c_int,
                                 loadtile_0.sh() as libc::c_int,
                                 loadtile_0.th() as libc::c_int);
                }
                if (*this).loadSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### LoadSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
                (*this).pipeSyncRequired = 1 as libc::c_int as u32_0
            }
            242 => {
                let mut loadtile_1: Gloadtile = (*ptr).loadtile;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetTileSize(%d,%d,%d,%d,%d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 loadtile_1.tile() as libc::c_int,
                                 loadtile_1.sl() as libc::c_int,
                                 loadtile_1.tl() as libc::c_int,
                                 loadtile_1.sh() as libc::c_int,
                                 loadtile_1.th() as libc::c_int);
                }
            }
            240 => {
                let mut loadtlut: Gloadtlut = (*ptr).loadtlut;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPLoadTLUTCmd(%d,%d),\x00" as *const u8
                                     as *const libc::c_char,
                                 loadtlut.tile() as libc::c_int,
                                 loadtlut.sh() as libc::c_int >>
                                     2 as libc::c_int);
                }
            }
            252 => {
                let mut setcombine: GsetcombineMod = (*ptr).setcombine;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetCombineLERP(%s,%s,%s,%s, %s,%s,%s,%s, %s,%s,%s,%s, %s,%s,%s,%s),\x00"
                                     as *const u8 as *const libc::c_char,
                                 UCodeDisas_ParseCombineColor(setcombine.a(),
                                                              1 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineColor(setcombine.b(),
                                                              2 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineColor(setcombine.c(),
                                                              3 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineColor(setcombine.d(),
                                                              4 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.z(),
                                                              1 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.y(),
                                                              2 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.x(),
                                                              3 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.w(),
                                                              4 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineColor(setcombine.e(),
                                                              1 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineColor(setcombine.f(),
                                                              2 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineColor(setcombine.g(),
                                                              3 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineColor(setcombine.h(),
                                                              4 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.v(),
                                                              1 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.u(),
                                                              2 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.t(),
                                                              3 as libc::c_int
                                                                  as u32_0),
                                 UCodeDisas_ParseCombineAlpha(setcombine.s(),
                                                              4 as libc::c_int
                                                                  as u32_0));
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            227 => {
                let mut current_block_102: u64;
                static mut sUCodeDisasModeHMacros: [F3dzexSetModeMacro; 12] =
                    [{
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetAlphaDither\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    4 as libc::c_int as u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_AD_PATTERN\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              4
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_AD_NOTPATTERN\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              4
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_AD_NOISE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((2
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              4
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_AD_DISABLE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((3
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              4
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetColorDither\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    6 as libc::c_int as u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CD_MAGICSQ\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              6
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CD_BAYER\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              6
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CD_NOISE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((2
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              6
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetCombineKey\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    8 as libc::c_int as u32_0,
                                                len:
                                                    1 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CK_NONE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              8
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CK_KEY\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              8
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetTextureConvert\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    9 as libc::c_int as u32_0,
                                                len:
                                                    3 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TC_CONV\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              9
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TC_FILTCONV\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((5
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              9
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TC_FILT\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((6
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              9
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetTextureFilter\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    12 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TF_POINT\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              12
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TF_AVERAGE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((3
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              12
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TF_BILERP\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((2
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              12
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetTextureLUT\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    14 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TT_NONE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              14
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TT_RGBA16\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((2
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              14
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TT_IA16\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((3
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              14
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetTextureLOD\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    16 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    1 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TL_TILE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              16
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TL_LOD\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              16
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetTextureDetail\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    17 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TD_CLAMP\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              17
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TD_SHARPEN\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              17
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TD_DETAIL\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((2
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              17
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetTexturePersp\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    19 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    1 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TP_PERSP\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              19
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_TP_NONE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              19
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetCycleType\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    20 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CYC_1CYCLE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              20
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CYC_2CYCLE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              20
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CYC_COPY\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((2
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              20
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CYC_FILL\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((3
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              20
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"SetColorDither\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    22 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CD_MAGICSQ\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              6
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CD_BAYER\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              6
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_CD_NOISE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((2
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              6
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"PipelineMode\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    23 as libc::c_int as
                                                        u32_0,
                                                len:
                                                    1 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_PM_1PRIMITIVE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              23
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_PM_NPRIMITIVE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              23
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     }];
                let mut len: u32_0 =
                    ((*curGfx.as_mut_ptr()).setothermode.len() as libc::c_int
                         + 1 as libc::c_int) as u32_0;
                let mut sft: u32_0 =
                    (-((*curGfx.as_mut_ptr()).setothermode.sft() as
                           libc::c_int) as
                         libc::c_uint).wrapping_sub(len).wrapping_add(32 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint);
                let mut s2: u32_0 =
                    (*curGfx.as_mut_ptr()).setothermode.data().wrapping_mul(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint);
                let mut i1: u32_0 = 0;
                let mut i2: u32_0 = 0;
                i1 = 0 as libc::c_int as u32_0;
                's_553:
                    loop  {
                        if !(i1 <
                                 (::std::mem::size_of::<[F3dzexSetModeMacro; 12]>()
                                      as
                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<F3dzexSetModeMacro>()
                                                                      as
                                                                      libc::c_ulong))
                           {
                            current_block_102 = 9073771928613846474;
                            break ;
                        }
                        if sft == sUCodeDisasModeHMacros[i1 as usize].shift {
                            i2 = 0 as libc::c_int as u32_0;
                            while i2 < 4 as libc::c_int as libc::c_uint {
                                if s2 ==
                                       sUCodeDisasModeHMacros[i1 as
                                                                  usize].values[i2
                                                                                    as
                                                                                    usize].value
                                   {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsDP%s(%s),\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     sUCodeDisasModeHMacros[i1
                                                                                as
                                                                                usize].name,
                                                     sUCodeDisasModeHMacros[i1
                                                                                as
                                                                                usize].values[i2
                                                                                                  as
                                                                                                  usize].name);
                                    }
                                    current_block_102 = 9540602145145119674;
                                    break 's_553 ;
                                } else { i2 = i2.wrapping_add(1) }
                            }
                        }
                        i1 = i1.wrapping_add(1)
                    }
                match current_block_102 {
                    9073771928613846474 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"gsSPSetOtherModeH(%d, %d, 0x%08x),\x00"
                                             as *const u8 as
                                             *const libc::c_char, sft, len,
                                         s2);
                        }
                    }
                    _ => { }
                }
                (*this).modeH &=
                    ((1 as libc::c_int - ((1 as libc::c_int) << len) << sft) -
                         1 as libc::c_int) as libc::c_uint;
                (*this).modeH |= s2;
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            226 => {
                let mut current_block_122: u64;
                static mut sUCodeDisasModeLMacros: [F3dzexSetModeMacro; 2] =
                    [{
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"gsDPSetAlphaCompare\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    0 as libc::c_int as u32_0,
                                                len:
                                                    2 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_AC_NONE\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_AC_THRESHOLD\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_AC_DITHER\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((3
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     },
                     {
                         let mut init =
                             F3dzexSetModeMacro{name:
                                                    b"gsDPSetDepthSource\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                shift:
                                                    2 as libc::c_int as u32_0,
                                                len:
                                                    1 as libc::c_int as u32_0,
                                                values:
                                                    [{
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_ZS_PIXEL\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((0
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              2
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"G_ZS_PRIM\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         ((1
                                                                                               as
                                                                                               libc::c_int)
                                                                                              <<
                                                                                              2
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     },
                                                     {
                                                         let mut init =
                                                             F3dzexSetModeMacroValue{name:
                                                                                         b"-1\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                     value:
                                                                                         -(1
                                                                                               as
                                                                                               libc::c_int)
                                                                                             as
                                                                                             u32_0,};
                                                         init
                                                     }],};
                         init
                     }];
                let mut len_0: u32_0 =
                    ((*curGfx.as_mut_ptr()).setothermode.len() as libc::c_int
                         + 1 as libc::c_int) as u32_0;
                let mut sft_0: u32_0 =
                    (-((*curGfx.as_mut_ptr()).setothermode.sft() as
                           libc::c_int) as
                         libc::c_uint).wrapping_sub(len_0).wrapping_add(32 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint);
                let mut s2_0: u32_0 =
                    (*curGfx.as_mut_ptr()).setothermode.data().wrapping_mul(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint);
                let mut i1_0: u32_0 = 0;
                let mut i2_0: u32_0 = 0;
                if sft_0 == 3 as libc::c_int as libc::c_uint {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"\ngsDPSetRenderBlender(\x00" as
                                         *const u8 as *const libc::c_char);
                    }
                    UCodeDisas_ParseRenderMode(this, s2_0);
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"\n),\x00" as *const u8 as
                                         *const libc::c_char);
                    }
                } else {
                    i1_0 = 0 as libc::c_int as u32_0;
                    's_707:
                        loop  {
                            if !(i1_0.wrapping_mul(1 as libc::c_int as
                                                       libc::c_uint) <
                                     (::std::mem::size_of::<[F3dzexSetModeMacro; 2]>()
                                          as
                                          libc::c_ulong).wrapping_div(::std::mem::size_of::<F3dzexSetModeMacro>()
                                                                          as
                                                                          libc::c_ulong))
                               {
                                current_block_122 = 1421636490742020198;
                                break ;
                            }
                            if sft_0 ==
                                   sUCodeDisasModeLMacros[i1_0 as usize].shift
                               {
                                i2_0 = 0 as libc::c_int as u32_0;
                                while i2_0 < 4 as libc::c_int as libc::c_uint
                                      {
                                    if s2_0 ==
                                           sUCodeDisasModeLMacros[i1_0 as
                                                                      usize].values[i2_0
                                                                                        as
                                                                                        usize].value
                                       {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsDP%s(%s),\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         sUCodeDisasModeLMacros[i1_0
                                                                                    as
                                                                                    usize].name,
                                                         sUCodeDisasModeLMacros[i1_0
                                                                                    as
                                                                                    usize].values[i2_0
                                                                                                      as
                                                                                                      usize].name);
                                        }
                                        current_block_122 =
                                            13875453145351153743;
                                        break 's_707 ;
                                    } else { i2_0 = i2_0.wrapping_add(1) }
                                }
                            }
                            i1_0 = i1_0.wrapping_add(1)
                        }
                    match current_block_122 {
                        13875453145351153743 => { }
                        _ => {
                            if (*this).enableLog != 0 {
                                osSyncPrintf(b"gsSPSetOtherModeL(%d, %d, 0x%08x),\x00"
                                                 as *const u8 as
                                                 *const libc::c_char, sft_0,
                                             len_0, s2_0);
                            }
                        }
                    }
                }
                (*this).modeL &=
                    ((1 as libc::c_int - ((1 as libc::c_int) << len_0) <<
                          sft_0) - 1 as libc::c_int) as libc::c_uint;
                (*this).modeL |= s2_0;
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            239 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetOtherMode(0x%08x, 0x%08x),\x00" as
                                     *const u8 as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).words.w0 &
                                     0xffffff as libc::c_int as libc::c_uint,
                                 (*curGfx.as_mut_ptr()).words.w1);
                }
                (*this).modeH =
                    (*curGfx.as_mut_ptr()).words.w0 &
                        0xfff as libc::c_int as libc::c_uint;
                (*this).modeL = (*curGfx.as_mut_ptr()).words.w1;
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            237 => {
                let mut setscissor: Gfillrect = (*ptr).fillrect;
                let mut modeStr: *const libc::c_char =
                    0 as *const libc::c_char;
                modeStr =
                    if setscissor.pad() as libc::c_int == 0 as libc::c_int {
                        b"G_SC_NON_INTERLACE\x00" as *const u8 as
                            *const libc::c_char
                    } else if setscissor.pad() as libc::c_int ==
                                  3 as libc::c_int {
                        b"G_SC_ODD_INTERLACE\x00" as *const u8 as
                            *const libc::c_char
                    } else if setscissor.pad() as libc::c_int ==
                                  2 as libc::c_int {
                        b"G_SC_EVEN_INTERLACE\x00" as *const u8 as
                            *const libc::c_char
                    } else { b"???\x00" as *const u8 as *const libc::c_char };
                if setscissor.x0frac() | setscissor.y0frac() |
                       setscissor.x1frac() | setscissor.y1frac() != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"gsDPSetScissorFrac(%s, %d, %d, %d, %d),\x00"
                                         as *const u8 as *const libc::c_char,
                                     modeStr,
                                     (setscissor.x0() << 2 as libc::c_int) +
                                         setscissor.x0frac(),
                                     (setscissor.y0() << 2 as libc::c_int) +
                                         setscissor.y0frac(),
                                     (setscissor.x1() << 2 as libc::c_int) +
                                         setscissor.x1frac(),
                                     (setscissor.y1() << 2 as libc::c_int) +
                                         setscissor.y1frac());
                    }
                } else if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetScissor(%s, %d, %d, %d, %d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 modeStr, setscissor.x0(), setscissor.y0(),
                                 setscissor.x1(), setscissor.y1());
                }
            }
            246 => {
                let mut fillrect: Gfillrect = (*ptr).fillrect;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPFillRectangle(%d, %d, %d, %d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 fillrect.x1(), fillrect.y1(), fillrect.x0(),
                                 fillrect.y0());
                }
                (*this).pipeSyncRequired = 1 as libc::c_int as u32_0
            }
            255 => {
                let mut fmt: u32_0 =
                    ((*curGfx.as_mut_ptr()).words.w0 &
                         0xe00000 as libc::c_int as libc::c_uint) >>
                        0x15 as libc::c_int &
                        0xff as libc::c_int as libc::c_uint;
                let mut siz: u32_0 =
                    ((*curGfx.as_mut_ptr()).words.w0 &
                         0x180000 as libc::c_int as libc::c_uint) >>
                        0x13 as libc::c_int &
                        0xff as libc::c_int as libc::c_uint;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetColorImage(G_IM_FMT_%s, G_IM_SIZ_%s, %d, 0x%08x(0x%08x) ),\x00"
                                     as *const u8 as *const libc::c_char,
                                 if fmt == 0 as libc::c_int as libc::c_uint {
                                     b"RGBA\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if fmt ==
                                               1 as libc::c_int as
                                                   libc::c_uint {
                                     b"YUV\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if fmt ==
                                               2 as libc::c_int as
                                                   libc::c_uint {
                                     b"CI\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if fmt ==
                                               3 as libc::c_int as
                                                   libc::c_uint {
                                     b"IA\x00" as *const u8 as
                                         *const libc::c_char
                                 } else {
                                     b"I\x00" as *const u8 as
                                         *const libc::c_char
                                 },
                                 if siz == 0 as libc::c_int as libc::c_uint {
                                     b"4b\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if siz ==
                                               1 as libc::c_int as
                                                   libc::c_uint {
                                     b"8b\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if siz ==
                                               2 as libc::c_int as
                                                   libc::c_uint {
                                     b"16b\x00" as *const u8 as
                                         *const libc::c_char
                                 } else {
                                     b"32b\x00" as *const u8 as
                                         *const libc::c_char
                                 },
                                 ((*curGfx.as_mut_ptr()).dma.len() as
                                      libc::c_int & 0xfff as libc::c_int) +
                                     1 as libc::c_int,
                                 (*curGfx.as_mut_ptr()).setimg.dram, addr);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            254 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetDepthImage(0x%08x(0x%08x)),\x00" as
                                     *const u8 as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).words.w1, addr);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            253 => {
                let mut fmt_0: u32_0 =
                    ((*curGfx.as_mut_ptr()).words.w0 &
                         0xe00000 as libc::c_int as libc::c_uint) >>
                        0x15 as libc::c_int &
                        0xff as libc::c_int as libc::c_uint;
                let mut siz_0: u32_0 =
                    ((*curGfx.as_mut_ptr()).words.w0 &
                         0x180000 as libc::c_int as libc::c_uint) >>
                        0x13 as libc::c_int &
                        0xff as libc::c_int as libc::c_uint;
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetTextureImage(G_IM_FMT_%s, G_IM_SIZ_%s, %d, 0x%08x(0x%08x)),\x00"
                                     as *const u8 as *const libc::c_char,
                                 if fmt_0 == 0 as libc::c_int as libc::c_uint
                                    {
                                     b"RGBA\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if fmt_0 ==
                                               1 as libc::c_int as
                                                   libc::c_uint {
                                     b"YUV\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if fmt_0 ==
                                               2 as libc::c_int as
                                                   libc::c_uint {
                                     b"CI\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if fmt_0 ==
                                               3 as libc::c_int as
                                                   libc::c_uint {
                                     b"IA\x00" as *const u8 as
                                         *const libc::c_char
                                 } else {
                                     b"I\x00" as *const u8 as
                                         *const libc::c_char
                                 },
                                 if siz_0 == 0 as libc::c_int as libc::c_uint
                                    {
                                     b"4b\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if siz_0 ==
                                               1 as libc::c_int as
                                                   libc::c_uint {
                                     b"8b\x00" as *const u8 as
                                         *const libc::c_char
                                 } else if siz_0 ==
                                               2 as libc::c_int as
                                                   libc::c_uint {
                                     b"16b\x00" as *const u8 as
                                         *const libc::c_char
                                 } else {
                                     b"32b\x00" as *const u8 as
                                         *const libc::c_char
                                 },
                                 ((*curGfx.as_mut_ptr()).dma.len() as
                                      libc::c_int & 0xfff as libc::c_int) +
                                     1 as libc::c_int,
                                 (*curGfx.as_mut_ptr()).setimg.dram, addr);
                }
            }
            251 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetEnvColor(%d, %d, %d, %d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).setcolor.r as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.g as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.b as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.a as
                                     libc::c_int);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            249 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetBlendColor(%d, %d, %d, %d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).setcolor.r as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.g as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.b as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.a as
                                     libc::c_int);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            248 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetFogColor(%d, %d, %d, %d),\x00" as
                                     *const u8 as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).setcolor.r as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.g as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.b as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.a as
                                     libc::c_int);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            247 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetFillColor(0x%08x),\x00" as *const u8
                                     as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).words.w1);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            238 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetPrimDepth(%d, %d),\x00" as *const u8
                                     as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).setprimdepth.z as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setprimdepth.d as
                                     libc::c_int);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            250 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPSetPrimColor(%d, %d, %d, %d, %d, %d),\x00"
                                     as *const u8 as *const libc::c_char,
                                 (*curGfx.as_mut_ptr()).setcolor.prim_min_level
                                     as libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.prim_level as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.r as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.g as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.b as
                                     libc::c_int,
                                 (*curGfx.as_mut_ptr()).setcolor.a as
                                     libc::c_int);
                }
            }
            233 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPFullSync(),\x00" as *const u8 as
                                     *const libc::c_char);
                }
                if (*this).pipeSyncRequired != 0 {
                    if (*this).enableLog != 0 {
                        osSyncPrintf(b"### PipeSync\xe3\x81\x8c\xe5\xbf\x85\xe8\xa6\x81\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                                         as *const u8 as *const libc::c_char);
                    }
                    (*this).syncErr = (*this).syncErr.wrapping_add(1)
                }
            }
            232 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPTileSync(),\x00" as *const u8 as
                                     *const libc::c_char);
                }
                (*this).tileSyncRequired = 0 as libc::c_int as u32_0
            }
            231 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPPipeSync(),\x00" as *const u8 as
                                     *const libc::c_char);
                }
                (*this).pipeSyncRequired = 0 as libc::c_int as u32_0
            }
            230 => {
                if (*this).enableLog != 0 {
                    osSyncPrintf(b"gsDPLoadSync(),\x00" as *const u8 as
                                     *const libc::c_char);
                }
                (*this).loadSyncRequired = 0 as libc::c_int as u32_0
            }
            0 => {
                match (*curGfx.as_mut_ptr()).noop.type_0 as libc::c_int {
                    0 => {
                        if (*curGfx.as_mut_ptr()).noop.value.u32_0 ==
                               0 as libc::c_int as libc::c_uint {
                            if (*this).enableLog != 0 {
                                osSyncPrintf(b"gsDPNoOp(),\x00" as *const u8
                                                 as *const libc::c_char);
                            }
                        } else if (*this).enableLog != 0 {
                            osSyncPrintf(b"gsDPNoOpTag(%08x),\x00" as
                                             *const u8 as *const libc::c_char,
                                         (*curGfx.as_mut_ptr()).noop.value.u32_0);
                        }
                    }
                    1 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpHere([%s:%d]),\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*curGfx.as_mut_ptr()).noop.value.u32_0,
                                         (*curGfx.as_mut_ptr()).noop.len as
                                             libc::c_int);
                        }
                    }
                    7 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpOpenDisp([%s:%d]),\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*curGfx.as_mut_ptr()).noop.value.u32_0,
                                         (*curGfx.as_mut_ptr()).noop.len as
                                             libc::c_int);
                        }
                    }
                    8 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpCloseDisp([%s:%d]),\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*curGfx.as_mut_ptr()).noop.value.u32_0,
                                         (*curGfx.as_mut_ptr()).noop.len as
                                             libc::c_int);
                        }
                    }
                    2 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpString(%c%s%c, %d),\x00"
                                             as *const u8 as
                                             *const libc::c_char, '\"' as i32,
                                         (*curGfx.as_mut_ptr()).noop.value.u32_0,
                                         '\"' as i32,
                                         (*curGfx.as_mut_ptr()).noop.len as
                                             libc::c_int);
                        }
                    }
                    3 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpWord(0x%08x, %d),\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*curGfx.as_mut_ptr()).noop.value.u32_0,
                                         (*curGfx.as_mut_ptr()).noop.len as
                                             libc::c_int);
                        }
                    }
                    4 => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpFloat(%8.3f, %d),\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*curGfx.as_mut_ptr()).noop.value.f32_0
                                             as libc::c_double,
                                         (*curGfx.as_mut_ptr()).noop.len as
                                             libc::c_int);
                        }
                    }
                    5 => {
                        if (*curGfx.as_mut_ptr()).noop.len as libc::c_int ==
                               0 as libc::c_int {
                            if (*this).enableLog != 0 {
                                osSyncPrintf(b"count_gsDPNoOpQuiet(),\x00" as
                                                 *const u8 as
                                                 *const libc::c_char);
                            }
                        } else if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpVerbose(),\x00" as
                                             *const u8 as
                                             *const libc::c_char);
                        }
                        (*this).enableLog =
                            (*curGfx.as_mut_ptr()).noop.len as s32
                    }
                    6 => {
                        /* ! @bug arguments are not printed */
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"count_gsDPNoOpCallBack(%08x,%d),\x00"
                                             as *const u8 as
                                             *const libc::c_char); /* ! @bug gmtx.addr shouldn't be here*/
                        }
                        ::std::mem::transmute::<libc::intptr_t,
                                                Option<unsafe extern "C" fn(_:
                                                                                *mut UCodeDisas,
                                                                            _:
                                                                                u32_0)
                                                           ->
                                                               ()>>((*curGfx.as_mut_ptr()).noop.value.u32_0
                                                                        as
                                                                        libc::intptr_t).expect("non-null function pointer")(this,
                                                                                                                            (*curGfx.as_mut_ptr()).noop.len
                                                                                                                                as
                                                                                                                                u32_0);
                    }
                    _ => {
                        if (*this).enableLog != 0 {
                            osSyncPrintf(b"gsDPNoOpTag3(%02x, %08x, %04x),\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*curGfx.as_mut_ptr()).noop.type_0 as
                                             libc::c_int,
                                         (*curGfx.as_mut_ptr()).noop.value.u32_0,
                                         (*curGfx.as_mut_ptr()).noop.len as
                                             libc::c_int);
                        }
                    }
                }
            }
            _ => {
                match (*this).ucodeType {
                    1 | 2 => {
                        match cmd as libc::c_int {
                            218 => {
                                let mut gmtx: Gmatrix = (*ptr).matrix;
                                let mut params: u32_0 = 0;
                                let mut mtx: MtxF = MtxF{mf: [[0.; 4]; 4],};
                                let mut i1_1: s32 = 0 as libc::c_int;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPMatrix(0x%08x(%08x), 0\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 gmtx.addr, addr);
                                }
                                params =
                                    (gmtx.params as libc::c_int ^
                                         0x1 as libc::c_int) as u32_0;
                                while i1_1 !=
                                          (::std::mem::size_of::<[F3dzexFlag; 3]>()
                                               as
                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<F3dzexFlag>()
                                                                               as
                                                                               libc::c_ulong)
                                              as s32 {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"|%s\x00" as *const u8
                                                         as
                                                         *const libc::c_char,
                                                     if sUCodeDisasMtxFlags[i1_1
                                                                                as
                                                                                usize].value
                                                            & params != 0 {
                                                         sUCodeDisasMtxFlags[i1_1
                                                                                 as
                                                                                 usize].setName
                                                     } else {
                                                         sUCodeDisasMtxFlags[i1_1
                                                                                 as
                                                                                 usize].unsetName
                                                     });
                                    }
                                    i1_1 += 1
                                }
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"),\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 gmtx.addr);
                                }
                                if (*this).enableLog >= 2 as libc::c_int {
                                    MtxConv_L2F(&mut mtx, addr as *mut Mtx);
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"\n\x00" as *const u8 as
                                                         *const libc::c_char);
                                    }
                                    // clang-format on
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"/ %04x.%04x %04x.%04x %04x.%04x %.04x.%04x \\/ %12.6f %12.6f %12.6f %12.6f \\\n| %04x.%04x %04x.%04x %04x.%04x %.04x.%04x || %12.6f %12.6f %12.6f %12.6f |\n| %04x.%04x %04x.%04x %04x.%04x %.04x.%04x || %12.6f %12.6f %12.6f %12.6f |\n\\ %04x.%04x %04x.%04x %04x.%04x %.04x.%04x /\\ %12.6f %12.6f %12.6f %12.6f /\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][0
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][0
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][0
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[3
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[3
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][0
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     mtx.mf[0 as libc::c_int
                                                                as
                                                                usize][0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[1 as libc::c_int
                                                                as
                                                                usize][0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[2 as libc::c_int
                                                                as
                                                                usize][0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[3 as libc::c_int
                                                                as
                                                                usize][0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[3
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[3
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     mtx.mf[0 as libc::c_int
                                                                as
                                                                usize][1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[1 as libc::c_int
                                                                as
                                                                usize][1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[2 as libc::c_int
                                                                as
                                                                usize][1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[3 as libc::c_int
                                                                as
                                                                usize][1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][2
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][2
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][2
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[3
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][2
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[3
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     mtx.mf[0 as libc::c_int
                                                                as
                                                                usize][2 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[1 as libc::c_int
                                                                as
                                                                usize][2 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[2 as libc::c_int
                                                                as
                                                                usize][2 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[3 as libc::c_int
                                                                as
                                                                usize][2 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][3
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][3
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][3
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][3
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][3
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][3
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.intPart[3
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize][3
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                         as libc::c_int,
                                                     (*(addr as
                                                            *mut Mtx)).c2rust_unnamed.fracPart[3
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize][3
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              usize]
                                                         as libc::c_int,
                                                     mtx.mf[0 as libc::c_int
                                                                as
                                                                usize][3 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[1 as libc::c_int
                                                                as
                                                                usize][3 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[2 as libc::c_int
                                                                as
                                                                usize][3 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double,
                                                     mtx.mf[3 as libc::c_int
                                                                as
                                                                usize][3 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as libc::c_double);
                                    }
                                }
                            }
                            1 => {
                                let mut numv: u32_0 =
                                    (*curGfx.as_mut_ptr()).words.w0;
                                let mut vbidx: u32_0 = 0;
                                numv >>= 12 as libc::c_int;
                                numv &= 0xff as libc::c_int as libc::c_uint;
                                vbidx =
                                    (((*curGfx.as_mut_ptr()).vtx.vbidx as
                                          libc::c_int >> 1 as libc::c_int) as
                                         libc::c_uint).wrapping_sub(numv);
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPVertex(0x%08x(0x%08x), %d, %d),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*curGfx.as_mut_ptr()).words.w1,
                                                 addr, numv, vbidx);
                                }
                                (*this).vtxCnt =
                                    ((*this).vtxCnt as
                                         libc::c_uint).wrapping_add(numv) as
                                        u32_0 as u32_0;
                                (*this).spvtxCnt =
                                    (*this).spvtxCnt.wrapping_add(1);
                                if (*this).enableLog >= 2 as libc::c_int {
                                    UCodeDisas_PrintVertices(this,
                                                             addr as *mut Vtx,
                                                             numv as s32,
                                                             vbidx as s32);
                                }
                            }
                            2 => {
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPModifyVertex(%d, %s, %08x),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*curGfx.as_mut_ptr()).dma.par()
                                                     as libc::c_int,
                                                 if (*curGfx.as_mut_ptr()).dma.len()
                                                        as libc::c_int ==
                                                        0x10 as libc::c_int {
                                                     b"G_MWO_POINT_RGBA\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                 } else if (*curGfx.as_mut_ptr()).dma.len()
                                                               as libc::c_int
                                                               ==
                                                               0x14 as
                                                                   libc::c_int
                                                  {
                                                     b"G_MWO_POINT_ST\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                 } else if (*curGfx.as_mut_ptr()).dma.len()
                                                               as libc::c_int
                                                               ==
                                                               0x18 as
                                                                   libc::c_int
                                                  {
                                                     b"G_MWO_POINT_XYSCREEN\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                 } else if (*curGfx.as_mut_ptr()).dma.len()
                                                               as libc::c_int
                                                               ==
                                                               0x1c as
                                                                   libc::c_int
                                                  {
                                                     b"G_MWO_POINT_ZSCREEN\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                 } else {
                                                     b"G_MWO_POINT_????\x00"
                                                         as *const u8 as
                                                         *const libc::c_char
                                                 },
                                                 (*curGfx.as_mut_ptr()).dma.addr);
                                }
                                (*this).vtxCnt =
                                    ((*this).vtxCnt as
                                         libc::c_uint).wrapping_add((*curGfx.as_mut_ptr()).dma.par())
                                        as u32_0 as u32_0;
                                (*this).spvtxCnt =
                                    (*this).spvtxCnt.wrapping_add(1)
                            }
                            5 => {
                                let mut gtri: Gtri1 = (*ptr).tri1;
                                let mut tri: Gtrimod = gtri.tri;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSP1Triangle(%d, %d, %d),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 tri.v0() as libc::c_int /
                                                     2 as libc::c_int,
                                                 tri.v1() as libc::c_int /
                                                     2 as libc::c_int,
                                                 tri.v2() as libc::c_int /
                                                     2 as libc::c_int);
                                }
                                (*this).tri1Cnt =
                                    (*this).tri1Cnt.wrapping_add(1);
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            8 => {
                                if (*curGfx.as_mut_ptr()).linefix.wd as
                                       libc::c_int == 0 as libc::c_int {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPLine3D(%d, %d),\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     (*curGfx.as_mut_ptr()).linefix.v0
                                                         as libc::c_int,
                                                     (*curGfx.as_mut_ptr()).linefix.v1
                                                         as libc::c_int);
                                    }
                                } else if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPLineW3D(%d, %d, %d),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*curGfx.as_mut_ptr()).linefix.v0
                                                     as libc::c_int,
                                                 (*curGfx.as_mut_ptr()).linefix.v1
                                                     as libc::c_int,
                                                 (*curGfx.as_mut_ptr()).linefix.wd
                                                     as libc::c_int);
                                }
                                (*this).lineCnt =
                                    (*this).lineCnt.wrapping_add(1);
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            6 => {
                                let mut tri2: Gtri2 = (*ptr).tri2;
                                let mut v0: u32_0 = 0;
                                let mut v1: u32_0 = 0;
                                let mut v2: u32_0 = 0;
                                let mut v3: u32_0 = 0;
                                let mut v4: u32_0 = 0;
                                let mut v5: u32_0 = 0;
                                v0 =
                                    (tri2.tri1.v0() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v1 =
                                    (tri2.tri1.v1() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v2 =
                                    (tri2.tri1.v2() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v3 =
                                    (tri2.tri2.v0() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v4 =
                                    (tri2.tri2.v1() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v5 =
                                    (tri2.tri2.v2() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSP2Triangles(%d, %d, %d, 0, %d, %d, %d, 0),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char, v0,
                                                 v1, v2, v3, v4, v5);
                                }
                                (*this).tri2Cnt =
                                    (*this).tri2Cnt.wrapping_add(1);
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            7 => {
                                let mut quad: Gquadmod = (*ptr).quad;
                                let mut v0_0: u32_0 = 0;
                                let mut v1_0: u32_0 = 0;
                                let mut v2_0: u32_0 = 0;
                                let mut v3_0: u32_0 = 0;
                                v0_0 =
                                    (quad.v0() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v1_0 =
                                    (quad.v1() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v2_0 =
                                    (quad.v2() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                v3_0 =
                                    (quad.v3() as libc::c_int /
                                         2 as libc::c_int) as u32_0;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSP1Quadrangle(%d, %d, %d, %d, 0),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 v0_0, v1_0, v2_0, v3_0);
                                }
                                (*this).quadCnt =
                                    (*this).quadCnt.wrapping_add(1);
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            3 => {
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPCullDisplayList(%d, %d),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*curGfx.as_mut_ptr()).cull.vstart
                                                     as libc::c_int /
                                                     2 as libc::c_int,
                                                 (*curGfx.as_mut_ptr()).cull.vend
                                                     as libc::c_int /
                                                     2 as libc::c_int);
                                }
                            }
                            4 => {
                                addr =
                                    UCodeDisas_TranslateAddr(this, rdpHalf);
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPBranchLessZraw(0x%08x(0x%08x), %d, 0x%08x),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 rdpHalf, addr,
                                                 ((*curGfx.as_mut_ptr()).words.w0
                                                      &
                                                      0xfff as libc::c_int as
                                                          libc::c_uint).wrapping_div(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint),
                                                 (*curGfx.as_mut_ptr()).words.w1);
                                }
                                ptr =
                                    (addr as
                                         *mut GfxMod).offset(-(1 as
                                                                   libc::c_int
                                                                   as isize))
                            }
                            215 => {
                                let mut texture: Gtexturemod = (*ptr).texmod;
                                if texture.lodscale() as libc::c_int ==
                                       0 as libc::c_int {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPTexture(%d, %d, %d, %d, %s),\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     texture.s as libc::c_int,
                                                     texture.t as libc::c_int,
                                                     texture.level() as
                                                         libc::c_int,
                                                     texture.tile() as
                                                         libc::c_int,
                                                     if texture.on as
                                                            libc::c_int != 0 {
                                                         b"G_ON\x00" as
                                                             *const u8 as
                                                             *const libc::c_char
                                                     } else {
                                                         b"G_OFF\x00" as
                                                             *const u8 as
                                                             *const libc::c_char
                                                     });
                                    }
                                } else if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPTextureL(%d, %d, %d, %d, %d, %s),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 texture.s as libc::c_int,
                                                 texture.t as libc::c_int,
                                                 texture.level() as
                                                     libc::c_int,
                                                 texture.lodscale() as
                                                     libc::c_int,
                                                 texture.tile() as
                                                     libc::c_int,
                                                 if texture.on as libc::c_int
                                                        != 0 {
                                                     b"G_ON\x00" as *const u8
                                                         as
                                                         *const libc::c_char
                                                 } else {
                                                     b"G_OFF\x00" as *const u8
                                                         as
                                                         *const libc::c_char
                                                 });
                                }
                            }
                            216 => {
                                let mut popmtx: Gpopmtxmod = (*ptr).popmtxmod;
                                if popmtx.param() as libc::c_int ==
                                       1 as libc::c_int {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPPopMatrix(G_MTX_MODELVIEW),\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                    }
                                } else if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPPopMatrixN(G_MTX_MODELVIEW, %d),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 popmtx.param() as
                                                     libc::c_int);
                                }
                            }
                            217 => {
                                let mut clearbits: u32_0 =
                                    (*curGfx.as_mut_ptr()).words.w0 &
                                        0xffffff as libc::c_int as
                                            libc::c_uint;
                                let mut setbits: u32_0 =
                                    (*curGfx.as_mut_ptr()).words.w1 &
                                        0xffffff as libc::c_int as
                                            libc::c_uint;
                                if clearbits ==
                                       0 as libc::c_int as libc::c_uint {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPLoadGeometryMode(\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                    }
                                    UCodeDisas_ParseGeometryMode(this,
                                                                 setbits);
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"),\x00" as *const u8 as
                                                         *const libc::c_char);
                                    }
                                } else if setbits ==
                                              0 as libc::c_int as libc::c_uint
                                 {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPClearGeometryMode(\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                    }
                                    UCodeDisas_ParseGeometryMode(this,
                                                                 !clearbits);
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"),\x00" as *const u8 as
                                                         *const libc::c_char);
                                    }
                                } else if clearbits ==
                                              0xffffff as libc::c_int as
                                                  libc::c_uint {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPSetGeometryMode(\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                    }
                                    UCodeDisas_ParseGeometryMode(this,
                                                                 setbits);
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"),\x00" as *const u8 as
                                                         *const libc::c_char);
                                    }
                                } else {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPGeometryMode(\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                    }
                                    UCodeDisas_ParseGeometryMode(this,
                                                                 !clearbits);
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b", \x00" as *const u8 as
                                                         *const libc::c_char);
                                    }
                                    UCodeDisas_ParseGeometryMode(this,
                                                                 setbits);
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"),\x00" as *const u8 as
                                                         *const libc::c_char);
                                    }
                                }
                                (*this).geometryMode &= clearbits;
                                (*this).geometryMode |= setbits
                                /* ! @bug  %.04x.%04x is a typo, should be  %04x.%04x */
                                    // clang-format off
                                // this break needs to be inside, but the other cases need their breaks to be
                                       // outside...
                            }
                            219 => {
                                let mut pad_0: u32_0 = 0;
                                let mut dma_0: Gdma = (*ptr).dma;
                                let mut movewd: Gmovewd = (*ptr).movewd;
                                movewd.set_index(dma_0.par());
                                movewd.set_offset(dma_0.len());
                                movewd.data = dma_0.addr;
                                match movewd.index() as libc::c_int {
                                    6 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPSegment(%d, 0x%08x),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movewd.offset() as
                                                             libc::c_int /
                                                             4 as libc::c_int,
                                                         movewd.data);
                                        }
                                        (*this).segments[(movewd.offset() as
                                                              libc::c_int /
                                                              4 as
                                                                  libc::c_int)
                                                             as usize] =
                                            movewd.data &
                                                0xffffff as libc::c_int as
                                                    libc::c_uint
                                    }
                                    4 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPClipRatio(FRUSTRATIO_%d), \x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         if movewd.data !=
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint
                                                            {
                                                             movewd.data
                                                         } else {
                                                             movewd.data.wrapping_neg()
                                                         });
                                        }
                                        ptr =
                                            ptr.offset((4 as libc::c_int -
                                                            1 as libc::c_int)
                                                           as isize)
                                    }
                                    2 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPNumLights(%d), \x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movewd.data.wrapping_div(24
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint));
                                        }
                                    }
                                    10 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPLightColor(%d, %d), \x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         ((movewd.offset() as
                                                               libc::c_int &
                                                               0xf0 as
                                                                   libc::c_int)
                                                              >>
                                                              5 as
                                                                  libc::c_int)
                                                             +
                                                             1 as libc::c_int,
                                                         movewd.data);
                                        }
                                        ptr =
                                            ptr.offset((2 as libc::c_int -
                                                            1 as libc::c_int)
                                                           as isize)
                                    }
                                    8 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPFogFactor(%d, %d),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movewd.data >>
                                                             16 as
                                                                 libc::c_int,
                                                         movewd.data &
                                                             0xffff as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint);
                                        }
                                    }
                                    14 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPPerspNormalize(%d),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movewd.data);
                                        }
                                    }
                                    _ => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsMoveWd(%d, %d, %d), \x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movewd.index() as
                                                             libc::c_int,
                                                         movewd.offset() as
                                                             libc::c_int,
                                                         movewd.data);
                                        }
                                    }
                                }
                            }
                            220 => {
                                let mut movemem: Gmovemem = (*ptr).movemem;
                                let mut vp: *mut Vp_t = addr as *mut Vp_t;
                                match movemem.index() as libc::c_int {
                                    8 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPViewport(0x%08x(0x%08x)),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movemem.data, vp);
                                        }
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"\t# vscale=[%d %d %d %d], \x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         (*vp).vscale[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int,
                                                         (*vp).vscale[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int,
                                                         (*vp).vscale[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int,
                                                         (*vp).vscale[3 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int);
                                        }
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"vtrans=[%d %d %d %d] \x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         (*vp).vtrans[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int,
                                                         (*vp).vtrans[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int,
                                                         (*vp).vtrans[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int,
                                                         (*vp).vtrans[3 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                             as libc::c_int);
                                        }
                                    }
                                    14 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPForceMatrix(0x%08x),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movemem.data);
                                        }
                                        ptr =
                                            ptr.offset(1 as libc::c_int as
                                                           isize)
                                    }
                                    10 => {
                                        match movemem.offset() as libc::c_int
                                                  * 8 as libc::c_int {
                                            0 => {
                                                if (*this).enableLog != 0 {
                                                    osSyncPrintf(b"gsSPLookAtX(0x%08x),\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 movemem.data);
                                                }
                                            }
                                            24 => {
                                                if (*this).enableLog != 0 {
                                                    osSyncPrintf(b"gsSPLookAtY(0x%08x),\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 movemem.data);
                                                }
                                            }
                                            _ => {
                                                if (*this).enableLog != 0 {
                                                    osSyncPrintf(b"gsSPLight(0x%08x,%d),\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 movemem.data,
                                                                 (movemem.offset()
                                                                      as
                                                                      libc::c_int
                                                                      *
                                                                      8 as
                                                                          libc::c_int
                                                                      -
                                                                      24 as
                                                                          libc::c_int)
                                                                     /
                                                                     24 as
                                                                         libc::c_int);
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsMoveMem(0x%08x, %d, %d, %d),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movemem.data,
                                                         ((movemem.size() as
                                                               libc::c_int >>
                                                               3 as
                                                                   libc::c_int)
                                                              +
                                                              1 as
                                                                  libc::c_int)
                                                             *
                                                             8 as libc::c_int,
                                                         movemem.index() as
                                                             libc::c_int,
                                                         movemem.offset() as
                                                             libc::c_int *
                                                             8 as
                                                                 libc::c_int);
                                        }
                                    }
                                }
                            }
                            _ => {
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"AnyDisplayList(),\x00" as
                                                     *const u8 as
                                                     *const libc::c_char);
                                }
                            }
                        }
                    }
                    3 => {
                        match cmd as libc::c_int {
                            10 => {
                                let mut words: Gwords = (*ptr).words;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPBgRectCopy(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 words.w1, addr);
                                }
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            9 => {
                                let mut words_0: Gwords = (*ptr).words;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPBgRect1Cyc(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 words_0.w1, addr);
                                }
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            2 => {
                                let mut words_1: Gwords = (*ptr).words;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjSprite(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 words_1.w1, addr);
                                }
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            1 => {
                                let mut words_2: Gwords = (*ptr).words;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjRectangle(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 words_2.w1, addr);
                                }
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            218 => {
                                let mut words_3: Gwords = (*ptr).words;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjRectangleR(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 words_3.w1, addr);
                                }
                                (*this).pipeSyncRequired =
                                    1 as libc::c_int as u32_0
                            }
                            228 => {
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"RDPHALF_0(0x%02x, 0x%08x, 0x%04x),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*curGfx.as_mut_ptr()).dma.par()
                                                     as libc::c_int,
                                                 (*curGfx.as_mut_ptr()).dma.addr,
                                                 (*curGfx.as_mut_ptr()).dma.len()
                                                     as libc::c_int);
                                }
                                sid =
                                    (*curGfx.as_mut_ptr()).dma.par() as u8_0;
                                rdpHalf = (*curGfx.as_mut_ptr()).dma.addr;
                                linkDlLow =
                                    (*curGfx.as_mut_ptr()).dma.len() as u16_0
                            }
                            220 => {
                                let mut dma_1: Gdma = (*ptr).dma;
                                if dma_1.par() as libc::c_int ==
                                       23 as libc::c_int {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPObjMatrix(0x%08x(0x%08x)),\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     dma_1.addr, addr);
                                    }
                                } else if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjSubMatrix(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 dma_1.addr, addr);
                                }
                            }
                            5 => {
                                let mut dma_2: Gdma = (*ptr).dma;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjLoadTxtr(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 dma_2.addr, addr);
                                }
                            }
                            6 => {
                                let mut dma_3: Gdma = (*ptr).dma;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjLoadTxSprite(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 dma_3.addr, addr);
                                }
                            }
                            7 => {
                                let mut dma_4: Gdma = (*ptr).dma;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjLoadTxRect(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 dma_4.addr, addr);
                                }
                            }
                            8 => {
                                let mut dma_5: Gdma = (*ptr).dma;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjLoadTxRectR(0x%08x(0x%08x)),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 dma_5.addr, addr);
                                }
                            }
                            4 => {
                                let mut dma_6: Gdma = (*ptr).dma;
                                let mut dlAddr: u32_0 =
                                    UCodeDisas_TranslateAddr(this,
                                                             ((dma_6.len() as
                                                                   libc::c_int)
                                                                  <<
                                                                  16 as
                                                                      libc::c_int
                                                                  |
                                                                  linkDlLow as
                                                                      libc::c_int)
                                                                 as u32_0);
                                let mut dmaAddr: u32_0 = dma_6.addr;
                                if dma_6.par() as libc::c_int ==
                                       0 as libc::c_int {
                                    if (*this).enableLog != 0 {
                                        osSyncPrintf(b"gsSPSelectDL(0x%08x, %d, 0x%08x, 0x%08x),\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     dlAddr,
                                                     sid as libc::c_int,
                                                     rdpHalf, dmaAddr);
                                    }
                                } else if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPSelectBranchDL(0x%08x, %d, 0x%08x, 0x%08x),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 dlAddr, sid as libc::c_int,
                                                 rdpHalf, dmaAddr);
                                }
                            }
                            219 => {
                                let mut pad_1: [u32_0; 2] = [0; 2];
                                let mut movewd_0: Gmovewd = (*ptr).movewd;
                                match movewd_0.index() as libc::c_int {
                                    6 => {
                                        let mut segId: u32_0 =
                                            (movewd_0.offset() as libc::c_int
                                                 / 4 as libc::c_int) as u32_0;
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPSegment(%d, 0x%08x),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         segId,
                                                         movewd_0.data);
                                        }
                                        (*this).segments[segId as usize] =
                                            movewd_0.data &
                                                0xffffff as libc::c_int as
                                                    libc::c_uint
                                    }
                                    8 => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsSPSetStatus(0x%08x, 0x%08x),\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movewd_0.offset() as
                                                             libc::c_int,
                                                         movewd_0.data);
                                        }
                                    }
                                    _ => {
                                        if (*this).enableLog != 0 {
                                            osSyncPrintf(b"gsMoveWd(%d, %d, %d), \x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         movewd_0.index() as
                                                             libc::c_int,
                                                         movewd_0.offset() as
                                                             libc::c_int,
                                                         movewd_0.data);
                                        }
                                    }
                                }
                            }
                            11 => {
                                let mut dma_7: Gdma = (*ptr).dma;
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"gsSPObjRenderMode(0x%08x),\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 dma_7.addr);
                                }
                            }
                            _ => {
                                if (*this).enableLog != 0 {
                                    osSyncPrintf(b"AnyDisplayList(),\x00" as
                                                     *const u8 as
                                                     *const libc::c_char);
                                }
                            }
                        }
                    }
                    _ => { }
                }
            }
        }
        if (*this).enableLog != 0 {
            osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        ptr = ptr.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_RegisterUCode(mut this: *mut UCodeDisas,
                                                  mut count: s32,
                                                  mut ucodeArray:
                                                      *mut UCodeInfo) {
    (*this).ucodeInfoCount = count;
    (*this).ucodeInfo = ucodeArray;
}
#[no_mangle]
pub unsafe extern "C" fn UCodeDisas_SetCurUCode(mut this: *mut UCodeDisas,
                                                mut ptr: *mut libc::c_void) {
    UCodeDisas_SetCurUCodeImpl(this, ptr);
}
