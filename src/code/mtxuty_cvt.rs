#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
                                 ptr: *mut libc::c_void,
                                 file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn guMtxL2F(m1: *mut MtxF, m2: *mut Mtx);
}
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn MtxConv_F2L(mut m1: *mut Mtx, mut m2: *mut MtxF) {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    LogUtils_CheckNullPointer(b"m1\x00" as *const u8 as *const libc::c_char,
                              m1 as *mut libc::c_void,
                              b"../mtxuty-cvt.c\x00" as *const u8 as
                                  *const libc::c_char, 31 as libc::c_int);
    LogUtils_CheckNullPointer(b"m2\x00" as *const u8 as *const libc::c_char,
                              m2 as *mut libc::c_void,
                              b"../mtxuty-cvt.c\x00" as *const u8 as
                                  *const libc::c_char, 32 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let mut value: s32 =
                ((*m2).mf[i as usize][j as usize] *
                     0x10000 as libc::c_int as libc::c_float) as s32;
            (*m1).c2rust_unnamed.intPart[i as usize][j as usize] =
                (value >> 16 as libc::c_int) as u16_0;
            (*m1).c2rust_unnamed.fracPart[i as usize][j as usize] =
                value as u16_0;
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MtxConv_L2F(mut m1: *mut MtxF, mut m2: *mut Mtx) {
    LogUtils_CheckNullPointer(b"m1\x00" as *const u8 as *const libc::c_char,
                              m1 as *mut libc::c_void,
                              b"../mtxuty-cvt.c\x00" as *const u8 as
                                  *const libc::c_char, 55 as libc::c_int);
    LogUtils_CheckNullPointer(b"m2\x00" as *const u8 as *const libc::c_char,
                              m2 as *mut libc::c_void,
                              b"../mtxuty-cvt.c\x00" as *const u8 as
                                  *const libc::c_char, 56 as libc::c_int);
    guMtxL2F(m1, m2);
}
