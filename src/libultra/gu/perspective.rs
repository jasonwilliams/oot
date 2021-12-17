#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn cosf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn guMtxIdentF(mf: *mut [f32_0; 4]);
    #[no_mangle]
    fn guMtxF2L(m1: *mut MtxF, m2: *mut Mtx);
}
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type f32_0 = libc::c_float;
pub type f64_0 = libc::c_double;
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
pub unsafe extern "C" fn guPerspectiveF(mut mf: *mut [f32_0; 4],
                                        mut perspNorm: *mut u16_0,
                                        mut fovy: f32_0, mut aspect: f32_0,
                                        mut near: f32_0, mut far: f32_0,
                                        mut scale: f32_0) {
    let mut yscale: f32_0 = 0.;
    let mut row: s32 = 0;
    let mut col: s32 = 0;
    guMtxIdentF(mf);
    fovy = (fovy as libc::c_double * (3.1415926f64 / 180.0f64)) as f32_0;
    yscale =
        cosf(fovy / 2 as libc::c_int as libc::c_float) /
            sinf(fovy / 2 as libc::c_int as libc::c_float);
    (*mf.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        yscale / aspect;
    (*mf.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        yscale;
    (*mf.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (near + far) / (near - far);
    (*mf.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -(1 as libc::c_int) as f32_0;
    (*mf.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        2 as libc::c_int as libc::c_float * near * far / (near - far);
    (*mf.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    row = 0 as libc::c_int;
    while row < 4 as libc::c_int {
        col = 0 as libc::c_int;
        while col < 4 as libc::c_int {
            let ref mut fresh0 = (*mf.offset(row as isize))[col as usize];
            *fresh0 *= scale;
            col += 1
        }
        row += 1
    }
    if !perspNorm.is_null() {
        if (near + far) as libc::c_double <= 2.0f64 {
            *perspNorm = 65535 as libc::c_int as u16_0
        } else {
            *perspNorm =
                (((1 as libc::c_int) << 17 as libc::c_int) as f64_0 /
                     (near + far) as libc::c_double) as u16_0;
            if *perspNorm as libc::c_int <= 0 as libc::c_int {
                *perspNorm = 1 as libc::c_int as u16_0
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn guPerspective(mut m: *mut Mtx,
                                       mut perspNorm: *mut u16_0,
                                       mut fovy: f32_0, mut aspect: f32_0,
                                       mut near: f32_0, mut far: f32_0,
                                       mut scale: f32_0) {
    let mut mf: [[f32_0; 4]; 4] = [[0.; 4]; 4];
    guPerspectiveF(mf.as_mut_ptr(), perspNorm, fovy, aspect, near, far,
                   scale);
    guMtxF2L(mf.as_mut_ptr() as *mut MtxF, m);
}
