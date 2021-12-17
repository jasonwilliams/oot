#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn ceilf(x: f32_0) -> f32_0;
    #[no_mangle]
    fn truncf(x: f32_0) -> f32_0;
    #[no_mangle]
    fn roundf(x: f32_0) -> f32_0;
    #[no_mangle]
    fn nearbyintf(x: f32_0) -> f32_0;
    #[no_mangle]
    fn cosf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    static mut qNaN0x10000: f32_0;
    #[no_mangle]
    fn floorf(x: f32_0) -> f32_0;
}
pub type s32 = libc::c_int;
pub type f32_0 = libc::c_float;
#[no_mangle]
pub static mut gUseAtanContFrac: s32 = 0;
#[no_mangle]
pub unsafe extern "C" fn Math_FTanF(mut x: f32_0) -> f32_0 {
    let mut sin: f32_0 = sinf(x);
    let mut cos: f32_0 = cosf(x);
    return sin / cos;
}
#[no_mangle]
pub unsafe extern "C" fn Math_FFloorF(mut x: f32_0) -> f32_0 {
    return floorf(x);
}
#[no_mangle]
pub unsafe extern "C" fn Math_FCeilF(mut x: f32_0) -> f32_0 {
    return ceilf(x);
}
#[no_mangle]
pub unsafe extern "C" fn Math_FRoundF(mut x: f32_0) -> f32_0 {
    return roundf(x);
}
#[no_mangle]
pub unsafe extern "C" fn Math_FTruncF(mut x: f32_0) -> f32_0 {
    return truncf(x);
}
#[no_mangle]
pub unsafe extern "C" fn Math_FNearbyIntF(mut x: f32_0) -> f32_0 {
    return nearbyintf(x);
}
/* Arctangent approximation using a Taylor series (one quadrant) */
#[no_mangle]
pub unsafe extern "C" fn Math_FAtanTaylorQF(mut x: f32_0) -> f32_0 {
    static mut coeffs: [f32_0; 9] =
        [-1.0f32 / 3 as libc::c_int as libc::c_float,
         1.0f32 / 5 as libc::c_int as libc::c_float,
         -1.0f32 / 7 as libc::c_int as libc::c_float,
         1.0f32 / 9 as libc::c_int as libc::c_float,
         -1.0f32 / 11 as libc::c_int as libc::c_float,
         1.0f32 / 13 as libc::c_int as libc::c_float,
         -1.0f32 / 15 as libc::c_int as libc::c_float,
         1.0f32 / 17 as libc::c_int as libc::c_float, 0.0f32];
    let mut poly: f32_0 = x;
    let mut sq: f32_0 = x * x;
    let mut exp: f32_0 = x * sq;
    let mut c: *const f32_0 = coeffs.as_ptr();
    let mut term: f32_0 = 0.;
    loop  {
        let fresh0 = c;
        c = c.offset(1);
        term = *fresh0 * exp;
        if poly + term == poly { break ; }
        poly = poly + term;
        exp = exp * sq
    }
    return poly;
}
/* Ditto for two quadrants */
#[no_mangle]
pub unsafe extern "C" fn Math_FAtanTaylorF(mut x: f32_0) -> f32_0 {
    let mut t: f32_0 = 0.;
    let mut q: f32_0 = 0.;
    if x > 0.0f32 {
        t = x
    } else if x < 0.0f32 {
        t = -x
    } else if x == 0.0f32 { return 0.0f32 } else { return qNaN0x10000 }
    if t <= 1.41421356237309504880f32 - 1.0f32 {
        return Math_FAtanTaylorQF(x)
    }
    if t >= 1.41421356237309504880f32 + 1.0f32 {
        q =
            3.14159265358979323846f32 / 2 as libc::c_int as libc::c_float -
                Math_FAtanTaylorQF(1.0f32 / t)
    } else {
        q =
            3.14159265358979323846f32 / 4 as libc::c_int as libc::c_float -
                Math_FAtanTaylorQF((1.0f32 - t) / (1.0f32 + t))
    }
    if x > 0.0f32 { return q } else { return -q };
}
/* Arctangent approximation using a continued fraction */
#[no_mangle]
pub unsafe extern "C" fn Math_FAtanContFracF(mut x: f32_0) -> f32_0 {
    let mut sector: s32 = 0;
    let mut z: f32_0 = 0.;
    let mut conv: f32_0 = 0.;
    let mut sq: f32_0 = 0.;
    let mut i: s32 = 0;
    if x >= -1.0f32 && x <= 1.0f32 {
        sector = 0 as libc::c_int
    } else if x > 1.0f32 {
        sector = 1 as libc::c_int;
        x = 1.0f32 / x
    } else if x < -1.0f32 {
        sector = -(1 as libc::c_int);
        x = 1.0f32 / x
    } else { return qNaN0x10000 }
    sq = x * x;
    conv = 0.0f32;
    z = 8.0f32;
    i = 8 as libc::c_int;
    while i != 0 as libc::c_int {
        conv = z * z * sq / (2.0f32 * z + 1.0f32 + conv);
        z -= 1.0f32;
        i -= 1
    }
    conv = x / (1.0f32 + conv);
    if sector == 0 as libc::c_int {
        return conv
    } else if sector > 0 as libc::c_int {
        return 3.14159265358979323846f32 / 2 as libc::c_int as libc::c_float -
                   conv
    } else {
        return -3.14159265358979323846f32 / 2 as libc::c_int as libc::c_float
                   - conv
    };
}
#[no_mangle]
pub unsafe extern "C" fn Math_FAtanF(mut x: f32_0) -> f32_0 {
    if gUseAtanContFrac == 0 {
        return Math_FAtanTaylorF(x)
    } else { return Math_FAtanContFracF(x) };
}
#[no_mangle]
pub unsafe extern "C" fn Math_FAtan2F(mut y: f32_0, mut x: f32_0) -> f32_0 {
    if x == 0.0f32 {
        if y == 0.0f32 {
            return 0.0f32
        } else if y > 0.0f32 {
            return 3.14159265358979323846f32 /
                       2 as libc::c_int as libc::c_float
        } else if y < 0.0f32 {
            return -3.14159265358979323846f32 /
                       2 as libc::c_int as libc::c_float
        } else { return qNaN0x10000 }
    } else if x >= 0.0f32 {
        return Math_FAtanF(y / x)
    } else if y < 0.0f32 {
        return Math_FAtanF(y / x) - 3.14159265358979323846f32
    } else { return 3.14159265358979323846f32 - Math_FAtanF(-(y / x)) };
}
#[no_mangle]
pub unsafe extern "C" fn Math_FAsinF(mut x: f32_0) -> f32_0 {
    return Math_FAtan2F(x, sqrtf(1.0f32 - x * x));
}
#[no_mangle]
pub unsafe extern "C" fn Math_FAcosF(mut x: f32_0) -> f32_0 {
    return 3.14159265358979323846f32 / 2 as libc::c_int as libc::c_float -
               Math_FAsinF(x);
}
