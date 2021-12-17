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
pub struct Hilite_t {
    pub x1: libc::c_int,
    pub y1: libc::c_int,
    pub x2: libc::c_int,
    pub y2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Light {
    pub l: Light_t,
    pub force_structure_alignment: [libc::c_longlong; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookAt {
    pub l: [Light; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Hilite {
    pub h: Hilite_t,
    pub force_structure_alignment: [libc::c_long; 4],
}
/* *
 * guLookAtHiliteF
 * This function creates the viewing matrix (floating point) and sets the LookAt/Hilite structures
 **/
#[no_mangle]
pub unsafe extern "C" fn guLookAtHiliteF(mut mf: *mut [f32_0; 4],
                                         mut l: *mut LookAt,
                                         mut h: *mut Hilite, mut xEye: f32_0,
                                         mut yEye: f32_0, mut zEye: f32_0,
                                         mut xAt: f32_0, mut yAt: f32_0,
                                         mut zAt: f32_0, mut xUp: f32_0,
                                         mut yUp: f32_0, mut zUp: f32_0,
                                         mut xl1: f32_0, mut yl1: f32_0,
                                         mut zl1: f32_0, mut xl2: f32_0,
                                         mut yl2: f32_0, mut zl2: f32_0,
                                         mut hiliteWidth: s32,
                                         mut hiliteHeight: s32) 
 /* size of hilite texture */
 {
    let mut length: f32_0 = 0.;
    let mut xLook: f32_0 = 0.;
    let mut yLook: f32_0 = 0.;
    let mut zLook: f32_0 = 0.;
    let mut xRight: f32_0 = 0.;
    let mut yRight: f32_0 = 0.;
    let mut zRight: f32_0 = 0.;
    let mut xHilite: f32_0 = 0.;
    let mut yHilite: f32_0 = 0.;
    let mut zHilite: f32_0 = 0.;
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
    /* hilite vectors */
    length =
        (1.0f64 / sqrtf(xl1 * xl1 + yl1 * yl1 + zl1 * zl1) as libc::c_double)
            as f32_0;
    xl1 *= length;
    yl1 *= length;
    zl1 *= length;
    xHilite = xl1 + xLook;
    yHilite = yl1 + yLook;
    zHilite = zl1 + zLook;
    length = sqrtf(xHilite * xHilite + yHilite * yHilite + zHilite * zHilite);
    if length as libc::c_double > 0.1f64 {
        length = (1.0f64 / length as libc::c_double) as f32_0;
        xHilite *= length;
        yHilite *= length;
        zHilite *= length;
        (*h).h.x1 =
            ((hiliteWidth * 4 as libc::c_int) as libc::c_float +
                 (xHilite * xRight + yHilite * yRight + zHilite * zRight) *
                     hiliteWidth as libc::c_float *
                     2 as libc::c_int as libc::c_float) as libc::c_int;
        (*h).h.y1 =
            ((hiliteHeight * 4 as libc::c_int) as libc::c_float +
                 (xHilite * xUp + yHilite * yUp + zHilite * zUp) *
                     hiliteHeight as libc::c_float *
                     2 as libc::c_int as libc::c_float) as libc::c_int
    } else {
        (*h).h.x1 = hiliteWidth * 2 as libc::c_int;
        (*h).h.y1 = hiliteHeight * 2 as libc::c_int
    }
    length =
        (1.0f64 / sqrtf(xl2 * xl2 + yl2 * yl2 + zl2 * zl2) as libc::c_double)
            as f32_0;
    xl2 *= length;
    yl2 *= length;
    zl2 *= length;
    xHilite = xl2 + xLook;
    yHilite = yl2 + yLook;
    zHilite = zl2 + zLook;
    length = sqrtf(xHilite * xHilite + yHilite * yHilite + zHilite * zHilite);
    if length as libc::c_double > 0.1f64 {
        length = (1.0f64 / length as libc::c_double) as f32_0;
        xHilite *= length;
        yHilite *= length;
        zHilite *= length;
        (*h).h.x2 =
            ((hiliteWidth * 4 as libc::c_int) as libc::c_float +
                 (xHilite * xRight + yHilite * yRight + zHilite * zRight) *
                     hiliteWidth as libc::c_float *
                     2 as libc::c_int as libc::c_float) as libc::c_int;
        (*h).h.y2 =
            ((hiliteHeight * 4 as libc::c_int) as libc::c_float +
                 (xHilite * xUp + yHilite * yUp + zHilite * zUp) *
                     hiliteHeight as libc::c_float *
                     2 as libc::c_int as libc::c_float) as libc::c_int
    } else {
        (*h).h.x2 = hiliteWidth * 2 as libc::c_int;
        (*h).h.y2 = hiliteHeight * 2 as libc::c_int
    }
    /* reflectance vectors = Up and Right */
    (*l).l[0 as libc::c_int as usize].l.dir[0 as libc::c_int as usize] =
        ((if xRight * 128.0f32 < 127.0f32 {
              (xRight) * 128.0f32
          } else { 127.0f32 }) as s32 & 0xff as libc::c_int) as libc::c_schar;
    (*l).l[0 as libc::c_int as usize].l.dir[1 as libc::c_int as usize] =
        ((if yRight * 128.0f32 < 127.0f32 {
              (yRight) * 128.0f32
          } else { 127.0f32 }) as s32 & 0xff as libc::c_int) as libc::c_schar;
    (*l).l[0 as libc::c_int as usize].l.dir[2 as libc::c_int as usize] =
        ((if zRight * 128.0f32 < 127.0f32 {
              (zRight) * 128.0f32
          } else { 127.0f32 }) as s32 & 0xff as libc::c_int) as libc::c_schar;
    (*l).l[1 as libc::c_int as usize].l.dir[0 as libc::c_int as usize] =
        ((if xUp * 128.0f32 < 127.0f32 { (xUp) * 128.0f32 } else { 127.0f32 })
             as s32 & 0xff as libc::c_int) as libc::c_schar;
    (*l).l[1 as libc::c_int as usize].l.dir[1 as libc::c_int as usize] =
        ((if yUp * 128.0f32 < 127.0f32 { (yUp) * 128.0f32 } else { 127.0f32 })
             as s32 & 0xff as libc::c_int) as libc::c_schar;
    (*l).l[1 as libc::c_int as usize].l.dir[2 as libc::c_int as usize] =
        ((if zUp * 128.0f32 < 127.0f32 { (zUp) * 128.0f32 } else { 127.0f32 })
             as s32 & 0xff as libc::c_int) as libc::c_schar;
    (*l).l[0 as libc::c_int as usize].l.col[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[0 as libc::c_int as usize].l.col[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[0 as libc::c_int as usize].l.col[2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[0 as libc::c_int as usize].l.pad1 =
        0 as libc::c_int as libc::c_char;
    (*l).l[0 as libc::c_int as usize].l.colc[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[0 as libc::c_int as usize].l.colc[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[0 as libc::c_int as usize].l.colc[2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[0 as libc::c_int as usize].l.pad2 =
        0 as libc::c_int as libc::c_char;
    (*l).l[1 as libc::c_int as usize].l.col[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[1 as libc::c_int as usize].l.col[1 as libc::c_int as usize] =
        0x80 as libc::c_int as libc::c_uchar;
    (*l).l[1 as libc::c_int as usize].l.col[2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[1 as libc::c_int as usize].l.pad1 =
        0 as libc::c_int as libc::c_char;
    (*l).l[1 as libc::c_int as usize].l.colc[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[1 as libc::c_int as usize].l.colc[1 as libc::c_int as usize] =
        0x80 as libc::c_int as libc::c_uchar;
    (*l).l[1 as libc::c_int as usize].l.colc[2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_uchar;
    (*l).l[1 as libc::c_int as usize].l.pad2 =
        0 as libc::c_int as libc::c_char;
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
/* *
 * guLookAtHilite
 * This function creates the viewing matrix (fixed point) and sets the LookAt/Hilite structures
 * Same args as previous function
 **/
#[no_mangle]
pub unsafe extern "C" fn guLookAtHilite(mut m: *mut Mtx, mut l: *mut LookAt,
                                        mut h: *mut Hilite, mut xEye: f32_0,
                                        mut yEye: f32_0, mut zEye: f32_0,
                                        mut xAt: f32_0, mut yAt: f32_0,
                                        mut zAt: f32_0, mut xUp: f32_0,
                                        mut yUp: f32_0, mut zUp: f32_0,
                                        mut xl1: f32_0, mut yl1: f32_0,
                                        mut zl1: f32_0, mut xl2: f32_0,
                                        mut yl2: f32_0, mut zl2: f32_0,
                                        mut hiliteWidth: s32,
                                        mut hiliteHeight: s32) {
    let mut mf: [[f32_0; 4]; 4] = [[0.; 4]; 4];
    guLookAtHiliteF(mf.as_mut_ptr(), l, h, xEye, yEye, zEye, xAt, yAt, zAt,
                    xUp, yUp, zUp, xl1, yl1, zl1, xl2, yl2, zl2, hiliteWidth,
                    hiliteHeight);
    guMtxF2L(mf.as_mut_ptr() as *mut MtxF, m);
}
