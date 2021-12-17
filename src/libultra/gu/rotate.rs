#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn guMtxIdentF(mf: *mut [f32_0; 4]);
    #[no_mangle]
    fn cosf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn guNormalize(x: *mut f32_0, y: *mut f32_0, z: *mut f32_0);
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
pub unsafe extern "C" fn guRotateF(mut m: *mut [f32_0; 4], mut a: f32_0,
                                   mut x: f32_0, mut y: f32_0, mut z: f32_0) {
    static mut D_80134D10: f32_0 = 3.14159265358979323846f32 / 180.0f32;
    let mut sine: f32_0 = 0.;
    let mut cosine: f32_0 = 0.;
    let mut ab: f32_0 = 0.;
    let mut bc: f32_0 = 0.;
    let mut ca: f32_0 = 0.;
    let mut t: f32_0 = 0.;
    let mut xs: f32_0 = 0.;
    let mut ys: f32_0 = 0.;
    let mut zs: f32_0 = 0.;
    guNormalize(&mut x, &mut y, &mut z);
    a = a * D_80134D10;
    sine = sinf(a);
    cosine = cosf(a);
    ab = x * y * (1 as libc::c_int as libc::c_float - cosine);
    bc = y * z * (1 as libc::c_int as libc::c_float - cosine);
    ca = z * x * (1 as libc::c_int as libc::c_float - cosine);
    guMtxIdentF(m);
    xs = x * sine;
    ys = y * sine;
    zs = z * sine;
    t = x * x;
    (*m.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (1 as libc::c_int as libc::c_float - t) * cosine + t;
    (*m.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        bc - xs;
    (*m.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        bc + xs;
    t = y * y;
    (*m.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (1 as libc::c_int as libc::c_float - t) * cosine + t;
    (*m.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        ca + ys;
    (*m.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        ca - ys;
    t = z * z;
    (*m.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (1 as libc::c_int as libc::c_float - t) * cosine + t;
    (*m.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        ab - zs;
    (*m.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        ab + zs;
}
#[no_mangle]
pub unsafe extern "C" fn guRotate(mut m: *mut Mtx, mut a: f32_0, mut x: f32_0,
                                  mut y: f32_0, mut z: f32_0) {
    let mut mf: [[f32_0; 4]; 4] = [[0.; 4]; 4];
    guRotateF(mf.as_mut_ptr(), a, x, y, z);
    guMtxF2L(mf.as_mut_ptr() as *mut MtxF, m);
}
