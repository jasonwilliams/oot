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
/* *
 * guPositionF
 * Creates a rotation/parallel translation modeling matrix (floating point)
 */
#[no_mangle]
pub unsafe extern "C" fn guPositionF(mut mf: *mut [f32_0; 4], mut rot: f32_0,
                                     mut pitch: f32_0, mut yaw: f32_0,
                                     mut scale: f32_0, mut x: f32_0,
                                     mut y: f32_0, mut z: f32_0) {
    static mut D_80134D00: f32_0 =
        (3.14159265358979323846f32 as libc::c_double / 180.0f64) as f32_0;
    let mut sinr: f32_0 = 0.;
    let mut sinp: f32_0 = 0.;
    let mut sinh: f32_0 = 0.;
    let mut cosr: f32_0 = 0.;
    let mut cosp: f32_0 = 0.;
    let mut cosh: f32_0 = 0.;
    rot *= D_80134D00;
    pitch *= D_80134D00;
    yaw *= D_80134D00;
    sinr = sinf(rot);
    cosr = cosf(rot);
    sinp = sinf(pitch);
    cosp = cosf(pitch);
    sinh = sinf(yaw);
    cosh = cosf(yaw);
    (*mf.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        cosp * cosh * scale;
    (*mf.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        cosp * sinh * scale;
    (*mf.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        -sinp * scale;
    (*mf.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*mf.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (sinr * sinp * cosh - cosr * sinh) * scale;
    (*mf.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (sinr * sinp * sinh + cosr * cosh) * scale;
    (*mf.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        sinr * cosp * scale;
    (*mf.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*mf.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (cosr * sinp * cosh + sinr * sinh) * scale;
    (*mf.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (cosr * sinp * sinh - sinr * cosh) * scale;
    (*mf.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        cosr * cosp * scale;
    (*mf.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*mf.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] = x;
    (*mf.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] = y;
    (*mf.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] = z;
    (*mf.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        1.0f32;
}
/* *
 * guPosition
 * Creates a rotational/parallel translation modeling matrix (fixed point)
 */
#[no_mangle]
pub unsafe extern "C" fn guPosition(mut m: *mut Mtx, mut rot: f32_0,
                                    mut pitch: f32_0, mut yaw: f32_0,
                                    mut scale: f32_0, mut x: f32_0,
                                    mut y: f32_0, mut z: f32_0) {
    let mut mf: [[f32_0; 4]; 4] = [[0.; 4]; 4];
    guPositionF(mf.as_mut_ptr(), rot, pitch, yaw, scale, x, y, z);
    guMtxF2L(mf.as_mut_ptr() as *mut MtxF, m);
}
