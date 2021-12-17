#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
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
pub struct TwoHeadGfxArena {
    pub size: u32_0,
    pub bufp: *mut Gfx,
    pub p: *mut Gfx,
    pub d: *mut Gfx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TwoHeadArena {
    pub size: u32_0,
    pub bufp: *mut libc::c_void,
    pub head: *mut libc::c_void,
    pub tail: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn THGA_Ct(mut thga: *mut TwoHeadGfxArena,
                                 mut start: *mut Gfx, mut size: u32_0) {
    THA_Ct(thga as *mut TwoHeadArena, start as *mut libc::c_void, size);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_Dt(mut thga: *mut TwoHeadGfxArena) {
    THA_Dt(thga as *mut TwoHeadArena);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_IsCrash(mut thga: *mut TwoHeadGfxArena)
 -> u32_0 {
    return THA_IsCrash(thga as *mut TwoHeadArena);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_Init(mut thga: *mut TwoHeadGfxArena) {
    THA_Init(thga as *mut TwoHeadArena);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_GetSize(mut thga: *mut TwoHeadGfxArena) -> s32 {
    return THA_GetSize(thga as *mut TwoHeadArena);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_GetHead(mut thga: *mut TwoHeadGfxArena)
 -> *mut Gfx {
    return THA_GetHead(thga as *mut TwoHeadArena) as *mut Gfx;
}
#[no_mangle]
pub unsafe extern "C" fn THGA_SetHead(mut thga: *mut TwoHeadGfxArena,
                                      mut start: *mut Gfx) {
    THA_SetHead(thga as *mut TwoHeadArena, start as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_GetTail(mut thga: *mut TwoHeadGfxArena)
 -> *mut Gfx {
    return THA_GetTail(thga as *mut TwoHeadArena) as *mut Gfx;
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocStartArray8(mut thga: *mut TwoHeadGfxArena,
                                               mut count: u32_0) -> *mut Gfx {
    return THA_AllocStart(thga as *mut TwoHeadArena,
                          count.wrapping_mul(8 as libc::c_int as
                                                 libc::c_uint)) as *mut Gfx;
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocStart8(mut thga: *mut TwoHeadGfxArena)
 -> *mut Gfx {
    return THGA_AllocStartArray8(thga, 1 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocStart8Wrapper(mut thga:
                                                     *mut TwoHeadGfxArena)
 -> *mut Gfx {
    return THGA_AllocStart8(thga);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocEnd(mut thga: *mut TwoHeadGfxArena,
                                       mut size: u32_0) -> *mut Gfx {
    return THA_AllocEnd(thga as *mut TwoHeadArena, size) as *mut Gfx;
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocEndArray64(mut thga: *mut TwoHeadGfxArena,
                                              mut count: u32_0) -> *mut Gfx {
    return THGA_AllocEnd(thga,
                         count.wrapping_mul(0x40 as libc::c_int as
                                                libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocEnd64(mut thga: *mut TwoHeadGfxArena)
 -> *mut Gfx {
    return THGA_AllocEnd(thga, 0x40 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocEndArray16(mut thga: *mut TwoHeadGfxArena,
                                              mut count: u32_0) -> *mut Gfx {
    return THGA_AllocEnd(thga,
                         count.wrapping_mul(0x10 as libc::c_int as
                                                libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn THGA_AllocEnd16(mut thga: *mut TwoHeadGfxArena)
 -> *mut Gfx {
    return THGA_AllocEnd(thga, 0x10 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn THA_GetHead(mut tha: *mut TwoHeadArena)
 -> *mut libc::c_void {
    return (*tha).head;
}
#[no_mangle]
pub unsafe extern "C" fn THA_SetHead(mut tha: *mut TwoHeadArena,
                                     mut start: *mut libc::c_void) {
    (*tha).head = start;
}
#[no_mangle]
pub unsafe extern "C" fn THA_GetTail(mut tha: *mut TwoHeadArena)
 -> *mut libc::c_void {
    return (*tha).tail;
}
#[no_mangle]
pub unsafe extern "C" fn THA_AllocStart(mut tha: *mut TwoHeadArena,
                                        mut size: u32_0)
 -> *mut libc::c_void {
    let mut start: *mut libc::c_void = (*tha).head;
    (*tha).head =
        ((*tha).head as u32_0).wrapping_add(size) as *mut libc::c_void;
    return start;
}
#[no_mangle]
pub unsafe extern "C" fn THA_AllocStart1(mut tha: *mut TwoHeadArena)
 -> *mut libc::c_void {
    return THA_AllocStart(tha, 1 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn THA_AllocEnd(mut tha: *mut TwoHeadArena,
                                      mut size: u32_0) -> *mut libc::c_void {
    let mut mask: u32_0 = 0;
    if size == 8 as libc::c_int as libc::c_uint {
        mask = !(7 as libc::c_int) as u32_0
    } else if size == 4 as libc::c_int as libc::c_uint ||
                  size == 12 as libc::c_int as libc::c_uint {
        mask = !(3 as libc::c_int) as u32_0
    } else if size == 2 as libc::c_int as libc::c_uint ||
                  size == 6 as libc::c_int as libc::c_uint ||
                  size == 10 as libc::c_int as libc::c_uint ||
                  size == 12 as libc::c_int as libc::c_uint ||
                  size == 14 as libc::c_int as libc::c_uint {
        mask = !(1 as libc::c_int) as u32_0
    } else {
        mask =
            if size >= 0x10 as libc::c_int as libc::c_uint {
                !(0xf as libc::c_int)
            } else { 0 as libc::c_int } as u32_0
    }
    (*tha).tail =
        (((*tha).tail as u32_0 & mask).wrapping_sub(size) & mask) as
            *mut libc::c_void;
    return (*tha).tail;
}
#[no_mangle]
pub unsafe extern "C" fn THA_AllocEndAlign16(mut tha: *mut TwoHeadArena,
                                             mut size: u32_0)
 -> *mut libc::c_void {
    let mut mask: u32_0 = !(0xf as libc::c_int) as u32_0;
    (*tha).tail =
        (((*tha).tail as u32_0 & mask).wrapping_sub(size) as libc::c_ulonglong
             & mask as u64_0) as *mut libc::c_void;
    return (*tha).tail;
}
#[no_mangle]
pub unsafe extern "C" fn THA_AllocEndAlign(mut tha: *mut TwoHeadArena,
                                           mut size: u32_0, mut mask: u32_0)
 -> *mut libc::c_void {
    (*tha).tail =
        (((*tha).tail as u32_0 & mask).wrapping_sub(size) & mask) as
            *mut libc::c_void;
    return (*tha).tail;
}
#[no_mangle]
pub unsafe extern "C" fn THA_GetSize(mut tha: *mut TwoHeadArena) -> s32 {
    return ((*tha).tail as u32_0).wrapping_sub((*tha).head as u32_0) as s32;
}
#[no_mangle]
pub unsafe extern "C" fn THA_IsCrash(mut tha: *mut TwoHeadArena) -> u32_0 {
    return (THA_GetSize(tha) < 0 as libc::c_int) as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn THA_Init(mut tha: *mut TwoHeadArena) {
    (*tha).head = (*tha).bufp;
    (*tha).tail =
        ((*tha).bufp as u32_0).wrapping_add((*tha).size) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn THA_Ct(mut tha: *mut TwoHeadArena,
                                mut ptr: *mut libc::c_void, mut size: u32_0) {
    (*tha).bufp = ptr;
    (*tha).size = size;
    THA_Init(tha);
}
#[no_mangle]
pub unsafe extern "C" fn THA_Dt(mut tha: *mut TwoHeadArena) {
    bzero(tha as *mut libc::c_void,
          ::std::mem::size_of::<TwoHeadArena>() as libc::c_ulong);
}
