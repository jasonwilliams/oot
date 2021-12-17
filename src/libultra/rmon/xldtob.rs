#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
use num_traits::ToPrimitive;
extern "C" {
    #[no_mangle]
    fn memcpy(dst: *mut libc::c_void, src: *const libc::c_void, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ldiv(num: s32, denom: s32) -> ldiv_t;
}
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type s64 = libc::c_longlong;
pub type f64_0 = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldiv_t {
    pub quot: s32,
    pub rem: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Pft {
    pub v: C2RustUnnamed,
    pub s: *mut libc::c_char,
    pub n0: s32,
    pub nz0: s32,
    pub n1: s32,
    pub nz1: s32,
    pub n2: s32,
    pub nz2: s32,
    pub prec: s32,
    pub width: s32,
    pub nchar: u32_0,
    pub flags: u32_0,
    pub qual: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ll: s64,
    pub ld: f64_0,
}
#[no_mangle]
pub static mut D_800122E0: [f64_0; 9] =
    [f128::f128::new(10e0).to_f64().unwrap(),
     f128::f128::new(10e1).to_f64().unwrap(),
     f128::f128::new(10e3).to_f64().unwrap(),
     f128::f128::new(10e7).to_f64().unwrap(),
     f128::f128::new(10e15).to_f64().unwrap(),
     f128::f128::new(10e31).to_f64().unwrap(),
     f128::f128::new(10e63).to_f64().unwrap(),
     f128::f128::new(10e127).to_f64().unwrap(),
     f128::f128::new(10e255).to_f64().unwrap()];
#[no_mangle]
pub unsafe extern "C" fn _Ldtob(mut args: *mut _Pft, mut type_0: u8_0) {
    let mut buff: [u8_0; 32] = [0; 32];
    let mut ptr: *mut u8_0 = buff.as_mut_ptr();
    let mut sp70: u32_0 = 0;
    let mut val: f64_0 = (*args).v.ld;
    /* maybe struct? */
    let mut err: s16 = 0;
    let mut nsig: s16 = 0;
    let mut exp: s16 = 0;
    let mut i: s32 = 0;
    let mut n: s32 = 0;
    let mut factor: f64_0 = 0.;
    let mut gen: s32 = 0;
    let mut j: s32 = 0;
    let mut lo: s32 = 0;
    let mut qr: ldiv_t = ldiv_t{quot: 0, rem: 0,};
    let mut drop_0: u8_0 = 0;
    let mut n2: s32 = 0;
    if (*args).prec < 0 as libc::c_int {
        (*args).prec = 6 as libc::c_int
    } else if (*args).prec == 0 as libc::c_int &&
                  (type_0 as libc::c_int == 'g' as i32 ||
                       type_0 as libc::c_int == 'G' as i32) {
        (*args).prec = 1 as libc::c_int
    }
    err = _Ldunscale(&mut exp, args);
    if err as libc::c_int > 0 as libc::c_int {
        (*args).n1 = 3 as libc::c_int;
        memcpy((*args).s as *mut libc::c_void,
               if err as libc::c_int == 2 as libc::c_int {
                   b"NaN\x00" as *const u8 as *const libc::c_char
               } else { b"Inf\x00" as *const u8 as *const libc::c_char } as
                   *const libc::c_void, (*args).n1 as u32_0);
        return
    }
    if err as libc::c_int == 0 as libc::c_int {
        nsig = 0 as libc::c_int as s16;
        exp = 0 as libc::c_int as s16
    } else {
        if val < 0 as libc::c_int as libc::c_double { val = -val }
        exp =
            (exp as libc::c_int * 30103 as libc::c_int /
                 0x186a0 as libc::c_int - 4 as libc::c_int) as s16;
        if (exp as libc::c_int) < 0 as libc::c_int {
            n = 3 as libc::c_int - exp as libc::c_int & !(3 as libc::c_int);
            exp = -n as s16;
            i = 0 as libc::c_int;
            while n > 0 as libc::c_int {
                if n & 1 as libc::c_int != 0 as libc::c_int {
                    val *= D_800122E0[i as usize]
                }
                n >>= 1 as libc::c_int;
                i += 1
            }
        } else if exp as libc::c_int > 0 as libc::c_int {
            factor = 1 as libc::c_int as f64_0;
            exp = (exp as libc::c_int & !(3 as libc::c_int)) as s16;
            n = exp as s32;
            i = 0 as libc::c_int;
            while n > 0 as libc::c_int {
                if n & 1 as libc::c_int != 0 as libc::c_int {
                    factor *= D_800122E0[i as usize]
                }
                n >>= 1 as libc::c_int;
                i += 1
            }
            val /= factor
        }
        gen =
            (if type_0 as libc::c_int == 'f' as i32 {
                 (exp as libc::c_int) + 10 as libc::c_int
             } else { 6 as libc::c_int }) + (*args).prec;
        if gen > 0x13 as libc::c_int { gen = 0x13 as libc::c_int }
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = '0' as i32 as u8_0;
        while gen > 0 as libc::c_int &&
                  (0 as libc::c_int as libc::c_double) < val {
            lo = val as s32;
            gen -= 8 as libc::c_int;
            if gen > 0 as libc::c_int {
                val = (val - lo as libc::c_double) * 1.0e8f64
            }
            ptr = ptr.offset(8 as libc::c_int as isize);
            j = 8 as libc::c_int;
            while lo > 0 as libc::c_int && { j -= 1; (j) >= 0 as libc::c_int }
                  {
                qr = ldiv(lo, 10 as libc::c_int);
                ptr = ptr.offset(-1);
                *ptr = (qr.rem + '0' as i32) as u8_0;
                lo = qr.quot
            }
            loop  {
                j -= 1;
                if !(j >= 0 as libc::c_int) { break ; }
                ptr = ptr.offset(-1);
                *ptr = '0' as i32 as u8_0
            }
            ptr = ptr.offset(8 as libc::c_int as isize)
        }
        gen =
            ptr.wrapping_offset_from(&mut *buff.as_mut_ptr().offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                         as *mut u8_0) as libc::c_int;
        ptr =
            &mut *buff.as_mut_ptr().offset(1 as libc::c_int as isize) as
                *mut u8_0;
        exp = (exp as libc::c_int + 7 as libc::c_int) as s16;
        while *ptr as libc::c_int == '0' as i32 {
            gen -= 1;
            exp -= 1;
            ptr = ptr.offset(1)
        }
        nsig =
            ((if type_0 as libc::c_int == 'f' as i32 {
                  (exp as libc::c_int) + 1 as libc::c_int
              } else {
                  (if type_0 as libc::c_int == 'e' as i32 ||
                          type_0 as libc::c_int == 'E' as i32 {
                       1 as libc::c_int
                   } else { 0 as libc::c_int })
              }) + (*args).prec) as s16;
        if gen < nsig as libc::c_int { nsig = gen as s16 }
        if nsig as libc::c_int > 0 as libc::c_int {
            if (nsig as libc::c_int) < gen &&
                   *ptr.offset(nsig as isize) as libc::c_int > '4' as i32 {
                drop_0 = '9' as i32 as u8_0
            } else { drop_0 = '0' as i32 as u8_0 }
            n2 = nsig as s32;
            loop  {
                n2 -= 1;
                if !(*ptr.offset(n2 as isize) as libc::c_int ==
                         drop_0 as libc::c_int) {
                    break ;
                }
                nsig -= 1
            }
            if drop_0 as libc::c_int == '9' as i32 {
                let ref mut fresh1 = *ptr.offset(n2 as isize);
                *fresh1 = (*fresh1).wrapping_add(1)
            }
            if n2 < 0 as libc::c_int {
                ptr = ptr.offset(-1);
                nsig += 1;
                exp += 1
            }
        }
    }
    _Genld(args, type_0, ptr, nsig, exp);
}
#[no_mangle]
pub unsafe extern "C" fn _Ldunscale(mut pex: *mut s16, mut px: *mut _Pft)
 -> s16 {
    let mut ps: *mut u16_0 = px as *mut u16_0;
    let mut xchar: s16 =
        ((*ps.offset(0 as libc::c_int as isize) as libc::c_int &
              (0x7fff as libc::c_int &
                   !(((1 as libc::c_int) << 4 as libc::c_int) -
                         1 as libc::c_int))) >> 4 as libc::c_int) as s16;
    if xchar as libc::c_int ==
           ((1 as libc::c_int) << 15 as libc::c_int - 4 as libc::c_int) -
               1 as libc::c_int {
        /* NaN or INF */
        *pex = 0 as libc::c_int as s16; /* 'e' format */
        return if *ps.offset(0 as libc::c_int as isize) as libc::c_int &
                      ((1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int != 0 ||
                      *ps.offset(1 as libc::c_int as isize) as libc::c_int !=
                          0 ||
                      *ps.offset(2 as libc::c_int as isize) as libc::c_int !=
                          0 ||
                      *ps.offset(3 as libc::c_int as isize) as libc::c_int !=
                          0 {
                   2 as libc::c_int
               } else { 1 as libc::c_int } as s16
    } else {
        if (0 as libc::c_int) < xchar as libc::c_int {
            *ps.offset(0 as libc::c_int as isize) =
                (*ps.offset(0 as libc::c_int as isize) as libc::c_int &
                     !(0x7fff as libc::c_int &
                           !(((1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int)) |
                     (0x3ff as libc::c_int) << 4 as libc::c_int) as u16_0;
            *pex =
                (xchar as libc::c_int -
                     (0x3ff as libc::c_int - 1 as libc::c_int)) as s16;
            return -(1 as libc::c_int) as s16
        }
    }
    if 0 as libc::c_int > xchar as libc::c_int {
        return 2 as libc::c_int as s16
    } else { *pex = 0 as libc::c_int as s16; return 0 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn _Genld(mut px: *mut _Pft, mut code: u8_0,
                                mut p: *mut u8_0, mut nsig: s16,
                                mut xexp: s16) {
    let mut point: u8_0 = '.' as i32 as u8_0;
    if nsig as libc::c_int <= 0 as libc::c_int {
        nsig = 1 as libc::c_int as s16;
        p = b"0\x00" as *const u8 as *const libc::c_char as *mut u8_0
    }
    if code as libc::c_int == 'f' as i32 ||
           (code as libc::c_int == 'g' as i32 ||
                code as libc::c_int == 'G' as i32) &&
               -(4 as libc::c_int) <= xexp as libc::c_int &&
               (xexp as libc::c_int) < (*px).prec {
        /* 'f' format */
        xexp += 1; /* change to leading digit count */
        if code as libc::c_int != 'f' as i32 {
            /* fixup for 'g' */
            if (*px).flags & 8 as libc::c_int as libc::c_uint == 0 &&
                   (nsig as libc::c_int) < (*px).prec {
                (*px).prec = nsig as s32
            }
            (*px).prec -= xexp as libc::c_int;
            if (*px).prec < 0 as libc::c_int { (*px).prec = 0 as libc::c_int }
        }
        if xexp as libc::c_int <= 0 as libc::c_int {
            /* digits only to right of point */
            let fresh2 = (*px).n1; /* enough digits before point */
            (*px).n1 = (*px).n1 + 1;
            *(*px).s.offset(fresh2 as isize) = '0' as i32 as libc::c_char;
            if (0 as libc::c_int) < (*px).prec ||
                   (*px).flags & 8 as libc::c_int as libc::c_uint != 0 {
                let fresh3 = (*px).n1;
                (*px).n1 = (*px).n1 + 1;
                *(*px).s.offset(fresh3 as isize) = point as libc::c_char
            }
            if (*px).prec < -(xexp as libc::c_int) {
                xexp = -(*px).prec as s16
            }
            (*px).nz1 = -(xexp as libc::c_int);
            (*px).prec += xexp as libc::c_int;
            if (*px).prec < nsig as libc::c_int { nsig = (*px).prec as s16 }
            (*px).n2 = nsig as s32;
            memcpy(&mut *(*px).s.offset((*px).n1 as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   p as *const libc::c_void, (*px).n2 as u32_0);
            (*px).nz2 = (*px).prec - nsig as libc::c_int
        } else if (nsig as libc::c_int) < xexp as libc::c_int {
            /* zeros before point */
            memcpy(&mut *(*px).s.offset((*px).n1 as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   p as *const libc::c_void, nsig as u32_0);
            (*px).n1 += nsig as libc::c_int;
            (*px).nz1 = xexp as libc::c_int - nsig as libc::c_int;
            if (0 as libc::c_int) < (*px).prec ||
                   (*px).flags & 8 as libc::c_int as libc::c_uint != 0 {
                *(*px).s.offset((*px).n1 as isize) = point as libc::c_char;
                (*px).n2 += 1
            }
            (*px).nz2 = (*px).prec
        } else {
            memcpy(&mut *(*px).s.offset((*px).n1 as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   p as *const libc::c_void, xexp as u32_0);
            (*px).n1 += xexp as libc::c_int;
            nsig = (nsig as libc::c_int - xexp as libc::c_int) as s16;
            if (0 as libc::c_int) < (*px).prec ||
                   (*px).flags & 8 as libc::c_int as libc::c_uint != 0 {
                let fresh4 = (*px).n1;
                (*px).n1 = (*px).n1 + 1;
                *(*px).s.offset(fresh4 as isize) = point as libc::c_char
            }
            if (*px).prec < nsig as libc::c_int { nsig = (*px).prec as s16 }
            memcpy(&mut *(*px).s.offset((*px).n1 as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   p.offset(xexp as libc::c_int as isize) as
                       *const libc::c_void, nsig as u32_0);
            (*px).n1 += nsig as libc::c_int;
            (*px).nz1 = (*px).prec - nsig as libc::c_int
        }
    } else {
        if code as libc::c_int == 'g' as i32 ||
               code as libc::c_int == 'G' as i32 {
            /* fixup for 'g' */
            if (nsig as libc::c_int) < (*px).prec { (*px).prec = nsig as s32 }
            (*px).prec -= 1;
            if (*px).prec < 0 as libc::c_int { (*px).prec = 0 as libc::c_int }
            code =
                if code as libc::c_int == 'g' as i32 {
                    'e' as i32
                } else { 'E' as i32 } as u8_0
        }
        let fresh5 = p;
        p = p.offset(1);
        let fresh6 = (*px).n1;
        (*px).n1 = (*px).n1 + 1;
        *(*px).s.offset(fresh6 as isize) = *fresh5 as libc::c_char;
        if (0 as libc::c_int) < (*px).prec ||
               (*px).flags & 8 as libc::c_int as libc::c_uint != 0 {
            let fresh7 = (*px).n1;
            (*px).n1 = (*px).n1 + 1;
            *(*px).s.offset(fresh7 as isize) = point as libc::c_char
        }
        if (0 as libc::c_int) < (*px).prec {
            /* put fraction digits */
            nsig -= 1; /* put exponent */
            if (*px).prec < nsig as libc::c_int {
                nsig = (*px).prec as s16
            } /* negative exponent */
            memcpy(&mut *(*px).s.offset((*px).n1 as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   p as *const libc::c_void, nsig as u32_0);
            (*px).n1 += nsig as libc::c_int;
            (*px).nz1 = (*px).prec - nsig as libc::c_int
        }
        p =
            &mut *(*px).s.offset((*px).n1 as isize) as *mut libc::c_char as
                *mut u8_0;
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = code;
        if 0 as libc::c_int <= xexp as libc::c_int {
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = '+' as i32 as u8_0
        } else {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = '-' as i32 as u8_0;
            xexp = -(xexp as libc::c_int) as s16
        }
        if 100 as libc::c_int <= xexp as libc::c_int {
            /* put oversize exponent */
            if 1000 as libc::c_int <= xexp as libc::c_int {
                let fresh11 = p;
                p = p.offset(1);
                *fresh11 =
                    (xexp as libc::c_int / 1000 as libc::c_int + '0' as i32)
                        as u8_0;
                xexp = (xexp as libc::c_int % 1000 as libc::c_int) as s16
            }
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 =
                (xexp as libc::c_int / 100 as libc::c_int + '0' as i32) as
                    u8_0;
            xexp = (xexp as libc::c_int % 100 as libc::c_int) as s16
        }
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 =
            (xexp as libc::c_int / 10 as libc::c_int + '0' as i32) as u8_0;
        xexp = (xexp as libc::c_int % 10 as libc::c_int) as s16;
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = (xexp as libc::c_int + '0' as i32) as u8_0;
        (*px).n2 =
            p.wrapping_offset_from(&mut *(*px).s.offset((*px).n1 as isize) as
                                       *mut libc::c_char as *mut u8_0) as
                libc::c_int
    }
    if (*px).flags & (16 as libc::c_int | 4 as libc::c_int) as libc::c_uint ==
           16 as libc::c_int as libc::c_uint {
        /* pad with leading zeros */
        let mut n: s32 =
            (*px).n0 + (*px).n1 + (*px).nz1 + (*px).n2 + (*px).nz2;
        if n < (*px).width { (*px).nz0 = (*px).width - n }
    };
}
