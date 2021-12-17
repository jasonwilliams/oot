#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn guMtxIdentF(mf: *mut [f32_0; 4]);
    #[no_mangle]
    fn guMtxF2L(m1: *mut MtxF, m2: *mut Mtx);
}
pub type u16_0 = libc::c_ushort;
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
pub unsafe extern "C" fn guLookAtF(mut mf: *mut [f32_0; 4], mut xEye: f32_0,
                                   mut yEye: f32_0, mut zEye: f32_0,
                                   mut xAt: f32_0, mut yAt: f32_0,
                                   mut zAt: f32_0, mut xUp: f32_0,
                                   mut yUp: f32_0, mut zUp: f32_0) {
    let mut length: f32_0 = 0.;
    let mut xLook: f32_0 = 0.;
    let mut yLook: f32_0 = 0.;
    let mut zLook: f32_0 = 0.;
    let mut xRight: f32_0 = 0.;
    let mut yRight: f32_0 = 0.;
    let mut zRight: f32_0 = 0.;
    guMtxIdentF(mf);
    xLook = xAt - xEye;
    yLook = yAt - yEye;
    zLook = zAt - zEye;
    length =
        (-1.0f64 /
             sqrtf(xLook * xLook + yLook * yLook + zLook * zLook) as
                 libc::c_double) as f32_0;
    xLook *= length;
    yLook *= length;
    zLook *= length;
    xRight = yUp * zLook - zUp * yLook;
    yRight = zUp * xLook - xUp * zLook;
    zRight = xUp * yLook - yUp * xLook;
    length =
        (1.0f64 /
             sqrtf(xRight * xRight + yRight * yRight + zRight * zRight) as
                 libc::c_double) as f32_0;
    xRight *= length;
    yRight *= length;
    zRight *= length;
    xUp = yLook * zRight - zLook * yRight;
    yUp = zLook * xRight - xLook * zRight;
    zUp = xLook * yRight - yLook * xRight;
    length =
        (1.0f64 / sqrtf(xUp * xUp + yUp * yUp + zUp * zUp) as libc::c_double)
            as f32_0;
    xUp *= length;
    yUp *= length;
    zUp *= length;
    (*mf.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        xRight;
    (*mf.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        yRight;
    (*mf.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        zRight;
    (*mf.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        -(xEye * xRight + yEye * yRight + zEye * zRight);
    (*mf.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] = xUp;
    (*mf.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] = yUp;
    (*mf.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] = zUp;
    (*mf.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        -(xEye * xUp + yEye * yUp + zEye * zUp);
    (*mf.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        xLook;
    (*mf.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        yLook;
    (*mf.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        zLook;
    (*mf.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        -(xEye * xLook + yEye * yLook + zEye * zLook);
    (*mf.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0 as libc::c_int as f32_0;
    (*mf.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0 as libc::c_int as f32_0;
    (*mf.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0 as libc::c_int as f32_0;
    (*mf.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        1 as libc::c_int as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn guLookAt(mut m: *mut Mtx, mut xEye: f32_0,
                                  mut yEye: f32_0, mut zEye: f32_0,
                                  mut xAt: f32_0, mut yAt: f32_0,
                                  mut zAt: f32_0, mut xUp: f32_0,
                                  mut yUp: f32_0, mut zUp: f32_0) {
    let mut mf: [[f32_0; 4]; 4] = [[0.; 4]; 4];
    guLookAtF(mf.as_mut_ptr(), xEye, yEye, zEye, xAt, yAt, zAt, xUp, yUp,
              zUp);
    guMtxF2L(mf.as_mut_ptr() as *mut MtxF, m);
}
