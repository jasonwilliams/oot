#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut __libm_qnan_f: f32_0;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type f32_0 = libc::c_float;
pub type f64_0 = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub union du {
    pub word: C2RustUnnamed,
    pub d: f64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub hi: u32_0,
    pub lo: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union fu {
    pub i: u32_0,
    pub f: f32_0,
}
static mut P: [du; 5] =
    [du{word:
            {
                let mut init =
                    C2RustUnnamed{hi: 0x3ff00000 as libc::c_int as u32_0,
                                  lo: 0 as libc::c_int as u32_0,};
                init
            },},
     du{word:
            {
                let mut init =
                    C2RustUnnamed{hi: 0xbfc55554 as libc::c_uint,
                                  lo: 0xbc83656d as libc::c_uint,};
                init
            },},
     du{word:
            {
                let mut init =
                    C2RustUnnamed{hi: 0x3f8110ed as libc::c_int as u32_0,
                                  lo: 0x3804c2a0 as libc::c_int as u32_0,};
                init
            },},
     du{word:
            {
                let mut init =
                    C2RustUnnamed{hi: 0xbf29f6ff as libc::c_uint,
                                  lo: 0xeea56814 as libc::c_uint,};
                init
            },},
     du{word:
            {
                let mut init =
                    C2RustUnnamed{hi: 0x3ec5dbdf as libc::c_int as u32_0,
                                  lo: 0xe314bfe as libc::c_int as u32_0,};
                init
            },}];
static mut rpi: du =
    du{word:
           {
               let mut init =
                   C2RustUnnamed{hi: 0x3fd45f30 as libc::c_int as u32_0,
                                 lo: 0x6dc9c883 as libc::c_int as u32_0,};
               init
           },};
static mut pihi: du =
    du{word:
           {
               let mut init =
                   C2RustUnnamed{hi: 0x400921fb as libc::c_int as u32_0,
                                 lo: 0x50000000 as libc::c_int as u32_0,};
               init
           },};
static mut pilo: du =
    du{word:
           {
               let mut init =
                   C2RustUnnamed{hi: 0x3e6110b4 as libc::c_int as u32_0,
                                 lo: 0x611a6263 as libc::c_int as u32_0,};
               init
           },};
static mut zero: fu = fu{i: 0 as libc::c_int as u32_0,};
#[no_mangle]
pub unsafe extern "C" fn sinf(mut x: f32_0) -> f32_0 {
    let mut dx: f64_0 = 0.;
    let mut xSq: f64_0 = 0.;
    let mut polyApprox: f64_0 = 0.;
    let mut dn: f64_0 = 0.;
    let mut n: s32 = 0;
    let mut result: f64_0 = 0.;
    let mut ix: s32 = *(&mut x as *mut f32_0 as *mut s32);
    let mut xpt: s32 = ix >> 22 as libc::c_int;
    xpt &= 0x1ff as libc::c_int;
    if xpt < 0xff as libc::c_int {
        dx = x as f64_0;
        if xpt >= 0xe6 as libc::c_int {
            xSq = dx * dx;
            polyApprox =
                ((P[4 as libc::c_int as usize].d * xSq +
                      P[3 as libc::c_int as usize].d) * xSq +
                     P[2 as libc::c_int as usize].d) * xSq +
                    P[1 as libc::c_int as usize].d;
            result = dx + dx * xSq * polyApprox;
            return result as f32_0
        }
        return x
    }
    if xpt < 0x136 as libc::c_int {
        dx = x as f64_0;
        dn = dx * rpi.d;
        n = if dn >= 0.0f64 { (dn) + 0.5f64 } else { (dn) - 0.5f64 } as s32;
        dn = n as f64_0;
        dx -= dn * pihi.d;
        dx -= dn * pilo.d;
        xSq = dx * dx;
        polyApprox =
            ((P[4 as libc::c_int as usize].d * xSq +
                  P[3 as libc::c_int as usize].d) * xSq +
                 P[2 as libc::c_int as usize].d) * xSq +
                P[1 as libc::c_int as usize].d;
        result = dx + dx * xSq * polyApprox;
        if n & 1 as libc::c_int == 0 { return result as f32_0 }
        return -(result as f32_0)
    }
    if x != x { return __libm_qnan_f }
    return zero.f;
}
