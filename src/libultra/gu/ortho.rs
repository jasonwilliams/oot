#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn guMtxIdentF(mf: *mut [f32_0; 4]);
    #[no_mangle]
    fn guMtxF2L(m1: *mut MtxF, m2: *mut Mtx);
}
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn guOrthoF(mut mf: *mut [f32_0; 4], mut left: f32_0,
                                  mut right: f32_0, mut bottom: f32_0,
                                  mut top: f32_0, mut near: f32_0,
                                  mut far: f32_0, mut scale: f32_0) {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    guMtxIdentF(mf);
    (*mf.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        2 as libc::c_int as libc::c_float / (right - left);
    (*mf.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        2 as libc::c_int as libc::c_float / (top - bottom);
    (*mf.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        -(2 as libc::c_int) as libc::c_float / (far - near);
    (*mf.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        -(right + left) / (right - left);
    (*mf.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        -(top + bottom) / (top - bottom);
    (*mf.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        -(far + near) / (far - near);
    (*mf.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        1 as libc::c_int as f32_0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let ref mut fresh0 = (*mf.offset(i as isize))[j as usize];
            *fresh0 *= scale;
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn guOrtho(mut mtx: *mut Mtx, mut left: f32_0,
                                 mut right: f32_0, mut bottom: f32_0,
                                 mut top: f32_0, mut near: f32_0,
                                 mut far: f32_0, mut scale: f32_0) {
    let mut mf: [[f32_0; 4]; 4] = [[0.; 4]; 4];
    guOrthoF(mf.as_mut_ptr(), left, right, bottom, top, near, far, scale);
    guMtxF2L(mf.as_mut_ptr() as *mut MtxF, mtx);
}
