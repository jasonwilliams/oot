#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn Camera_Normal0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Normal1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Normal2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Normal3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Normal4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Parallel0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Parallel1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Parallel2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Parallel3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Parallel4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_KeepOn0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_KeepOn1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_KeepOn2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_KeepOn3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_KeepOn4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Subj0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Subj1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Subj2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Subj3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Subj4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Jump0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Jump1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Jump2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Jump3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Jump4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Battle0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Battle1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Battle2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Battle3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Battle4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Fixed0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Fixed1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Fixed2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Fixed3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Fixed4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Data0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Data1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Data2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Data3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Data4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique5(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique6(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique7(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique8(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Unique9(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo5(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo6(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo7(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo8(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Demo9(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special0(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special1(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special2(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special3(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special4(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special5(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special6(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special7(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special8(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_Special9(camera: *mut Camera) -> s32;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
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
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub f_odd: f32_0,
    pub f_even: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadContext {
    pub at: u64_0,
    pub v0: u64_0,
    pub v1: u64_0,
    pub a0: u64_0,
    pub a1: u64_0,
    pub a2: u64_0,
    pub a3: u64_0,
    pub t0: u64_0,
    pub t1: u64_0,
    pub t2: u64_0,
    pub t3: u64_0,
    pub t4: u64_0,
    pub t5: u64_0,
    pub t6: u64_0,
    pub t7: u64_0,
    pub s0: u64_0,
    pub s1: u64_0,
    pub s2: u64_0,
    pub s3: u64_0,
    pub s4: u64_0,
    pub s5: u64_0,
    pub s6: u64_0,
    pub s7: u64_0,
    pub t8: u64_0,
    pub t9: u64_0,
    pub gp: u64_0,
    pub sp: u64_0,
    pub s8: u64_0,
    pub ra: u64_0,
    pub lo: u64_0,
    pub hi: u64_0,
    pub sr: u32_0,
    pub pc: u32_0,
    pub cause: u32_0,
    pub badvaddr: u32_0,
    pub rcp: u32_0,
    pub fpcsr: u32_0,
    pub fp0: __OSfp,
    pub fp2: __OSfp,
    pub fp4: __OSfp,
    pub fp6: __OSfp,
    pub fp8: __OSfp,
    pub fp10: __OSfp,
    pub fp12: __OSfp,
    pub fp14: __OSfp,
    pub fp16: __OSfp,
    pub fp18: __OSfp,
    pub fp20: __OSfp,
    pub fp22: __OSfp,
    pub fp24: __OSfp,
    pub fp26: __OSfp,
    pub fp28: __OSfp,
    pub fp30: __OSfp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadprofile {
    pub flag: u32_0,
    pub count: u32_0,
    pub time: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSThread {
    pub next: *mut OSThread,
    pub priority: OSPri,
    pub queue: *mut *mut OSThread,
    pub tlnext: *mut OSThread,
    pub state: u16_0,
    pub flags: u16_0,
    pub id: OSId,
    pub fp: s32,
    pub thprof: *mut __OSThreadprofile,
    pub context: __OSThreadContext,
}
pub type OSMesg = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSMesgQueue {
    pub mtqueue: *mut OSThread,
    pub fullqueue: *mut OSThread,
    pub validCount: s32,
    pub first: s32,
    pub msgCount: s32,
    pub msg: *mut OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSTask_t {
    pub type_0: u32_0,
    pub flags: u32_0,
    pub ucode_boot: *mut u64_0,
    pub ucode_boot_size: u32_0,
    pub ucode: *mut u64_0,
    pub ucode_size: u32_0,
    pub ucode_data: *mut u64_0,
    pub ucode_data_size: u32_0,
    pub dram_stack: *mut u64_0,
    pub dram_stack_size: u32_0,
    pub output_buff: *mut u64_0,
    pub output_buff_size: *mut u64_0,
    pub data_ptr: *mut u64_0,
    pub data_size: u32_0,
    pub yield_data_ptr: *mut u64_0,
    pub yield_data_size: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OSTask {
    pub t: OSTask_t,
    pub force_structure_alignment: libc::c_longlong,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSContPad {
    pub button: u16_0,
    pub stick_x: s8,
    pub stick_y: s8,
    pub errno: u8_0,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vp {
    pub vp: Vp_t,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Light_t {
    pub col: [libc::c_uchar; 3],
    pub pad1: libc::c_char,
    pub colc: [libc::c_uchar; 3],
    pub pad2: libc::c_char,
    pub dir: [libc::c_schar; 3],
    pub pad3: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ambient_t {
    pub col: [libc::c_uchar; 3],
    pub pad1: libc::c_char,
    pub colc: [libc::c_uchar; 3],
    pub pad2: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Light {
    pub l: Light_t,
    pub force_structure_alignment: [libc::c_longlong; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Ambient {
    pub l: Ambient_t,
    pub force_structure_alignment: [libc::c_longlong; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lightsn {
    pub a: Ambient,
    pub l: [Light; 7],
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
pub struct Vec3us {
    pub x: u16_0,
    pub y: u16_0,
    pub z: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3s {
    pub x: s16,
    pub y: s16,
    pub z: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3i {
    pub x: s32,
    pub y: s32,
    pub z: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sphere16 {
    pub center: Vec3s,
    pub radius: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cylinder16 {
    pub radius: s16,
    pub height: s16,
    pub yShift: s16,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Linef {
    pub a: Vec3f,
    pub b: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VecSph {
    pub r: f32_0,
    pub pitch: s16,
    pub yaw: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color_RGB8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Color_RGBA8_u32 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color_RGBAf {
    pub r: f32_0,
    pub g: f32_0,
    pub b: f32_0,
    pub a: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightPoint {
    pub x: s16,
    pub y: s16,
    pub z: s16,
    pub color: [u8_0; 3],
    pub drawGlow: u8_0,
    pub radius: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightDirectional {
    pub x: s8,
    pub y: s8,
    pub z: s8,
    pub color: [u8_0; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LightParams {
    pub point: LightPoint,
    pub dir: LightDirectional,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightInfo {
    pub type_0: u8_0,
    pub params: LightParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lights {
    pub numLights: u8_0,
    pub l: Lightsn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightNode {
    pub info: *mut LightInfo,
    pub prev: *mut LightNode,
    pub next: *mut LightNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightContext {
    pub listHead: *mut LightNode,
    pub ambientColor: [u8_0; 3],
    pub fogColor: [u8_0; 3],
    pub fogNear: s16,
    pub fogFar: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlobalContext {
    pub state: GameState,
    pub sceneNum: s16,
    pub sceneConfig: u8_0,
    pub unk_A7: [libc::c_char; 9],
    pub sceneSegment: *mut libc::c_void,
    pub view: View,
    pub mainCamera: Camera,
    pub subCameras: [Camera; 3],
    pub cameraPtrs: [*mut Camera; 4],
    pub activeCamera: s16,
    pub nextCamera: s16,
    pub sequenceCtx: SequenceContext,
    pub lightCtx: LightContext,
    pub frameAdvCtx: FrameAdvanceContext,
    pub colCtx: CollisionContext,
    pub actorCtx: ActorContext,
    pub csCtx: CutsceneContext,
    pub soundSources: [SoundSource; 16],
    pub sramCtx: SramContext,
    pub skyboxCtx: SkyboxContext,
    pub msgCtx: MessageContext,
    pub interfaceCtx: InterfaceContext,
    pub pauseCtx: PauseContext,
    pub gameOverCtx: GameOverContext,
    pub envCtx: EnvironmentContext,
    pub animationCtx: AnimationContext,
    pub objectCtx: ObjectContext,
    pub roomCtx: RoomContext,
    pub transiActorCtx: TransitionActorContext,
    pub playerInit: Option<unsafe extern "C" fn(_: *mut Player,
                                                _: *mut GlobalContext,
                                                _: *mut FlexSkeletonHeader)
                               -> ()>,
    pub playerUpdate: Option<unsafe extern "C" fn(_: *mut Player,
                                                  _: *mut GlobalContext,
                                                  _: *mut Input) -> ()>,
    pub isPlayerDroppingFish: Option<unsafe extern "C" fn(_:
                                                              *mut GlobalContext)
                                         -> s32>,
    pub startPlayerFishing: Option<unsafe extern "C" fn(_: *mut GlobalContext)
                                       -> s32>,
    pub grabPlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                _: *mut Player) -> s32>,
    pub startPlayerCutscene: Option<unsafe extern "C" fn(_:
                                                             *mut GlobalContext,
                                                         _: *mut Actor,
                                                         _: s32) -> s32>,
    pub func_11D54: Option<unsafe extern "C" fn(_: *mut Player,
                                                _: *mut GlobalContext) -> ()>,
    pub damagePlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                  _: s32) -> s32>,
    pub talkWithPlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                    _: *mut Actor) -> ()>,
    pub viewProjectionMtxF: MtxF,
    pub billboardMtxF: MtxF,
    pub billboardMtx: *mut Mtx,
    pub gameplayFrames: u32_0,
    pub linkAgeOnLoad: u8_0,
    pub unk_11DE9: u8_0,
    pub curSpawn: u8_0,
    pub numSetupActors: u8_0,
    pub numRooms: u8_0,
    pub roomList: *mut RomFile,
    pub linkActorEntry: *mut ActorEntry,
    pub setupActorList: *mut ActorEntry,
    pub unk_11DFC: *mut libc::c_void,
    pub setupEntranceList: *mut EntranceEntry,
    pub setupExitList: *mut s16,
    pub setupPathList: *mut Path,
    pub cUpElfMsgs: *mut ElfMessage,
    pub specialEffects: *mut libc::c_void,
    pub skyboxId: u8_0,
    pub sceneLoadFlag: s8,
    pub unk_11E16: s16,
    pub unk_11E18: s16,
    pub nextEntranceIndex: s16,
    pub unk_11E1C: [libc::c_char; 64],
    pub shootingGalleryStatus: s8,
    pub bombchuBowlingStatus: s8,
    pub fadeTransition: u8_0,
    pub colChkCtx: CollisionCheckContext,
    pub envFlags: [u16_0; 20],
    pub pauseBgPreRender: PreRender,
    pub unk_12174: [libc::c_char; 83],
    pub unk_121C7: s8,
    pub transitionCtx: TransitionContext,
    pub unk_12418: [libc::c_char; 3],
    pub transitionMode: u8_0,
    pub transitionFade: TransitionFade,
    pub unk_12428: [libc::c_char; 3],
    pub unk_1242B: u8_0,
    pub loadedScene: *mut SceneTableEntry,
    pub unk_12430: [libc::c_char; 232],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SceneTableEntry {
    pub sceneFile: RomFile,
    pub titleFile: RomFile,
    pub unk_10: u8_0,
    pub config: u8_0,
    pub unk_12: u8_0,
    pub unk_13: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RomFile {
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionFade {
    pub fadeType: u8_0,
    pub isDone: u8_0,
    pub fadeDirection: u8_0,
    pub fadeColor: Color_RGBA8_u32,
    pub fadeTimer: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionContext {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub transitionType: s32,
    pub init: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                         -> *mut libc::c_void>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub update: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                           -> ()>,
    pub draw: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut *mut Gfx) -> ()>,
    pub start: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub setType: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                            -> ()>,
    pub setColor: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0)
                             -> ()>,
    pub setEnvColor: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: u32_0) -> ()>,
    pub isDone: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> s32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub fade: TransitionFade,
    pub circle: TransitionCircle,
    pub triforce: TransitionTriforce,
    pub wipe: TransitionWipe,
    pub data: [libc::c_char; 552],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionWipe {
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub direction: u8_0,
    pub frame: u8_0,
    pub isDone: u8_0,
    pub texX: u16_0,
    pub texY: u16_0,
    pub normal: u16_0,
    pub projection: Mtx,
    pub lookAt: Mtx,
    pub modelView: [[Mtx; 3]; 2],
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
pub struct CollisionCheckContext {
    pub colATCount: s16,
    pub sacFlags: u16_0,
    pub colAT: [*mut Collider; 50],
    pub colACCount: s32,
    pub colAC: [*mut Collider; 60],
    pub colOCCount: s32,
    pub colOC: [*mut Collider; 50],
    pub colLineCount: s32,
    pub colLine: [*mut OcLine; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcLine {
    pub line: Linef,
    pub ocFlags: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Collider {
    pub actor: *mut Actor,
    pub at: *mut Actor,
    pub ac: *mut Actor,
    pub oc: *mut Actor,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
    pub ocFlags2: u8_0,
    pub colType: u8_0,
    pub shape: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Actor {
    pub id: s16,
    pub category: u8_0,
    pub room: s8,
    pub flags: u32_0,
    pub home: PosRot,
    pub params: s16,
    pub objBankIndex: s8,
    pub targetMode: s8,
    pub sfx: u16_0,
    pub world: PosRot,
    pub focus: PosRot,
    pub targetArrowOffset: f32_0,
    pub scale: Vec3f,
    pub velocity: Vec3f,
    pub speedXZ: f32_0,
    pub gravity: f32_0,
    pub minVelocityY: f32_0,
    pub wallPoly: *mut CollisionPoly,
    pub floorPoly: *mut CollisionPoly,
    pub wallBgId: u8_0,
    pub floorBgId: u8_0,
    pub wallYaw: s16,
    pub floorHeight: f32_0,
    pub yDistToWater: f32_0,
    pub bgCheckFlags: u16_0,
    pub yawTowardsPlayer: s16,
    pub xyzDistToPlayerSq: f32_0,
    pub xzDistToPlayer: f32_0,
    pub yDistToPlayer: f32_0,
    pub colChkInfo: CollisionCheckInfo,
    pub shape: ActorShape,
    pub projectedPos: Vec3f,
    pub projectedW: f32_0,
    pub uncullZoneForward: f32_0,
    pub uncullZoneScale: f32_0,
    pub uncullZoneDownward: f32_0,
    pub prevPos: Vec3f,
    pub isTargeted: u8_0,
    pub targetPriority: u8_0,
    pub textId: u16_0,
    pub freezeTimer: u16_0,
    pub colorFilterParams: u16_0,
    pub colorFilterTimer: u8_0,
    pub isDrawn: u8_0,
    pub dropFlag: u8_0,
    pub naviEnemyId: u8_0,
    pub parent: *mut Actor,
    pub child: *mut Actor,
    pub prev: *mut Actor,
    pub next: *mut Actor,
    pub init: ActorFunc,
    pub destroy: ActorFunc,
    pub update: ActorFunc,
    pub draw: ActorFunc,
    pub overlayEntry: *mut ActorOverlay,
    pub dbgPad: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorOverlay {
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
    pub vramStart: *mut libc::c_void,
    pub vramEnd: *mut libc::c_void,
    pub loadedRamAddr: *mut libc::c_void,
    pub initInfo: *mut ActorInit,
    pub name: *mut libc::c_char,
    pub allocType: u16_0,
    pub numLoaded: s8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorInit {
    pub id: s16,
    pub category: u8_0,
    pub flags: u32_0,
    pub objectId: s16,
    pub instanceSize: u32_0,
    pub init: ActorFunc,
    pub destroy: ActorFunc,
    pub update: ActorFunc,
    pub draw: ActorFunc,
}
pub type ActorFunc
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorShape {
    pub rot: Vec3s,
    pub face: s16,
    pub yOffset: f32_0,
    pub shadowDraw: ActorShadowFunc,
    pub shadowScale: f32_0,
    pub shadowAlpha: u8_0,
    pub feetFloorFlags: u8_0,
    pub feetPos: [Vec3f; 2],
}
pub type ActorShadowFunc
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut Lights,
                                _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckInfo {
    pub damageTable: *mut DamageTable,
    pub displacement: Vec3f,
    pub cylRadius: s16,
    pub cylHeight: s16,
    pub cylYShift: s16,
    pub mass: u8_0,
    pub health: u8_0,
    pub damage: u8_0,
    pub damageEffect: u8_0,
    pub atHitEffect: u8_0,
    pub acHitEffect: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DamageTable {
    pub table: [u8_0; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionPoly {
    pub type_0: u16_0,
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub flags_vIA: u16_0,
    pub flags_vIB: u16_0,
    pub vIC: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosRot {
    pub pos: Vec3f,
    pub rot: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ElfMessage {
    pub byte0: u8_0,
    pub byte1: u8_0,
    pub byte2: u8_0,
    pub byte3: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Path {
    pub count: u8_0,
    pub points: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntranceEntry {
    pub spawn: u8_0,
    pub room: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorEntry {
    pub id: s16,
    pub pos: Vec3s,
    pub rot: Vec3s,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Player {
    pub actor: Actor,
    pub currentTunic: s8,
    pub currentSword: s8,
    pub currentShield: s8,
    pub currentBoots: s8,
    pub heldItemButton: s8,
    pub heldItemActionParam: s8,
    pub heldItemId: u8_0,
    pub prevBoots: s8,
    pub itemActionParam: s8,
    pub unk_155: [libc::c_char; 3],
    pub modelGroup: u8_0,
    pub nextModelGroup: u8_0,
    pub unk_15A: s8,
    pub modelAnimType: u8_0,
    pub leftHandType: u8_0,
    pub rightHandType: u8_0,
    pub sheathType: u8_0,
    pub currentMask: u8_0,
    pub rightHandDLists: *mut *mut Gfx,
    pub leftHandDLists: *mut *mut Gfx,
    pub sheathDLists: *mut *mut Gfx,
    pub waistDLists: *mut *mut Gfx,
    pub giObjectLoading: u8_0,
    pub giObjectDmaRequest: DmaRequest,
    pub giObjectLoadQueue: OSMesgQueue,
    pub giObjectLoadMsg: OSMesg,
    pub giObjectSegment: *mut libc::c_void,
    pub skelAnime: SkelAnime,
    pub jointTable: [Vec3s; 24],
    pub morphTable: [Vec3s; 24],
    pub blendTable: [Vec3s; 24],
    pub unk_3A8: [s16; 2],
    pub heldActor: *mut Actor,
    pub leftHandPos: Vec3f,
    pub unk_3BC: Vec3s,
    pub unk_3C4: *mut Actor,
    pub unk_3C8: Vec3f,
    pub unk_3D4: [libc::c_char; 88],
    pub doorType: s8,
    pub doorDirection: s8,
    pub doorTimer: s16,
    pub doorActor: *mut Actor,
    pub getItemId: s8,
    pub getItemDirection: u16_0,
    pub interactRangeActor: *mut Actor,
    pub mountSide: s8,
    pub unk_43D: [libc::c_char; 3],
    pub rideActor: *mut Actor,
    pub csMode: u8_0,
    pub prevCsMode: u8_0,
    pub unk_446: u8_0,
    pub unk_447: u8_0,
    pub unk_448: *mut Actor,
    pub unk_44C: [libc::c_char; 4],
    pub unk_450: Vec3f,
    pub unk_45C: Vec3f,
    pub unk_468: [libc::c_char; 2],
    pub unk_46A: s16,
    pub unk_46C: s16,
    pub unk_46E: [libc::c_char; 42],
    pub cylinder: ColliderCylinder,
    pub swordQuads: [ColliderQuad; 2],
    pub shieldQuad: ColliderQuad,
    pub unk_664: *mut Actor,
    pub unk_668: [libc::c_char; 4],
    pub unk_66C: s32,
    pub swordEffectIndex: s32,
    pub func_674: PlayerFunc674,
    pub ageProperties: *mut PlayerAgeProperties,
    pub stateFlags1: u32_0,
    pub stateFlags2: u32_0,
    pub unk_684: *mut Actor,
    pub boomerangActor: *mut Actor,
    pub naviActor: *mut Actor,
    pub naviTextId: s16,
    pub stateFlags3: u8_0,
    pub exchangeItemId: s8,
    pub targetActor: *mut Actor,
    pub targetActorDistance: f32_0,
    pub unk_69C: [libc::c_char; 4],
    pub unk_6A0: f32_0,
    pub unk_6A4: f32_0,
    pub unk_6A8: *mut Actor,
    pub unk_6AC: s8,
    pub unk_6AD: u8_0,
    pub unk_6AE: u16_0,
    pub unk_6B0: s16,
    pub unk_6B4: [libc::c_char; 4],
    pub unk_6B6: s16,
    pub unk_6B8: s16,
    pub unk_6BA: s16,
    pub unk_6BC: s16,
    pub unk_6BE: s16,
    pub unk_6C0: s16,
    pub unk_6C2: s16,
    pub unk_6C4: f32_0,
    pub skelAnime2: SkelAnime,
    pub jointTable2: [Vec3s; 24],
    pub morphTable2: [Vec3s; 24],
    pub func_82C: PlayerFunc82C,
    pub unk_830: f32_0,
    pub unk_834: s16,
    pub unk_836: s8,
    pub unk_837: u8_0,
    pub linearVelocity: f32_0,
    pub currentYaw: s16,
    pub targetYaw: s16,
    pub unk_840: u16_0,
    pub swordAnimation: s8,
    pub swordState: s8,
    pub unk_844: s8,
    pub unk_845: u8_0,
    pub unk_846: u8_0,
    pub unk_847: [s8; 4],
    pub unk_84B: [s8; 4],
    pub unk_84F: s8,
    pub unk_850: s16,
    pub unk_854: f32_0,
    pub unk_858: f32_0,
    pub unk_85C: f32_0,
    pub unk_860: s16,
    pub unk_862: s8,
    pub unk_864: f32_0,
    pub unk_868: f32_0,
    pub unk_86C: f32_0,
    pub unk_870: f32_0,
    pub unk_874: f32_0,
    pub unk_878: f32_0,
    pub unk_87C: s16,
    pub unk_87E: s16,
    pub unk_880: f32_0,
    pub wallHeight: f32_0,
    pub wallDistance: f32_0,
    pub unk_88C: u8_0,
    pub unk_88D: u8_0,
    pub unk_88E: u8_0,
    pub unk_88F: u8_0,
    pub unk_890: u8_0,
    pub shockTimer: u8_0,
    pub unk_892: u8_0,
    pub hoverBootsTimer: u8_0,
    pub fallStartHeight: s16,
    pub fallDistance: s16,
    pub unk_898: s16,
    pub unk_89A: s16,
    pub unk_89C: s16,
    pub unk_89E: u16_0,
    pub unk_8A0: u8_0,
    pub unk_8A1: u8_0,
    pub unk_8A2: s16,
    pub unk_8A4: f32_0,
    pub unk_8A8: f32_0,
    pub windSpeed: f32_0,
    pub windDirection: s16,
    pub swordInfo: [WeaponInfo; 3],
    pub bodyPartsPos: [Vec3f; 18],
    pub mf_9E0: MtxF,
    pub shieldMf: MtxF,
    pub isBurning: u8_0,
    pub flameTimers: [u8_0; 18],
    pub unk_A73: u8_0,
    pub func_A74: PlayerFuncA74,
    pub invincibilityTimer: s8,
    pub unk_A79: u8_0,
    pub unk_A7A: u8_0,
    pub unk_A7B: u8_0,
    pub unk_A7C: f32_0,
    pub unk_A80: s16,
    pub unk_A82: u16_0,
    pub unk_A84: s16,
    pub unk_A86: s8,
    pub unk_A87: u8_0,
    pub unk_A88: Vec3f,
}
pub type PlayerFuncA74
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut Player) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WeaponInfo {
    pub active: s32,
    pub tip: Vec3f,
    pub base: Vec3f,
}
pub type PlayerFunc82C
    =
    Option<unsafe extern "C" fn(_: *mut Player, _: *mut GlobalContext)
               -> s32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelAnime {
    pub limbCount: u8_0,
    pub mode: u8_0,
    pub dListCount: u8_0,
    pub taper: s8,
    pub skeleton: *mut *mut libc::c_void,
    pub animation: *mut libc::c_void,
    pub startFrame: f32_0,
    pub endFrame: f32_0,
    pub animLength: f32_0,
    pub curFrame: f32_0,
    pub playSpeed: f32_0,
    pub jointTable: *mut Vec3s,
    pub morphTable: *mut Vec3s,
    pub morphWeight: f32_0,
    pub morphRate: f32_0,
    pub update: Option<unsafe extern "C" fn() -> s32>,
    pub initFlags: s8,
    pub moveFlags: u8_0,
    pub prevRot: s16,
    pub prevTransl: Vec3s,
    pub baseTransl: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerAgeProperties {
    pub unk_00: f32_0,
    pub unk_04: f32_0,
    pub unk_08: f32_0,
    pub unk_0C: f32_0,
    pub unk_10: f32_0,
    pub unk_14: f32_0,
    pub unk_18: f32_0,
    pub unk_1C: f32_0,
    pub unk_20: f32_0,
    pub unk_24: f32_0,
    pub unk_28: f32_0,
    pub unk_2C: f32_0,
    pub unk_30: f32_0,
    pub unk_34: f32_0,
    pub unk_38: f32_0,
    pub unk_3C: f32_0,
    pub unk_40: f32_0,
    pub unk_44: Vec3s,
    pub unk_4A: [Vec3s; 4],
    pub unk_62: [Vec3s; 4],
    pub unk_7A: [Vec3s; 2],
    pub unk_86: [Vec3s; 2],
    pub unk_92: u16_0,
    pub unk_94: u16_0,
    pub unk_98: *mut LinkAnimationHeader,
    pub unk_9C: *mut LinkAnimationHeader,
    pub unk_A0: *mut LinkAnimationHeader,
    pub unk_A4: *mut LinkAnimationHeader,
    pub unk_A8: *mut LinkAnimationHeader,
    pub unk_AC: [*mut LinkAnimationHeader; 4],
    pub unk_BC: [*mut LinkAnimationHeader; 2],
    pub unk_C4: [*mut LinkAnimationHeader; 2],
    pub unk_CC: [*mut LinkAnimationHeader; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkAnimationHeader {
    pub common: AnimationHeaderCommon,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationHeaderCommon {
    pub frameCount: s16,
}
pub type PlayerFunc674
    =
    Option<unsafe extern "C" fn(_: *mut Player, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuad {
    pub base: Collider,
    pub info: ColliderInfo,
    pub dim: ColliderQuadDim,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuadDim {
    pub quad: [Vec3f; 4],
    pub dcMid: Vec3s,
    pub baMid: Vec3s,
    pub acDist: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderInfo {
    pub toucher: ColliderTouch,
    pub bumper: ColliderBump,
    pub elemType: u8_0,
    pub toucherFlags: u8_0,
    pub bumperFlags: u8_0,
    pub ocElemFlags: u8_0,
    pub atHit: *mut Collider,
    pub acHit: *mut Collider,
    pub atHitInfo: *mut ColliderInfo,
    pub acHitInfo: *mut ColliderInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderBump {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub defense: u8_0,
    pub hitPos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTouch {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub damage: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderCylinder {
    pub base: Collider,
    pub info: ColliderInfo,
    pub dim: Cylinder16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DmaRequest {
    pub vromAddr: u32_0,
    pub dramAddr: *mut libc::c_void,
    pub size: u32_0,
    pub filename: *const libc::c_char,
    pub line: s32,
    pub unk_14: s32,
    pub notifyQueue: *mut OSMesgQueue,
    pub notifyMsg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FlexSkeletonHeader {
    pub sh: SkeletonHeader,
    pub dListCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkeletonHeader {
    pub segment: *mut *mut libc::c_void,
    pub limbCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionActorContext {
    pub numActors: u8_0,
    pub list: *mut TransitionActorEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionActorEntry {
    pub sides: [C2RustUnnamed_6; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub room: s8,
    pub effects: s8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RoomContext {
    pub curRoom: Room,
    pub prevRoom: Room,
    pub bufPtrs: [*mut libc::c_void; 2],
    pub unk_30: u8_0,
    pub status: s8,
    pub unk_34: *mut libc::c_void,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub unk_74: [s16; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Room {
    pub num: s8,
    pub unk_01: u8_0,
    pub unk_02: u8_0,
    pub unk_03: u8_0,
    pub echo: s8,
    pub showInvisActors: u8_0,
    pub mesh: *mut Mesh,
    pub segment: *mut libc::c_void,
    pub unk_10: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Mesh {
    pub polygon: Polygon,
    pub polygon0: PolygonType0,
    pub polygon1: PolygonType1,
    pub polygon2: PolygonType2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType2 {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType1 {
    pub type_0: u8_0,
    pub format: u8_0,
    pub dlist: *mut Gfx,
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub single: C2RustUnnamed_9,
    pub multi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub count: u8_0,
    pub list: *mut BgImage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgImage {
    pub unk_00: u16_0,
    pub id: u8_0,
    pub source: u32_0,
    pub unk_0C: u32_0,
    pub tlut: u32_0,
    pub width: u16_0,
    pub height: u16_0,
    pub fmt: u8_0,
    pub siz: u8_0,
    pub mode0: u16_0,
    pub tlutCount: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub source: *mut libc::c_void,
    pub unk_0C: u32_0,
    pub tlut: *mut libc::c_void,
    pub width: u16_0,
    pub height: u16_0,
    pub fmt: u8_0,
    pub siz: u8_0,
    pub mode0: u16_0,
    pub tlutCount: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType0 {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Polygon {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectContext {
    pub spaceStart: *mut libc::c_void,
    pub spaceEnd: *mut libc::c_void,
    pub num: u8_0,
    pub unk_09: u8_0,
    pub mainKeepIndex: u8_0,
    pub subKeepIndex: u8_0,
    pub status: [ObjectStatus; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectStatus {
    pub id: s16,
    pub segment: *mut libc::c_void,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationContext {
    pub animationCount: s16,
    pub entries: [AnimationEntry; 50],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationEntry {
    pub type_0: u8_0,
    pub data: AnimationEntryData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union AnimationEntryData {
    pub load: AnimEntryLoadFrame,
    pub copy: AnimEntryCopyAll,
    pub interp: AnimEntryInterp,
    pub copy1: AnimEntryCopyTrue,
    pub copy0: AnimEntryCopyFalse,
    pub move_0: AnimEntryMoveActor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryMoveActor {
    pub actor: *mut Actor,
    pub skelAnime: *mut SkelAnime,
    pub unk_08: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyFalse {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
    pub copyFlag: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyTrue {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
    pub copyFlag: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryInterp {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub base: *mut Vec3s,
    pub mod_0: *mut Vec3s,
    pub weight: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyAll {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryLoadFrame {
    pub req: DmaRequest,
    pub msgQueue: OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnvironmentContext {
    pub unk_00: [libc::c_char; 2],
    pub timeIncrement: u16_0,
    pub sunPos: Vec3f,
    pub skybox1Index: u8_0,
    pub skybox2Index: u8_0,
    pub unk_12: [libc::c_char; 1],
    pub skyboxBlend: u8_0,
    pub unk_14: [libc::c_char; 1],
    pub skyboxDisabled: u8_0,
    pub sunMoonDisabled: u8_0,
    pub unk_17: u8_0,
    pub unk_18: u8_0,
    pub unk_19: u8_0,
    pub unk_1A: u16_0,
    pub unk_1C: [libc::c_char; 2],
    pub indoors: u8_0,
    pub unk_1F: u8_0,
    pub unk_20: u8_0,
    pub unk_21: u8_0,
    pub unk_22: u16_0,
    pub unk_24: u16_0,
    pub unk_26: [libc::c_char; 2],
    pub dirLight1: LightInfo,
    pub dirLight2: LightInfo,
    pub skyboxDmaState: s8,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub unk_84: f32_0,
    pub unk_88: f32_0,
    pub adjAmbientColor: [s16; 3],
    pub adjLight1Color: [s16; 3],
    pub adjFogColor: [s16; 3],
    pub adjFogNear: s16,
    pub adjFogFar: s16,
    pub unk_A2: [libc::c_char; 6],
    pub windDirection: Vec3s,
    pub windSpeed: f32_0,
    pub numLightSettings: u8_0,
    pub lightSettingsList: *mut EnvLightSettings,
    pub blendIndoorLights: u8_0,
    pub unk_BD: u8_0,
    pub unk_BE: u8_0,
    pub unk_BF: u8_0,
    pub lightSettings: EnvLightSettings,
    pub unk_D6: u16_0,
    pub unk_D8: f32_0,
    pub unk_DC: u8_0,
    pub gloomySkyMode: u8_0,
    pub unk_DE: u8_0,
    pub lightningMode: u8_0,
    pub unk_E0: u8_0,
    pub fillScreen: u8_0,
    pub screenFillColor: [u8_0; 4],
    pub sandstormState: u8_0,
    pub sandstormPrimA: u8_0,
    pub sandstormEnvA: u8_0,
    pub customSkyboxFilter: u8_0,
    pub skyboxFilterColor: [u8_0; 4],
    pub unk_EE: [u8_0; 4],
    pub unk_F2: [u8_0; 4],
    pub unk_F6: [libc::c_char; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnvLightSettings {
    pub ambientColor: [u8_0; 3],
    pub light1Dir: [s8; 3],
    pub light1Color: [u8_0; 3],
    pub light2Dir: [s8; 3],
    pub light2Color: [u8_0; 3],
    pub fogColor: [u8_0; 3],
    pub fogNear: s16,
    pub fogFar: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameOverContext {
    pub state: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PauseContext {
    pub view: View,
    pub iconItemSegment: *mut u8_0,
    pub iconItem24Segment: *mut u8_0,
    pub iconItemAltSegment: *mut u8_0,
    pub iconItemLangSegment: *mut u8_0,
    pub nameSegment: *mut u8_0,
    pub playerSegment: *mut u8_0,
    pub unk_140: [libc::c_char; 4],
    pub itemPageVtx: *mut Vtx,
    pub equipPageVtx: *mut Vtx,
    pub mapPageVtx: *mut Vtx,
    pub questPageVtx: *mut Vtx,
    pub infoPanelVtx: *mut Vtx,
    pub itemVtx: *mut Vtx,
    pub equipVtx: *mut Vtx,
    pub unk_160: [libc::c_char; 4],
    pub questVtx: *mut Vtx,
    pub cursorVtx: *mut Vtx,
    pub saveVtx: *mut Vtx,
    pub unk_170: [libc::c_char; 36],
    pub ocarinaStaff: *mut OcarinaStaff,
    pub unk_198: [libc::c_char; 32],
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub state: u16_0,
    pub debugState: u16_0,
    pub eye: Vec3f,
    pub unk_1E4: u16_0,
    pub mode: u16_0,
    pub pageIndex: u16_0,
    pub unk_1EA: u16_0,
    pub unk_1EC: u16_0,
    pub unk_1F0: f32_0,
    pub unk_1F4: f32_0,
    pub unk_1F8: f32_0,
    pub unk_1FC: f32_0,
    pub unk_200: f32_0,
    pub unk_204: f32_0,
    pub alpha: u16_0,
    pub offsetY: s16,
    pub unk_20C: [libc::c_char; 8],
    pub stickRelX: s16,
    pub stickRelY: s16,
    pub cursorPoint: [s16; 5],
    pub cursorX: [s16; 5],
    pub cursorY: [s16; 5],
    pub dungeonMapSlot: s16,
    pub cursorSpecialPos: s16,
    pub pageSwitchTimer: s16,
    pub namedItem: u16_0,
    pub cursorItem: [u16_0; 4],
    pub cursorSlot: [u16_0; 4],
    pub equipTargetItem: u16_0,
    pub equipTargetSlot: u16_0,
    pub equipTargetCBtn: u16_0,
    pub equipAnimX: s16,
    pub equipAnimY: s16,
    pub equipAnimAlpha: s16,
    pub infoPanelOffsetY: s16,
    pub nameDisplayTimer: u16_0,
    pub nameColorSet: u16_0,
    pub cursorColorSet: s16,
    pub promptChoice: s16,
    pub ocarinaSongIdx: s16,
    pub worldMapPoints: [u8_0; 20],
    pub tradeQuestLocation: u8_0,
    pub playerSkelAnime: SkelAnime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcarinaStaff {
    pub noteIdx: u8_0,
    pub state: u8_0,
    pub pos: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct View {
    pub magic: s32,
    pub gfxCtx: *mut GraphicsContext,
    pub viewport: Viewport,
    pub fovy: f32_0,
    pub zNear: f32_0,
    pub zFar: f32_0,
    pub scale: f32_0,
    pub eye: Vec3f,
    pub lookAt: Vec3f,
    pub up: Vec3f,
    pub vp: Vp,
    pub projection: Mtx,
    pub viewing: Mtx,
    pub projectionPtr: *mut Mtx,
    pub viewingPtr: *mut Mtx,
    pub unk_E8: Vec3f,
    pub unk_F4: Vec3f,
    pub unk_100: f32_0,
    pub unk_104: Vec3f,
    pub unk_110: Vec3f,
    pub normal: u16_0,
    pub flags: s32,
    pub unk_124: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Viewport {
    pub topY: s32,
    pub bottomY: s32,
    pub leftX: s32,
    pub rightX: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GraphicsContext {
    pub polyOpaBuffer: *mut Gfx,
    pub polyXluBuffer: *mut Gfx,
    pub unk_008: [libc::c_char; 8],
    pub overlayBuffer: *mut Gfx,
    pub unk_014: u32_0,
    pub unk_018: [libc::c_char; 32],
    pub msgBuff: [OSMesg; 8],
    pub schedMsgQ: *mut OSMesgQueue,
    pub queue: OSMesgQueue,
    pub unk_074: [libc::c_char; 4],
    pub task: OSScTask,
    pub unk_0D0: [libc::c_char; 224],
    pub workBuffer: *mut Gfx,
    pub work: TwoHeadGfxArena,
    pub unk_01C4: [libc::c_char; 192],
    pub viMode: *mut OSViMode,
    pub unk_0288: [libc::c_char; 32],
    pub overlay: TwoHeadGfxArena,
    pub polyOpa: TwoHeadGfxArena,
    pub polyXlu: TwoHeadGfxArena,
    pub gfxPoolIdx: u32_0,
    pub curFrameBuffer: *mut u16_0,
    pub unk_2E0: [libc::c_char; 4],
    pub viFeatures: u32_0,
    pub fbIdx: s32,
    pub callback: Option<unsafe extern "C" fn(_: *mut GraphicsContext,
                                              _: *mut libc::c_void) -> ()>,
    pub callbackParam: *mut libc::c_void,
    pub xScale: f32_0,
    pub yScale: f32_0,
    pub unk_2FC: [libc::c_char; 4],
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
pub struct OSScTask {
    pub next: *mut OSScTask,
    pub state: u32_0,
    pub flags: u32_0,
    pub framebuffer: *mut CfbInfo,
    pub list: OSTask,
    pub msgQ: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CfbInfo {
    pub fb1: *mut u16_0,
    pub swapBuffer: *mut u16_0,
    pub viMode: *mut OSViMode,
    pub features: u32_0,
    pub unk_10: u8_0,
    pub updateRate: s8,
    pub updateRate2: s8,
    pub unk_13: u8_0,
    pub xScale: f32_0,
    pub yScale: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterfaceContext {
    pub view: View,
    pub actionVtx: *mut Vtx,
    pub beatingHeartVtx: *mut Vtx,
    pub parameterSegment: *mut u8_0,
    pub doActionSegment: *mut u8_0,
    pub iconItemSegment: *mut u8_0,
    pub mapSegment: *mut u8_0,
    pub mapPalette: [u8_0; 32],
    pub dmaRequest_160: DmaRequest,
    pub dmaRequest_180: DmaRequest,
    pub unk_1A0: [libc::c_char; 32],
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub viewport: Viewport,
    pub unk_1EC: s16,
    pub unk_1EE: u16_0,
    pub unk_1F0: u16_0,
    pub unk_1F4: f32_0,
    pub naviCalling: s16,
    pub unk_1FA: s16,
    pub unk_1FC: s16,
    pub unk_1FE: s16,
    pub unk_200: s16,
    pub beatingHeartPrim: [s16; 3],
    pub beatingHeartEnv: [s16; 3],
    pub heartsPrimR: [s16; 2],
    pub heartsPrimG: [s16; 2],
    pub heartsPrimB: [s16; 2],
    pub heartsEnvR: [s16; 2],
    pub heartsEnvG: [s16; 2],
    pub heartsEnvB: [s16; 2],
    pub unk_226: s16,
    pub unk_228: s16,
    pub unk_22A: s16,
    pub unk_22C: s16,
    pub unk_22E: s16,
    pub unk_230: s16,
    pub counterDigits: [s16; 4],
    pub numHorseBoosts: u8_0,
    pub unk_23C: u16_0,
    pub hbaAmmo: u16_0,
    pub unk_240: u16_0,
    pub unk_242: u16_0,
    pub unk_244: u16_0,
    pub aAlpha: u16_0,
    pub bAlpha: u16_0,
    pub cLeftAlpha: u16_0,
    pub cDownAlpha: u16_0,
    pub cRightAlpha: u16_0,
    pub healthAlpha: u16_0,
    pub magicAlpha: u16_0,
    pub minimapAlpha: u16_0,
    pub startAlpha: s16,
    pub unk_258: s16,
    pub unk_25A: s16,
    pub mapRoomNum: s16,
    pub mapPaletteIndex: s16,
    pub unk_260: u8_0,
    pub unk_261: u8_0,
    pub restrictions: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub hGauge: u8_0,
    pub bButton: u8_0,
    pub aButton: u8_0,
    pub bottles: u8_0,
    pub tradeItems: u8_0,
    pub hookshot: u8_0,
    pub ocarina: u8_0,
    pub warpSongs: u8_0,
    pub sunsSong: u8_0,
    pub farores: u8_0,
    pub dinsNayrus: u8_0,
    pub all: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MessageContext {
    pub view: View,
    pub font: Font,
    pub textboxSegment: *mut libc::c_void,
    pub unk_E2B4: [libc::c_char; 4],
    pub ocarinaStaff: *mut OcarinaStaff,
    pub unk_E2BC: [libc::c_char; 60],
    pub textId: u16_0,
    pub choiceTextId: u16_0,
    pub textBoxProperties: u8_0,
    pub textBoxType: u8_0,
    pub textBoxPos: u8_0,
    pub msgLength: s32,
    pub msgMode: u8_0,
    pub unk_E305: [libc::c_char; 1],
    pub msgBufDecoded: [u8_0; 200],
    pub msgBufPos: u16_0,
    pub unk_E3D0: u16_0,
    pub textDrawPos: u16_0,
    pub decodedTextLen: u16_0,
    pub textUnskippable: u16_0,
    pub textPosX: s16,
    pub textPosY: s16,
    pub textColorR: s16,
    pub textColorG: s16,
    pub textColorB: s16,
    pub textColorAlpha: s16,
    pub textboxEndType: u8_0,
    pub choiceIndex: u8_0,
    pub choiceNum: u8_0,
    pub stateTimer: u8_0,
    pub textDelayTimer: u16_0,
    pub textDelay: u16_0,
    pub lastPlayedSong: u16_0,
    pub ocarinaMode: u16_0,
    pub ocarinaAction: u16_0,
    pub unk_E3F2: u16_0,
    pub unk_E3F4: u16_0,
    pub textboxBackgroundIdx: u16_0,
    pub textboxBackgroundForeColorIdx: u8_0,
    pub textboxBackgroundBackColorIdx: u8_0,
    pub textboxBackgroundYOffsetIdx: u8_0,
    pub textboxBackgroundUnkArg: u8_0,
    pub unk_E3FC: [libc::c_char; 2],
    pub textboxColorRed: s16,
    pub textboxColorGreen: s16,
    pub textboxColorBlue: s16,
    pub textboxColorAlphaTarget: s16,
    pub textboxColorAlphaCurrent: s16,
    pub talkActor: *mut Actor,
    pub disableWarpSongs: s16,
    pub unk_E40E: s16,
    pub lastOcaNoteIdx: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub msgOffset: u32_0,
    pub msgLength: u32_0,
    pub charTexBuf: [u8_0; 15360],
    pub iconBuf: [u8_0; 128],
    pub fontBuf: [u8_0; 40960],
    pub c2rust_unnamed: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub msgBuf: [libc::c_char; 1280],
    pub msgBufWide: [u16_0; 640],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkyboxContext {
    pub unk_00: [libc::c_char; 296],
    pub staticSegments: [*mut libc::c_void; 2],
    pub palettes: *mut [u16_0; 256],
    pub dListBuf: *mut [Gfx; 150],
    pub unk_138: *mut Gfx,
    pub roomVtx: *mut Vtx,
    pub unk_140: s16,
    pub rot: Vec3f,
    pub unk_150: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SramContext {
    pub readBuff: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundSource {
    pub countdown: u16_0,
    pub originPos: Vec3f,
    pub relativePos: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CutsceneContext {
    pub unk_00: [libc::c_char; 4],
    pub segment: *mut libc::c_void,
    pub state: u8_0,
    pub unk_0C: f32_0,
    pub frames: u16_0,
    pub unk_12: u16_0,
    pub unk_14: s32,
    pub unk_18: u16_0,
    pub unk_1A: u8_0,
    pub unk_1B: u8_0,
    pub cameraFocus: *mut CutsceneCameraPoint,
    pub cameraPosition: *mut CutsceneCameraPoint,
    pub linkAction: *mut CsCmdActorAction,
    pub npcActions: [*mut CsCmdActorAction; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdActorAction {
    pub action: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub rot: Vec3s,
    pub urot: Vec3us,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CutsceneCameraPoint {
    pub continueFlag: s8,
    pub cameraRoll: s8,
    pub nextPointFrame: u16_0,
    pub viewAngle: f32_0,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorContext {
    pub freezeFlashTimer: u8_0,
    pub unk_01: [libc::c_char; 1],
    pub unk_02: u8_0,
    pub unk_03: u8_0,
    pub unk_04: [libc::c_char; 4],
    pub total: u8_0,
    pub unk_09: [libc::c_char; 3],
    pub actorLists: [ActorListEntry; 12],
    pub targetCtx: TargetContext,
    pub flags: C2RustUnnamed_13,
    pub titleCtx: TitleCardContext,
    pub unk_138: [libc::c_char; 4],
    pub absoluteSpace: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TitleCardContext {
    pub texture: *mut libc::c_void,
    pub x: s16,
    pub y: s16,
    pub width: u8_0,
    pub height: u8_0,
    pub durationTimer: u8_0,
    pub delayTimer: u8_0,
    pub alpha: s16,
    pub intensity: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub swch: u32_0,
    pub tempSwch: u32_0,
    pub unk0: u32_0,
    pub unk1: u32_0,
    pub chest: u32_0,
    pub clear: u32_0,
    pub tempClear: u32_0,
    pub collect: u32_0,
    pub tempCollect: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetContext {
    pub naviRefPos: Vec3f,
    pub targetCenterPos: Vec3f,
    pub naviInner: Color_RGBAf,
    pub naviOuter: Color_RGBAf,
    pub arrowPointedActor: *mut Actor,
    pub targetedActor: *mut Actor,
    pub unk_40: f32_0,
    pub unk_44: f32_0,
    pub unk_48: s16,
    pub activeCategory: u8_0,
    pub unk_4B: u8_0,
    pub unk_4C: s8,
    pub unk_4D: [libc::c_char; 3],
    pub arr_50: [TargetContextEntry; 3],
    pub unk_8C: *mut Actor,
    pub bgmEnemy: *mut Actor,
    pub unk_94: *mut Actor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetContextEntry {
    pub pos: Vec3f,
    pub unk_0C: f32_0,
    pub color: Color_RGB8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorListEntry {
    pub length: s32,
    pub head: *mut Actor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionContext {
    pub colHeader: *mut CollisionHeader,
    pub minBounds: Vec3f,
    pub maxBounds: Vec3f,
    pub subdivAmount: Vec3i,
    pub subdivLength: Vec3f,
    pub subdivLengthInv: Vec3f,
    pub lookupTbl: *mut StaticLookup,
    pub polyNodes: SSNodeList,
    pub dyna: DynaCollisionContext,
    pub memSize: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaCollisionContext {
    pub bitFlag: u8_0,
    pub bgActors: [BgActor; 50],
    pub bgActorFlags: [u16_0; 50],
    pub polyList: *mut CollisionPoly,
    pub vtxList: *mut Vec3s,
    pub polyNodes: DynaSSNodeList,
    pub polyNodesMax: s32,
    pub polyListMax: s32,
    pub vtxListMax: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaSSNodeList {
    pub tbl: *mut SSNode,
    pub count: s32,
    pub max: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSNode {
    pub polyId: s16,
    pub next: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgActor {
    pub actor: *mut Actor,
    pub colHeader: *mut CollisionHeader,
    pub dynaLookup: DynaLookup,
    pub vtxStartIndex: u16_0,
    pub prevTransform: ScaleRotPos,
    pub curTransform: ScaleRotPos,
    pub boundingSphere: Sphere16,
    pub minY: f32_0,
    pub maxY: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScaleRotPos {
    pub scale: Vec3f,
    pub rot: Vec3s,
    pub pos: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaLookup {
    pub polyStartIndex: u16_0,
    pub ceiling: SSList,
    pub wall: SSList,
    pub floor: SSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSList {
    pub head: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionHeader {
    pub minBounds: Vec3s,
    pub maxBounds: Vec3s,
    pub numVertices: u16_0,
    pub vtxList: *mut Vec3s,
    pub numPolygons: u16_0,
    pub polyList: *mut CollisionPoly,
    pub surfaceTypeList: *mut SurfaceType,
    pub cameraDataList: *mut CamData,
    pub numWaterBoxes: u16_0,
    pub waterBoxes: *mut WaterBox,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaterBox {
    pub xMin: s16,
    pub ySurface: s16,
    pub zMin: s16,
    pub xLength: s16,
    pub zLength: s16,
    pub properties: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CamData {
    pub cameraSType: u16_0,
    pub numCameras: s16,
    pub camPosData: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceType {
    pub data: [u32_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSNodeList {
    pub max: u16_0,
    pub count: u16_0,
    pub tbl: *mut SSNode,
    pub polyCheckTbl: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StaticLookup {
    pub floor: SSList,
    pub wall: SSList,
    pub ceiling: SSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameAdvanceContext {
    pub enabled: s32,
    pub timer: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SequenceContext {
    pub seqId: u8_0,
    pub natureAmbienceId: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Camera {
    pub paramData: [libc::c_char; 80],
    pub at: Vec3f,
    pub eye: Vec3f,
    pub up: Vec3f,
    pub eyeNext: Vec3f,
    pub skyboxOffset: Vec3f,
    pub globalCtx: *mut GlobalContext,
    pub player: *mut Player,
    pub playerPosRot: PosRot,
    pub target: *mut Actor,
    pub targetPosRot: PosRot,
    pub rUpdateRateInv: f32_0,
    pub pitchUpdateRateInv: f32_0,
    pub yawUpdateRateInv: f32_0,
    pub xzOffsetUpdateRate: f32_0,
    pub yOffsetUpdateRate: f32_0,
    pub fovUpdateRate: f32_0,
    pub xzSpeed: f32_0,
    pub dist: f32_0,
    pub speedRatio: f32_0,
    pub posOffset: Vec3f,
    pub playerPosDelta: Vec3f,
    pub fov: f32_0,
    pub atLERPStepScale: f32_0,
    pub playerGroundY: f32_0,
    pub floorNorm: Vec3f,
    pub waterYPos: f32_0,
    pub waterPrevCamIdx: s32,
    pub waterPrevCamSetting: s32,
    pub waterQuakeId: s32,
    pub data0: *mut libc::c_void,
    pub data1: *mut libc::c_void,
    pub data2: s16,
    pub data3: s16,
    pub uid: s16,
    pub unk_132: [libc::c_char; 2],
    pub inputDir: Vec3s,
    pub camDir: Vec3s,
    pub status: s16,
    pub setting: s16,
    pub mode: s16,
    pub bgCheckId: s16,
    pub camDataIdx: s16,
    pub unk_14A: s16,
    pub unk_14C: s16,
    pub childCamIdx: s16,
    pub unk_150: s16,
    pub unk_152: s16,
    pub prevSetting: s16,
    pub nextCamDataIdx: s16,
    pub nextBGCheckId: s16,
    pub roll: s16,
    pub paramFlags: s16,
    pub animState: s16,
    pub timer: s16,
    pub parentCamIdx: s16,
    pub thisIdx: s16,
    pub prevCamDataIdx: s16,
    pub csId: s16,
    pub unk_16A: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub gfxCtx: *mut GraphicsContext,
    pub main: GameStateFunc,
    pub destroy: GameStateFunc,
    pub init: GameStateFunc,
    pub size: u32_0,
    pub input: [Input; 4],
    pub tha: TwoHeadArena,
    pub alloc: GameAlloc,
    pub running: u32_0,
    pub frames: u32_0,
    pub unk_A0: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAlloc {
    pub base: GameAllocEntry,
    pub head: *mut GameAllocEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAllocEntry {
    pub next: *mut GameAllocEntry,
    pub prev: *mut GameAllocEntry,
    pub size: u32_0,
    pub unk_0C: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TwoHeadArena {
    pub size: u32_0,
    pub bufp: *mut libc::c_void,
    pub head: *mut libc::c_void,
    pub tail: *mut libc::c_void,
}
pub type GameStateFunc
    =
    Option<unsafe extern "C" fn(_: *mut GameState) -> ()>;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const CAM_SET_MAX: C2RustUnnamed_14 = 66;
pub const CAM_SET_NORMAL4: C2RustUnnamed_14 = 65;
pub const CAM_SET_PIVOT_FROM_SIDE: C2RustUnnamed_14 = 64;
pub const CAM_SET_DIRECTED_YAW: C2RustUnnamed_14 = 63;
pub const CAM_SET_DUNGEON2: C2RustUnnamed_14 = 62;
pub const CAM_SET_JABU_TENTACLE: C2RustUnnamed_14 = 61;
pub const CAM_SET_CS_C: C2RustUnnamed_14 = 60;
pub const CAM_SET_FISHING: C2RustUnnamed_14 = 59;
pub const CAM_SET_NORMAL2: C2RustUnnamed_14 = 58;
pub const CAM_SET_PIVOT_VERTICAL: C2RustUnnamed_14 = 57;
pub const CAM_SET_TURN_AROUND: C2RustUnnamed_14 = 56;
pub const CAM_SET_FIRE_BIRDS_EYE: C2RustUnnamed_14 = 55;
pub const CAM_SET_MEADOW_UNUSED: C2RustUnnamed_14 = 54;
pub const CAM_SET_MEADOW_BIRDS_EYE: C2RustUnnamed_14 = 53;
pub const CAM_SET_BIG_OCTO: C2RustUnnamed_14 = 52;
pub const CAM_SET_FOREST_DEFEAT_POE: C2RustUnnamed_14 = 51;
pub const CAM_SET_FOREST_UNUSED: C2RustUnnamed_14 = 50;
pub const CAM_SET_FIRE_STAIRCASE: C2RustUnnamed_14 = 49;
pub const CAM_SET_FIRE_PLATFORM: C2RustUnnamed_14 = 48;
pub const CAM_SET_SCENE_TRANSITION: C2RustUnnamed_14 = 47;
pub const CAM_SET_SCENE_UNUSED: C2RustUnnamed_14 = 46;
pub const CAM_SET_BEAN_LOST_WOODS: C2RustUnnamed_14 = 45;
pub const CAM_SET_BEAN_GENERIC: C2RustUnnamed_14 = 44;
pub const CAM_SET_CS_ATTENTION: C2RustUnnamed_14 = 43;
pub const CAM_SET_CS_3: C2RustUnnamed_14 = 42;
pub const CAM_SET_ITEM_UNUSED: C2RustUnnamed_14 = 41;
pub const CAM_SET_SLOW_CHEST_CS: C2RustUnnamed_14 = 40;
pub const CAM_SET_FOREST_BIRDS_EYE: C2RustUnnamed_14 = 39;
pub const CAM_SET_CS_TWISTED_HALLWAY: C2RustUnnamed_14 = 38;
pub const CAM_SET_CS_0: C2RustUnnamed_14 = 37;
pub const CAM_SET_PIVOT_WATER_SURFACE: C2RustUnnamed_14 = 36;
pub const CAM_SET_PIVOT_CORNER: C2RustUnnamed_14 = 35;
pub const CAM_SET_FREE2: C2RustUnnamed_14 = 34;
pub const CAM_SET_FREE0: C2RustUnnamed_14 = 33;
pub const CAM_SET_START1: C2RustUnnamed_14 = 32;
pub const CAM_SET_START0: C2RustUnnamed_14 = 31;
pub const CAM_SET_CRAWLSPACE: C2RustUnnamed_14 = 30;
pub const CAM_SET_DOORC: C2RustUnnamed_14 = 29;
pub const CAM_SET_DOOR0: C2RustUnnamed_14 = 28;
pub const CAM_SET_PREREND_SIDE_SCROLL: C2RustUnnamed_14 = 27;
pub const CAM_SET_PREREND_PIVET: C2RustUnnamed_14 = 26;
pub const CAM_SET_PREREND_FIXED: C2RustUnnamed_14 = 25;
pub const CAM_SET_PIVOT_IN_FRONT: C2RustUnnamed_14 = 24;
pub const CAM_SET_PIVOT_SHOP_BROWSING: C2RustUnnamed_14 = 23;
pub const CAM_SET_PIVOT_CRAWLSPACE: C2RustUnnamed_14 = 22;
pub const CAM_SET_CHU_BOWLING: C2RustUnnamed_14 = 21;
pub const CAM_SET_MARKET_BALCONY: C2RustUnnamed_14 = 20;
pub const CAM_SET_TOWER_UNUSED: C2RustUnnamed_14 = 19;
pub const CAM_SET_TOWER_CLIMB: C2RustUnnamed_14 = 18;
pub const CAM_SET_BOSS_GANON: C2RustUnnamed_14 = 17;
pub const CAM_SET_BOSS_GANONDORF: C2RustUnnamed_14 = 16;
pub const CAM_SET_BOSS_TWINROVA_FLOOR: C2RustUnnamed_14 = 15;
pub const CAM_SET_BOSS_TWINROVA_PLATFORM: C2RustUnnamed_14 = 14;
pub const CAM_SET_BOSS_MORPHA: C2RustUnnamed_14 = 13;
pub const CAM_SET_BOSS_BONGO: C2RustUnnamed_14 = 12;
pub const CAM_SET_BOSS_VOLVAGIA: C2RustUnnamed_14 = 11;
pub const CAM_SET_BOSS_PHANTOM_GANON: C2RustUnnamed_14 = 10;
pub const CAM_SET_BOSS_BARINADE: C2RustUnnamed_14 = 9;
pub const CAM_SET_BOSS_DODONGO: C2RustUnnamed_14 = 8;
pub const CAM_SET_BOSS_GOHMA: C2RustUnnamed_14 = 7;
pub const CAM_SET_HORSE: C2RustUnnamed_14 = 6;
pub const CAM_SET_NORMAL3: C2RustUnnamed_14 = 5;
pub const CAM_SET_DUNGEON1: C2RustUnnamed_14 = 4;
pub const CAM_SET_DUNGEON0: C2RustUnnamed_14 = 3;
pub const CAM_SET_NORMAL1: C2RustUnnamed_14 = 2;
pub const CAM_SET_NORMAL0: C2RustUnnamed_14 = 1;
pub const CAM_SET_NONE: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const CAM_FUNC_MAX: C2RustUnnamed_15 = 71;
pub const CAM_FUNC_SPEC9: C2RustUnnamed_15 = 70;
pub const CAM_FUNC_SPEC8: C2RustUnnamed_15 = 69;
pub const CAM_FUNC_SPEC7: C2RustUnnamed_15 = 68;
pub const CAM_FUNC_SPEC6: C2RustUnnamed_15 = 67;
pub const CAM_FUNC_SPEC5: C2RustUnnamed_15 = 66;
pub const CAM_FUNC_SPEC4: C2RustUnnamed_15 = 65;
pub const CAM_FUNC_SPEC3: C2RustUnnamed_15 = 64;
pub const CAM_FUNC_SPEC2: C2RustUnnamed_15 = 63;
pub const CAM_FUNC_SPEC1: C2RustUnnamed_15 = 62;
pub const CAM_FUNC_SPEC0: C2RustUnnamed_15 = 61;
pub const CAM_FUNC_DEMO9: C2RustUnnamed_15 = 60;
pub const CAM_FUNC_DEMO8: C2RustUnnamed_15 = 59;
pub const CAM_FUNC_DEMO7: C2RustUnnamed_15 = 58;
pub const CAM_FUNC_DEMO6: C2RustUnnamed_15 = 57;
pub const CAM_FUNC_DEMO5: C2RustUnnamed_15 = 56;
pub const CAM_FUNC_DEMO4: C2RustUnnamed_15 = 55;
pub const CAM_FUNC_DEMO3: C2RustUnnamed_15 = 54;
pub const CAM_FUNC_DEMO2: C2RustUnnamed_15 = 53;
pub const CAM_FUNC_DEMO1: C2RustUnnamed_15 = 52;
pub const CAM_FUNC_DEMO0: C2RustUnnamed_15 = 51;
pub const CAM_FUNC_UNIQ9: C2RustUnnamed_15 = 50;
pub const CAM_FUNC_UNIQ8: C2RustUnnamed_15 = 49;
pub const CAM_FUNC_UNIQ7: C2RustUnnamed_15 = 48;
pub const CAM_FUNC_UNIQ6: C2RustUnnamed_15 = 47;
pub const CAM_FUNC_UNIQ5: C2RustUnnamed_15 = 46;
pub const CAM_FUNC_UNIQ4: C2RustUnnamed_15 = 45;
pub const CAM_FUNC_UNIQ3: C2RustUnnamed_15 = 44;
pub const CAM_FUNC_UNIQ2: C2RustUnnamed_15 = 43;
pub const CAM_FUNC_UNIQ1: C2RustUnnamed_15 = 42;
pub const CAM_FUNC_UNIQ0: C2RustUnnamed_15 = 41;
pub const CAM_FUNC_DATA4: C2RustUnnamed_15 = 40;
pub const CAM_FUNC_DATA3: C2RustUnnamed_15 = 39;
pub const CAM_FUNC_DATA2: C2RustUnnamed_15 = 38;
pub const CAM_FUNC_DATA1: C2RustUnnamed_15 = 37;
pub const CAM_FUNC_DATA0: C2RustUnnamed_15 = 36;
pub const CAM_FUNC_FIXD4: C2RustUnnamed_15 = 35;
pub const CAM_FUNC_FIXD3: C2RustUnnamed_15 = 34;
pub const CAM_FUNC_FIXD2: C2RustUnnamed_15 = 33;
pub const CAM_FUNC_FIXD1: C2RustUnnamed_15 = 32;
pub const CAM_FUNC_FIXD0: C2RustUnnamed_15 = 31;
pub const CAM_FUNC_BATT4: C2RustUnnamed_15 = 30;
pub const CAM_FUNC_BATT3: C2RustUnnamed_15 = 29;
pub const CAM_FUNC_BATT2: C2RustUnnamed_15 = 28;
pub const CAM_FUNC_BATT1: C2RustUnnamed_15 = 27;
pub const CAM_FUNC_BATT0: C2RustUnnamed_15 = 26;
pub const CAM_FUNC_JUMP4: C2RustUnnamed_15 = 25;
pub const CAM_FUNC_JUMP3: C2RustUnnamed_15 = 24;
pub const CAM_FUNC_JUMP2: C2RustUnnamed_15 = 23;
pub const CAM_FUNC_JUMP1: C2RustUnnamed_15 = 22;
pub const CAM_FUNC_JUMP0: C2RustUnnamed_15 = 21;
pub const CAM_FUNC_SUBJ4: C2RustUnnamed_15 = 20;
pub const CAM_FUNC_SUBJ3: C2RustUnnamed_15 = 19;
pub const CAM_FUNC_SUBJ2: C2RustUnnamed_15 = 18;
pub const CAM_FUNC_SUBJ1: C2RustUnnamed_15 = 17;
pub const CAM_FUNC_SUBJ0: C2RustUnnamed_15 = 16;
pub const CAM_FUNC_KEEP4: C2RustUnnamed_15 = 15;
pub const CAM_FUNC_KEEP3: C2RustUnnamed_15 = 14;
pub const CAM_FUNC_KEEP2: C2RustUnnamed_15 = 13;
pub const CAM_FUNC_KEEP1: C2RustUnnamed_15 = 12;
pub const CAM_FUNC_KEEP0: C2RustUnnamed_15 = 11;
pub const CAM_FUNC_PARA4: C2RustUnnamed_15 = 10;
pub const CAM_FUNC_PARA3: C2RustUnnamed_15 = 9;
pub const CAM_FUNC_PARA2: C2RustUnnamed_15 = 8;
pub const CAM_FUNC_PARA1: C2RustUnnamed_15 = 7;
pub const CAM_FUNC_PARA0: C2RustUnnamed_15 = 6;
pub const CAM_FUNC_NORM4: C2RustUnnamed_15 = 5;
pub const CAM_FUNC_NORM3: C2RustUnnamed_15 = 4;
pub const CAM_FUNC_NORM2: C2RustUnnamed_15 = 3;
pub const CAM_FUNC_NORM1: C2RustUnnamed_15 = 2;
pub const CAM_FUNC_NORM0: C2RustUnnamed_15 = 1;
pub const CAM_FUNC_NONE: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnePointCsFull {
    pub actionFlags: u8_0,
    pub unk_01: u8_0,
    pub initFlags: s16,
    pub timerInit: s16,
    pub rollTargetInit: s16,
    pub fovTargetInit: f32_0,
    pub lerpStepScale: f32_0,
    pub atTargetInit: Vec3f,
    pub eyeTargetInit: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbCameraSub {
    pub mode: s16,
    pub nFrames: s16,
    pub nPoints: s16,
    pub unkIdx: s16,
    pub unk_08: s16,
    pub unk_0A: s16,
    pub unk_0C: s32,
    pub unk_10: [libc::c_char; 20],
    pub position: [CutsceneCameraPoint; 129],
    pub lookAt: [CutsceneCameraPoint; 129],
    pub demoCtrlMenu: s16,
    pub demoCtrlActionIdx: s16,
    pub demoCtrlToggleSwitch: s16,
    pub unk_104A: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbCamera {
    pub unk_00: s32,
    pub at: Vec3f,
    pub eye: Vec3f,
    pub unk_1C: Vec3f,
    pub unk_28: [libc::c_char; 12],
    pub unk_34: s32,
    pub unk_38: s32,
    pub unk_3C: s32,
    pub unk_40: s32,
    pub unk_44: s32,
    pub fov: f32_0,
    pub roll: s16,
    pub unk_4E: [libc::c_char; 2],
    pub rollDegrees: f32_0,
    pub unk_54: Vec3f,
    pub unk_60: Vec3f,
    pub unk_6C: Vec3f,
    pub unk_78: s16,
    pub unk_7A: s16,
    pub sub: DbCameraSub,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CameraModeValue {
    pub val: s16,
    pub param: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CameraMode {
    pub funcIdx: s16,
    pub valueCnt: s16,
    pub values: *mut CameraModeValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CameraSetting {
    pub c2rust_unnamed: C2RustUnnamed_16,
    pub cameraModes: *mut CameraMode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub unk_00: u32_0,
    pub c2rust_unnamed: C2RustUnnamed_17,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    #[bitfield(name = "unk_bit0", ty = "u32_0", bits = "0..=0")]
    #[bitfield(name = "unk_bit1", ty = "u32_0", bits = "1..=1")]
    #[bitfield(name = "validModes", ty = "u32_0", bits = "2..=31")]
    pub unk_bit0_unk_bit1_validModes: [u8; 4],
}
/*==================================================================*/
// Data
#[no_mangle]
pub static mut sOREGInit: [s16; 53] =
    [0 as libc::c_int as s16, 1 as libc::c_int as s16,
     5 as libc::c_int as s16, 5 as libc::c_int as s16,
     5 as libc::c_int as s16, 14500 as libc::c_int as s16,
     20 as libc::c_int as s16, 16 as libc::c_int as s16,
     150 as libc::c_int as s16, 25 as libc::c_int as s16,
     150 as libc::c_int as s16, 6 as libc::c_int as s16,
     10 as libc::c_int as s16, 10 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     1 as libc::c_int as s16, 100 as libc::c_int as s16,
     250 as libc::c_int as s16, 120 as libc::c_int as s16,
     80 as libc::c_int as s16, 30 as libc::c_int as s16,
     120 as libc::c_int as s16, 4 as libc::c_int as s16,
     1 as libc::c_int as s16, 50 as libc::c_int as s16,
     20 as libc::c_int as s16, 1800 as libc::c_int as s16,
     50 as libc::c_int as s16, 50 as libc::c_int as s16,
     50 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16, -(10 as libc::c_int) as s16,
     -(5460 as libc::c_int) as s16, -(9100 as libc::c_int) as s16,
     -(6 as libc::c_int) as s16, 8 as libc::c_int as s16,
     15 as libc::c_int as s16, 75 as libc::c_int as s16,
     60 as libc::c_int as s16, 12 as libc::c_int as s16,
     110 as libc::c_int as s16, 40 as libc::c_int as s16,
     50 as libc::c_int as s16, 250 as libc::c_int as s16,
     -(10 as libc::c_int) as s16, 30 as libc::c_int as s16,
     30 as libc::c_int as s16, 70 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16];
#[no_mangle]
pub static mut sOREGInitCnt: s16 = 53 as libc::c_int as s16;
#[no_mangle]
pub static mut sPREGInit: [s16; 27] =
    [-(20 as libc::c_int) as s16, 200 as libc::c_int as s16,
     300 as libc::c_int as s16, 10 as libc::c_int as s16,
     12 as libc::c_int as s16, 10 as libc::c_int as s16,
     35 as libc::c_int as s16, 60 as libc::c_int as s16,
     60 as libc::c_int as s16, 3 as libc::c_int as s16,
     0 as libc::c_int as s16, -(40 as libc::c_int) as s16,
     20 as libc::c_int as s16, 25 as libc::c_int as s16,
     45 as libc::c_int as s16, -(5 as libc::c_int) as s16,
     15 as libc::c_int as s16, 15 as libc::c_int as s16,
     20 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     6 as libc::c_int as s16, 60 as libc::c_int as s16,
     30 as libc::c_int as s16, 0 as libc::c_int as s16,
     5 as libc::c_int as s16];
#[no_mangle]
pub static mut sPREGInitCnt: s16 = 27 as libc::c_int as s16;
#[no_mangle]
pub static mut sCameraSettingNames: [[libc::c_char; 12]; 66] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"NONE      \x00\x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"NORMAL0    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"NORMAL1    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DUNGEON0   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DUNGEON1   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"NORMAL3    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"HORSE0     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_GOMA  \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_DODO  \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_BARI  \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_FGANON\x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_BAL   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_SHADES\x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_MOFA  \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_TWIN0 \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_TWIN1 \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_GANON1\x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOSS_GANON2\x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"TOWER0     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"TOWER1     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"FIXED0     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"FIXED1     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CIRCLE0    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CIRCLE2    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CIRCLE3    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"PREREND0   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"PREREND1   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"PREREND3   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DOOR0      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DOORC      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"RAIL3      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"START0     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"START1     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"FREE0      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"FREE2      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CIRCLE4    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CIRCLE5    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DEMO0      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DEMO1      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"MORI1      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"ITEM0      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"ITEM1      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DEMO3      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DEMO4      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"UFOBEAN    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"LIFTBEAN   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"SCENE0     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"SCENE1     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"HIDAN1     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"HIDAN2     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"MORI2      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"MORI3      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"TAKO       \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"SPOT05A    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"SPOT05B    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"HIDAN3     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"ITEM2      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CIRCLE6    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"NORMAL2    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"FISHING    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DEMOC      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"UO_FIBER   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"DUNGEON2   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"TEPPEN     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CIRCLE7    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"NORMAL4    \x00")]
    };
#[no_mangle]
pub static mut sCameraModeNames: [[libc::c_char; 12]; 21] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"NORMAL     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"PARALLEL   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"KEEPON     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"TALK       \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BATTLE     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CLIMB      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"SUBJECT    \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOWARROW   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOWARROWZ  \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"FOOKSHOT   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOOMERANG  \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"PACHINCO   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CLIMBZ     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"JUMP       \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"HANG       \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"HANGZ      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"FREEFALL   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"CHARGE     \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"STILL      \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"PUSHPULL   \x00"),
         *::std::mem::transmute::<&[u8; 12],
                                  &mut [libc::c_char; 12]>(b"BOOKEEPON  \x00")]
    };
#[no_mangle]
pub static mut D_8011A3A0: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A3C8: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A3F4: [CameraModeValue; 13] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 140 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2001 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A428: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3500 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A458: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A488: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A4AC: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 19 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 20 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A4D0: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(7 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 14 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 19 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 20 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A4F4: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 19 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 20 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A518: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 6 as libc::c_int as s16,
                             param: 22 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A538: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 19 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 20 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A55C: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(7 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 14 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(9 as libc::c_int) as s16,
                             param: 19 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(63 as libc::c_int) as s16,
                             param: 20 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A580: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 999 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2006 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A5A4: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A5C4: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A5E0: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A5FC: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A61C: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 2 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A638: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf003 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A660: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x206a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A68C: [CameraModeValue; 13] =
    [{
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 140 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 85 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2001 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(15 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetNorm1ModeNormVals: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetNorm1ModeParaVals: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A714: [CameraModeValue; 13] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 140 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2001 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A748: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 65 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A778: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 6 as libc::c_int as s16,
                             param: 22 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A798: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A7B8: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A7D8: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A7FC: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 999 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2006 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A820: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 2 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A83C: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A858: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A874: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf003 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A89C: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A8C4: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A8F0: [CameraModeValue; 13] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 140 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2001 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A924: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A954: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A974: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A994: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A9B8: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 999 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2006 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A9DC: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 2 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011A9F8: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AA14: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AA30: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf003 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AA58: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AA80: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3500 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AAB0: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AAD0: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AAF0: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AB14: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 999 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2006 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AB38: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 2 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AB54: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AB70: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AB8C: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf003 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011ABB4: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x206a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetNorm3ModeNormVals: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 280 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x4 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AC08: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AC34: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3500 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetNorm3ModeBoomVals: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 18 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x5 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetHrse0ModeNormVals: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 220 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 16 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x600 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetHrse0ModeParaVals: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 220 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(2 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2600 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011ACD4: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(7 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 14 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 19 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 20 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2600 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011ACF8: [CameraModeValue; 13] =
    [{
         let mut init =
             CameraModeValue{val: -(60 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 220 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2601 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(60 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AD2C: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(60 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 140 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3500 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AD5C: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x1 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AD84: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011ADB4: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011ADDC: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 160 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AE0C: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AE34: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 125 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AE64: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AE8C: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AEBC: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 16 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AEE4: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AF14: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x83 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AF3C: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2082 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AF6C: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x83 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AF94: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AFBC: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011AFEC: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B014: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B044: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B06C: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 330 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 330 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B094: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 2 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B0B0: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B0D8: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
/*start here */
#[no_mangle]
pub static mut D_8011B108: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 280 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 23 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 8 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B12C: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 280 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 23 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 8 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x80 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B150: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 270 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 23 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 8 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B174: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 270 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 23 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 6 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetFixd0ModeNormVals: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B1A8: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B1B8: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3500 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B1C8: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetCirc0ModeNormVals: [CameraModeValue; 5] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x1 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetCirc2ModeNormVals: [CameraModeValue; 3] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3f00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B1F8: [CameraModeValue; 5] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x4 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B20C: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetPR0ModeKeepTalkVals: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetPR1ModeNormVals: [CameraModeValue; 2] =
    [{
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetPR1ModeKeepVals: [CameraModeValue; 2] =
    [{
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetPreRend1ModeTalkVals: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 24 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 25 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 4 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3500 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetDoor0ModeNormVals: [CameraModeValue; 3] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3200 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetDoorCModeNormVals: [CameraModeValue; 3] =
    [{
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3202 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetDoorCModeParaVals: [CameraModeValue; 3] =
    [{
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x320a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetRail3ModeNormVals: [CameraModeValue; 6] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 2 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3200 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B270: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0x1 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetFree0ModeNormVals: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0xff00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetFree1ModeNormVals: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0xff01 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetCirc4ModeNormVals: [CameraModeValue; 5] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B290: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B2A0: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2001 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B2B0: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0x3200 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetMori1ModeNormVals: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 450 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xc as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B2E0: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0x3501 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B2E4: [CameraModeValue; 3] =
    [{
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3200 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetDemo3ModeNormVals: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0x3212 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B2F4: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B31C: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B348: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B368: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B384: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B3A0: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B3C8: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 16 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B3F0: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B41C: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B43C: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B458: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B474: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetScn0ModeNormVals: [CameraModeValue; 3] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x10a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B4A8: [CameraModeValue; 4] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 150 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x210 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B4B8: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 400 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 14 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x12 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B4E0: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B510: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 8 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x12 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B538: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x12 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B560: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x201a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B58C: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x201a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B5B8: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x12 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B5E0: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 750 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 750 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x12 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B608: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 750 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x201a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B634: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 750 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B660: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 750 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 750 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x12 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B688: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B6B0: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B6DC: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B708: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 500 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B730: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 120 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 170 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2502 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 6 as libc::c_int as s16,
                             param: 22 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetCirc6ModeNormVals: [CameraModeValue; 2] =
    [{
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3200 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B75C: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B784: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 55 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf02 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B7AC: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2f0a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B7D8: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 55 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2f02 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B808: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3f20 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B838: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 19 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 20 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 21 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B85C: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 12 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B87C: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B89C: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B8B8: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(120 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2f00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetDemoCModeNormVals: [CameraModeValue; 1] =
    [{
         let mut init =
             CameraModeValue{val: 0x3f00 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B8D8: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 300 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 26 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B900: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 160 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B930: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B958: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x200a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sSetDung2ModeBattVals: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2002 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B9B4: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B9D4: [CameraModeValue; 8] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 15 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011B9F4: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BA18: [CameraModeValue; 9] =
    [{
         let mut init =
             CameraModeValue{val: -(40 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 18 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 999 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2006 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BA3C: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BA58: [CameraModeValue; 7] =
    [{
         let mut init =
             CameraModeValue{val: -(100 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2000 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BA74: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 350 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 100 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0xf003 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BA9C: [CameraModeValue; 11] =
    [{
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 280 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 25 as libc::c_int as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 10 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x206a as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BAC8: [CameraModeValue; 10] =
    [{
         let mut init =
             CameraModeValue{val: -(10 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 280 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 320 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(8 as libc::c_int) as s16,
                             param: 3 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 5 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 6 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 60 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 80 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BAF0: [CameraModeValue; 13] =
    [{
         let mut init =
             CameraModeValue{val: -(20 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 180 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 35 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(5 as libc::c_int) as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 20 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x2001 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(50 as libc::c_int) as s16,
                             param: 11 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 12 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BB24: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(80 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 250 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(8 as libc::c_int) as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: -(8 as libc::c_int) as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 30 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x3520 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011BB54: [CameraModeValue; 12] =
    [{
         let mut init =
             CameraModeValue{val: -(30 as libc::c_int) as s16,
                             param: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 1 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 200 as libc::c_int as s16,
                             param: 2 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 40 as libc::c_int as s16,
                             param: 13 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 14 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0 as libc::c_int as s16,
                             param: 15 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 5 as libc::c_int as s16,
                             param: 16 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 70 as libc::c_int as s16,
                             param: 17 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 45 as libc::c_int as s16,
                             param: 7 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 50 as libc::c_int as s16,
                             param: 8 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 10 as libc::c_int as s16,
                             param: 4 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             CameraModeValue{val: 0x35a0 as libc::c_int as s16,
                             param: 9 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut sCamSetNorm0Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A3A0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A458.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5FC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetNorm1Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values:
                                sSetNorm1ModeNormVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values:
                                sSetNorm1ModeParaVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A714.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A748.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A7D8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A778.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A7FC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A798.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A83C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A858.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A7B8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A820.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A874.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDungeon0Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A89C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A8C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A8F0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A924.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A994.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A778.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A9B8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A954.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A9F8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011AA14.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A974.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A9DC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AA30.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDungeon1Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AA58.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A8C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A714.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AA80.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A924.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011AAF0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A778.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011AB14.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011AAB0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011AB54.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011AB70.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011AAD0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011AB38.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AB8C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011ABB4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetNorm3Modes: [CameraMode; 20] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP3 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values:
                                sSetNorm3ModeNormVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011AC08.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AC34.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A458.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP3 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values:
                                sSetNorm3ModeBoomVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetHorse0Modes: [CameraMode; 9] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values:
                                sSetHrse0ModeNormVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values:
                                sSetHrse0ModeParaVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011ACF8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AD2C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011ACD4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossGomaModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AD5C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AD84.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A9DC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossDodoModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011ADB4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011ADDC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossBariModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AE0C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AE34.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossFGanonModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AE64.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AE8C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossBalModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AEBC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AEE4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossShadesModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AF14.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AF3C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AF6C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AF6C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossMofaModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AF94.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AFBC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossTwin0Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011AFEC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B014.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossTwin1Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B044.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B014.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossGanon1Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B06C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011AE8C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011B094.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetBossGanon2Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B0B0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B0D8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetTower0Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011B108.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A458.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011B12C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetTower1Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011B150.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A458.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011B174.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetFixed0Modes: [CameraMode; 4] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD1 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values:
                                sSetFixd0ModeNormVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD1 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values: D_8011B1A8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD1 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values: D_8011B1B8.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetFixed1Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD1 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values: D_8011B1C8.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetCirc0Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD2 as libc::c_int as s16,
                            valueCnt: 5 as libc::c_int as s16,
                            values:
                                sSetCirc0ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetCirc2Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DATA4 as libc::c_int as s16,
                            valueCnt: 3 as libc::c_int as s16,
                            values:
                                sSetCirc2ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetCirc3Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD4 as libc::c_int as s16,
                            valueCnt: 5 as libc::c_int as s16,
                            values: D_8011B1F8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A748.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetPreRend0Modes: [CameraMode; 4] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD3 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B20C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD3 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values:
                                sSetPR0ModeKeepTalkVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD3 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values:
                                sSetPR0ModeKeepTalkVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetPreRend1Modes: [CameraMode; 4] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ7 as libc::c_int as s16,
                            valueCnt: 2 as libc::c_int as s16,
                            values: sSetPR1ModeNormVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ7 as libc::c_int as s16,
                            valueCnt: 2 as libc::c_int as s16,
                            values: sSetPR1ModeKeepVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP0 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values:
                                sSetPreRend1ModeTalkVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetPreRend3Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC6 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B20C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDoor0Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ3 as libc::c_int as s16,
                            valueCnt: 3 as libc::c_int as s16,
                            values:
                                sSetDoor0ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDoorCModes: [CameraMode; 2] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC9 as libc::c_int as s16,
                            valueCnt: 3 as libc::c_int as s16,
                            values:
                                sSetDoorCModeNormVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC9 as libc::c_int as s16,
                            valueCnt: 3 as libc::c_int as s16,
                            values:
                                sSetDoorCModeParaVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetRail3Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ4 as libc::c_int as s16,
                            valueCnt: 6 as libc::c_int as s16,
                            values:
                                sSetRail3ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetStart0Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ0 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B20C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetStart1Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ0 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B270.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetFree0Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ6 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values:
                                sSetFree0ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetFree1Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ6 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values:
                                sSetFree1ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetCirc4Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD2 as libc::c_int as s16,
                            valueCnt: 5 as libc::c_int as s16,
                            values:
                                sSetCirc4ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetCirc5Modes: [CameraMode; 2] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ2 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values: D_8011B290.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ2 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values: D_8011B2A0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDemo0Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DEMO1 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B2B0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDemo1Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DEMO2 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B2B0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetMori1Modes: [CameraMode; 4] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values:
                                sSetMori1ModeNormVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA3 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B2E0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetItem0Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DEMO3 as libc::c_int as s16,
                            valueCnt: 3 as libc::c_int as s16,
                            values: D_8011B2E4.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetItem1Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DEMO4 as libc::c_int as s16,
                            valueCnt: 3 as libc::c_int as s16,
                            values: D_8011B2E4.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDemo3Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DEMO9 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values:
                                sSetDemo3ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDemo4Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DEMO5 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B2B0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetUFOBeanModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B2F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B31C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A748.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011B348.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011B368.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011B384.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B3A0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetLiftBeanModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B3F0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A748.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011B41C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011B43C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011B458.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B474.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetScene0Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC9 as libc::c_int as s16,
                            valueCnt: 3 as libc::c_int as s16,
                            values: sSetScn0ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetScene1Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ2 as libc::c_int as s16,
                            valueCnt: 4 as libc::c_int as s16,
                            values: D_8011B4A8.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetHidan1Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC7 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B20C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetHidan2Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC4 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B2B0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetMori2Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ5 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B2B0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetMori3Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_DEMO6 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values: D_8011B2B0.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetTakoModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B4B8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A8C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A8F0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B4E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A994.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A778.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A9B8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A954.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A9F8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011AA14.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A974.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A9DC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B510.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetSpot05AModes: [CameraMode; 6] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B560.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: 0 as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: 0 as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B58C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B5B8.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetSpot05BModes: [CameraMode; 6] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B608.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: 0 as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: 0 as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B634.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B660.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetHidan3Modes: [CameraMode; 6] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B688.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B6B0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: 0 as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: 0 as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B6DC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B708.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetItem2Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP4 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011B730.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetCirc6Modes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC0 as libc::c_int as s16,
                            valueCnt: 2 as libc::c_int as s16,
                            values:
                                sSetCirc6ModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetNorm2Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B75C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A748.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetFishingModes: [CameraMode; 17] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B784.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B7AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B7D8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B808.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B7D8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011B838.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011B85C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011B89C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011B8B8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011B87C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDemoCModes: [CameraMode; 1] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ9 as libc::c_int as s16,
                            valueCnt: 1 as libc::c_int as s16,
                            values:
                                sSetDemoCModeNormVals.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetUOFiberModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B8D8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011B900.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetDungeon2Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B930.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011B958.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A8F0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values:
                                sSetDung2ModeBattVals.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011B9F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A778.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011BA18.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011B9B4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011BA3C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011BA58.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011B9D4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011BA74.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011BA9C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetTeppenModes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011BAC8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011BAF0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011BB24.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A458.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetCirc7Modes: [CameraMode; 12] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD4 as libc::c_int as s16,
                            valueCnt: 5 as libc::c_int as s16,
                            values: D_8011B1F8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_FIXD4 as libc::c_int as s16,
                            valueCnt: 5 as libc::c_int as s16,
                            values: D_8011B1F8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A428.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NONE as libc::c_int as s16,
                            valueCnt: 0 as libc::c_int as s16,
                            values:
                                0 as *const CameraModeValue as
                                    *mut CameraModeValue,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCamSetNorm4Modes: [CameraMode; 21] =
    unsafe {
        [{
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011B75C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A3C8.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A3F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP3 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011BB54.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT1 as libc::c_int as s16,
                            valueCnt: 12 as libc::c_int as s16,
                            values: D_8011A748.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A488.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4AC.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4D0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A4F4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SPEC5 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A518.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A538.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_SUBJ3 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A55C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP2 as libc::c_int as s16,
                            valueCnt: 9 as libc::c_int as s16,
                            values: D_8011A580.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5C4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_UNIQ1 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A5E0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_JUMP1 as libc::c_int as s16,
                            valueCnt: 8 as libc::c_int as s16,
                            values: D_8011A5A4.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_BATT4 as libc::c_int as s16,
                            valueCnt: 7 as libc::c_int as s16,
                            values: D_8011A61C.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_NORM1 as libc::c_int as s16,
                            valueCnt: 10 as libc::c_int as s16,
                            values: D_8011A638.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_PARA1 as libc::c_int as s16,
                            valueCnt: 11 as libc::c_int as s16,
                            values: D_8011A660.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraMode{funcIdx: CAM_FUNC_KEEP1 as libc::c_int as s16,
                            valueCnt: 13 as libc::c_int as s16,
                            values: D_8011A68C.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCameraSettings: [CameraSetting; 66] =
    unsafe {
        [{
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0 as libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   0 as *const CameraMode as
                                       *mut CameraMode,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetNorm0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetNorm1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetDungeon0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetDungeon1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x50ff7ff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetNorm3Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x8500018f as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetHorse0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossGomaModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossDodoModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossBariModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossFGanonModes.as_ptr() as
                                       *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossBalModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossShadesModes.as_ptr() as
                                       *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossMofaModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossTwin0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossTwin1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossGanon1Modes.as_ptr() as
                                       *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetBossGanon2Modes.as_ptr() as
                                       *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x851fffff as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetTower0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x851fffff as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetTower1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x8500000d as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetFixed0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x85000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetFixed1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x85000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetCirc0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x85000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetCirc2Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x851e1fff as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetCirc3Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x8c00000d as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetPreRend0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x8c00000d as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetPreRend1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x8c000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetPreRend3Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xc5000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetDoor0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xc5000003 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetDoorCModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xc5000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetRail3Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xc5000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetStart0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xc5000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetStart1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetFree0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetFree1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x85000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetCirc4Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000003 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetCirc5Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xce000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetDemo0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x4e000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetDemo1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000009 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetMori1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetItem0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetItem1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetDemo3Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetDemo4Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x451fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetUFOBeanModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x451fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetLiftBeanModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xc5000001 as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetScene0Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetScene1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetHidan1Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetHidan2Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetMori2Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetMori3Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x451fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetTakoModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000033 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetSpot05AModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000033 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetSpot05BModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000033 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetHidan3Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x4a000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetItem2Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x5000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetCirc6Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetNorm2Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x501e05f as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetFishingModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x45000001 as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetDemoCModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetUOFiberModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetDungeon2Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetTeppenModes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0xc5000ecd as
                                                            libc::c_uint,},
                               cameraModes:
                                   sCamSetCirc7Modes.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 CameraSetting{c2rust_unnamed:
                                   C2RustUnnamed_16{unk_00:
                                                        0x51fffff as
                                                            libc::c_int as
                                                            u32_0,},
                               cameraModes:
                                   sCamSetNorm4Modes.as_ptr() as *mut _,};
             init
         }]
    };
#[no_mangle]
pub static mut sCameraFunctions:
           [Option<unsafe extern "C" fn(_: *mut Camera) -> s32>; 71] =
    unsafe {
        [None,
         Some(Camera_Normal0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Normal1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Normal2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Normal3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Normal4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Parallel0 as
                  unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Parallel1 as
                  unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Parallel2 as
                  unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Parallel3 as
                  unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Parallel4 as
                  unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_KeepOn0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_KeepOn1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_KeepOn2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_KeepOn3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_KeepOn4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Subj0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Subj1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Subj2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Subj3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Subj4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Jump0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Jump1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Jump2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Jump3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Jump4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Battle0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Battle1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Battle2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Battle3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Battle4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Fixed0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Fixed1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Fixed2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Fixed3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Fixed4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Data0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Data1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Data2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Data3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Data4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique5 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique6 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique7 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique8 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Unique9 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo5 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo6 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo7 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo8 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Demo9 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special0 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special1 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special2 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special3 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special4 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special5 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special6 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special7 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special8 as unsafe extern "C" fn(_: *mut Camera) -> s32),
         Some(Camera_Special9 as unsafe extern "C" fn(_: *mut Camera) -> s32)]
    };
#[no_mangle]
pub static mut sInitRegs: s32 = 1 as libc::c_int;
#[no_mangle]
pub static mut gDbgCamEnabled: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut sDbgModeIdx: s32 = -(1 as libc::c_int);
#[no_mangle]
pub static mut sNextUID: s16 = 0 as libc::c_int as s16;
#[no_mangle]
pub static mut sCameraInterfaceFlags: s32 = 1 as libc::c_int;
#[no_mangle]
pub static mut sCameraInterfaceAlpha: s32 = 0x2 as libc::c_int;
#[no_mangle]
pub static mut sCameraShrinkWindowVal: s32 = 0x20 as libc::c_int;
#[no_mangle]
pub static mut D_8011D3AC: s32 = -(1 as libc::c_int);
#[no_mangle]
pub static mut D_8011D3B0: [s16; 14] =
    [0xaaa as libc::c_int as s16, 0xf556 as libc::c_int as s16,
     0x1555 as libc::c_int as s16, 0xeaab as libc::c_int as s16,
     0x2aaa as libc::c_int as s16, 0xd556 as libc::c_int as s16,
     0x3fff as libc::c_int as s16, 0xc001 as libc::c_int as s16,
     0x5555 as libc::c_int as s16, 0xaaab as libc::c_int as s16,
     0x6aaa as libc::c_int as s16, 0x9556 as libc::c_int as s16,
     0x7fff as libc::c_int as s16, 0 as libc::c_int as s16];
#[no_mangle]
pub static mut D_8011D3CC: [s16; 14] =
    [0 as libc::c_int as s16, 0x2c6 as libc::c_int as s16,
     0x58c as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0xfd3a as libc::c_int as s16,
     0 as libc::c_int as s16, 0x852 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0xb18 as libc::c_int as s16, 0x2c6 as libc::c_int as s16,
     0xfa74 as libc::c_int as s16, 0 as libc::c_int as s16];
#[no_mangle]
pub static mut sUpdateCameraDirection: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut D_8011D3EC: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut D_8011D3F0: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut sDemo5PrevAction12Frame: s32 = -(16 as libc::c_int);
#[no_mangle]
pub static mut sCameraFunctionNames: [[libc::c_char; 8]; 76] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"NONE   \x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"NORM0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"NORM1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"NORM2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"NORM3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"NORM4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"PARA0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"PARA1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"PARA2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"PARA3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"PARA4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"KEEP0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"KEEP1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"KEEP2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"KEEP3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"KEEP4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SUBJ0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SUBJ1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SUBJ2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SUBJ3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SUBJ4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"JUMP0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"JUMP1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"JUMP2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"JUMP3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"JUMP4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"BATT0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"BATT1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"BATT2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"BATT3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"BATT4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"FIXD0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"FIXD1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"FIXD2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"FIXD3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"FIXD4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DATA0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DATA1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DATA2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DATA3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DATA4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ5()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ6()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ7()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ8()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"UNIQ9()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO5()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO6()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO7()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO8()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"DEMO9()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC0()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC1()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC2()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC3()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC4()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC5()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC6()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC7()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC8()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"SPEC9()\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 8],
                                  &mut [libc::c_char; 8]>(b"\x00\x00\x00\x00\x00\x00\x00\x00")]
    };
#[no_mangle]
pub static mut D_8011D658: [VecSph; 4] =
    [{
         let mut init =
             VecSph{r: 50.0f32,
                    pitch: 0xee3a as libc::c_int as s16,
                    yaw: 0xd558 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             VecSph{r: 75.0f32,
                    pitch: 0 as libc::c_int as s16,
                    yaw: 0x8008 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             VecSph{r: 80.0f32,
                    pitch: 0xee3a as libc::c_int as s16,
                    yaw: 0x8008 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             VecSph{r: 15.0f32,
                    pitch: 0xee3a as libc::c_int as s16,
                    yaw: 0x8008 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub static mut D_8011D678: [Vec3f; 4] =
    [{ let mut init = Vec3f{x: 0.0f32, y: 40.0f32, z: 20.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 40.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 3.0f32, z: -3.0f32,}; init },
     {
         let mut init = Vec3f{x: 0.0f32, y: 3.0f32, z: -3.0f64 as f32_0,};
         init
     }];
/* ******************************************************
 * OnePoint initalization values for Demo5
 ********************************************************/
#[no_mangle]
pub static mut sDemo5PrevSfxFrame: s32 = -(200 as libc::c_int);
// target is player, far from eye
#[no_mangle]
pub static mut D_8011D6AC: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x303 as libc::c_int as s16,
                            timerInit: 0x13 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
// target is player close to current eye
#[no_mangle]
pub static mut D_8011D724: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2424 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -20.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 0x13 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
// target is close to player
#[no_mangle]
pub static mut D_8011D79C: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xcf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xc1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x303 as libc::c_int as s16,
                            timerInit: 0x13 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xc1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x303 as libc::c_int as s16,
                            timerInit: 0x9 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
// target is within 300 units of eye, and player is within 30 units of eye
#[no_mangle]
pub static mut D_8011D83C: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2141 as libc::c_int as s16,
                            timerInit: 0x14 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 0.2f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 10.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
// target is within 700 units of eye, angle between player/eye and target/eye is less than
// 76.9 degrees.  The x/y coordinates of the target on screen is between (21, 41) and (300, 200),
// and the player is farther than 30 units of the eye
#[no_mangle]
pub static mut D_8011D88C: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x303 as libc::c_int as s16,
                            timerInit: 0x14 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
// same as above, but the target is NOT within the screen area.
#[no_mangle]
pub static mut D_8011D8DC: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 0x14 as libc::c_int as s16,
                            rollTargetInit: 0x1 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: 10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x82 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 0x5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 5.0f32,
                                              y: 5.0f32,
                                              z: -200.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
// target is a door.
#[no_mangle]
pub static mut D_8011D954: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xc1c1 as libc::c_int as s16,
                            timerInit: 0x14 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 250.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x5b1 as libc::c_int as s16,
                            timerInit: 0x5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x82 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 0x5 as libc::c_int as s16,
                            rollTargetInit: 0x2 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: -150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
// otherwise
#[no_mangle]
pub static mut D_8011D9F4: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x504 as libc::c_int as s16,
                            timerInit: 0x14 as libc::c_int as s16,
                            rollTargetInit: 0x2 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 300.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x82 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 0x5 as libc::c_int as s16,
                            rollTargetInit: 0x2 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: -150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 0x1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
#[no_mangle]
pub static mut D_8011DA6C: [Vec3f; 4] =
    [{ let mut init = Vec3f{x: 3050.0f32, y: 700.0f32, z: 0.0f32,}; init },
     {
         let mut init = Vec3f{x: 1755.0f32, y: 3415.0f32, z: -380.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: -3120.0f32, y: 3160.0f32, z: 245.0f32,};
         init
     }, { let mut init = Vec3f{x: 0.0f32, y: -10.0f32, z: 240.0f32,}; init }];
#[no_mangle]
pub static mut D_8011DA9C: [Vec3f; 4] =
    [{ let mut init = Vec3f{x: 3160.0f32, y: 2150.0f32, z: 0.0f32,}; init },
     {
         let mut init = Vec3f{x: 1515.0f32, y: 4130.0f32, z: -835.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: -3040.0f32, y: 4135.0f32, z: 230.0f32,};
         init
     },
     { let mut init = Vec3f{x: -50.0f32, y: 600.0f32, z: -75.0f32,}; init }];
#[no_mangle]
pub static mut D_8011DACC: [f32_0; 4] =
    [1570.0f32, 3680.0f32, 3700.0f32, 395.0f32];
#[no_mangle]
pub static mut D_8011DADC: [f32_0; 4] =
    [320.0f32, 320.0f32, 320.0f32, 0.0f32];
#[no_mangle]
pub static mut D_8011DAEC: [s16; 8] =
    [-(2000 as libc::c_int) as s16, -(1000 as libc::c_int) as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16];
#[no_mangle]
pub static mut D_8011DAFC: [s16; 6] =
    [CAM_SET_NORMAL0 as libc::c_int as s16,
     CAM_SET_NORMAL1 as libc::c_int as s16,
     CAM_SET_NORMAL2 as libc::c_int as s16,
     CAM_SET_DUNGEON0 as libc::c_int as s16,
     CAM_SET_DUNGEON1 as libc::c_int as s16,
     CAM_SET_DUNGEON2 as libc::c_int as s16];
#[no_mangle]
pub static mut D_8015BD7C: *mut GlobalContext =
    0 as *const GlobalContext as *mut GlobalContext;
#[no_mangle]
pub static mut D_8015BD80: DbCamera =
    DbCamera{unk_00: 0,
             at: Vec3f{x: 0., y: 0., z: 0.,},
             eye: Vec3f{x: 0., y: 0., z: 0.,},
             unk_1C: Vec3f{x: 0., y: 0., z: 0.,},
             unk_28: [0; 12],
             unk_34: 0,
             unk_38: 0,
             unk_3C: 0,
             unk_40: 0,
             unk_44: 0,
             fov: 0.,
             roll: 0,
             unk_4E: [0; 2],
             rollDegrees: 0.,
             unk_54: Vec3f{x: 0., y: 0., z: 0.,},
             unk_60: Vec3f{x: 0., y: 0., z: 0.,},
             unk_6C: Vec3f{x: 0., y: 0., z: 0.,},
             unk_78: 0,
             unk_7A: 0,
             sub:
                 DbCameraSub{mode: 0,
                             nFrames: 0,
                             nPoints: 0,
                             unkIdx: 0,
                             unk_08: 0,
                             unk_0A: 0,
                             unk_0C: 0,
                             unk_10: [0; 20],
                             position:
                                 [CutsceneCameraPoint{continueFlag: 0,
                                                      cameraRoll: 0,
                                                      nextPointFrame: 0,
                                                      viewAngle: 0.,
                                                      pos:
                                                          Vec3s{x: 0,
                                                                y: 0,
                                                                z: 0,},};
                                     129],
                             lookAt:
                                 [CutsceneCameraPoint{continueFlag: 0,
                                                      cameraRoll: 0,
                                                      nextPointFrame: 0,
                                                      viewAngle: 0.,
                                                      pos:
                                                          Vec3s{x: 0,
                                                                y: 0,
                                                                z: 0,},};
                                     129],
                             demoCtrlMenu: 0,
                             demoCtrlActionIdx: 0,
                             demoCtrlToggleSwitch: 0,
                             unk_104A: Vec3s{x: 0, y: 0, z: 0,},},};
#[no_mangle]
pub static mut playerFloorPoly: *mut CollisionPoly =
    0 as *const CollisionPoly as *mut CollisionPoly;
