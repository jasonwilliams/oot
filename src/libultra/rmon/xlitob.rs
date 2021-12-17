#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(dst: *mut libc::c_void, src: *const libc::c_void, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lldiv(num: s64, denom: s64) -> lldiv_t;
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type s64 = libc::c_longlong;
pub type u64_0 = libc::c_ulonglong;
pub type f64_0 = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lldiv_t {
    pub quot: s64,
    pub rem: s64,
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
pub static mut D_8000AF70: [u8_0; 17] =
    unsafe {
        *::std::mem::transmute::<&[u8; 17],
                                 &mut [u8_0; 17]>(b"0123456789abcdef\x00")
    };
#[no_mangle]
pub static mut D_8000AF84: [u8_0; 17] =
    unsafe {
        *::std::mem::transmute::<&[u8; 17],
                                 &mut [u8_0; 17]>(b"0123456789ABCDEF\x00")
    };
#[no_mangle]
pub unsafe extern "C" fn _Litob(mut args: *mut _Pft, mut type_0: u8_0) {
    let mut buff: [u8_0; 24] = [0; 24];
    let mut numMap: *const u8_0 = 0 as *const u8_0;
    let mut base: s32 = 0;
    let mut idx: s32 = 0;
    let mut num: u64_0 = 0;
    let mut quotrem: lldiv_t = lldiv_t{quot: 0, rem: 0,};
    if type_0 as libc::c_int == 'X' as i32 {
        numMap = D_8000AF84.as_mut_ptr()
    } else { numMap = D_8000AF70.as_mut_ptr() }
    base =
        if type_0 as libc::c_int == 'o' as i32 {
            8 as libc::c_int
        } else if type_0 as libc::c_int != 'x' as i32 &&
                      type_0 as libc::c_int != 'X' as i32 {
            10 as libc::c_int
        } else { 16 as libc::c_int };
    idx = 0x18 as libc::c_int;
    num = (*args).v.ll as u64_0;
    if (type_0 as libc::c_int == 'd' as i32 ||
            type_0 as libc::c_int == 'i' as i32) &&
           (*args).v.ll < 0 as libc::c_int as libc::c_longlong {
        num = num.wrapping_neg()
    }
    if num != 0 as libc::c_int as libc::c_ulonglong ||
           (*args).prec != 0 as libc::c_int {
        idx -= 1;
        buff[idx as usize] =
            *numMap.offset(num.wrapping_rem(base as libc::c_ulonglong) as
                               isize)
    }
    (*args).v.ll = num.wrapping_div(base as libc::c_ulonglong) as s64;
    while (*args).v.ll > 0 as libc::c_int as libc::c_longlong &&
              idx > 0 as libc::c_int {
        quotrem = lldiv((*args).v.ll, base as s64);
        (*args).v.ll = quotrem.quot;
        idx -= 1;
        buff[idx as usize] = *numMap.offset(quotrem.rem as isize)
    }
    (*args).n1 = 0x18 as libc::c_int - idx;
    memcpy((*args).s as *mut libc::c_void,
           buff.as_mut_ptr().offset(idx as isize) as *const libc::c_void,
           (*args).n1 as u32_0);
    if (*args).n1 < (*args).prec { (*args).nz0 = (*args).prec - (*args).n1 }
    if (*args).prec < 0 as libc::c_int &&
           (*args).flags &
               (16 as libc::c_int | 4 as libc::c_int) as libc::c_uint ==
               16 as libc::c_int as libc::c_uint {
        idx = (*args).width - (*args).n0 - (*args).nz0 - (*args).n1;
        if idx > 0 as libc::c_int { (*args).nz0 += idx }
    };
}
