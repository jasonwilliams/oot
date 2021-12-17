#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn strchr(str: *const libc::c_char, ch: s32) -> *const libc::c_char;
    #[no_mangle]
    fn strlen(str: *const libc::c_char) -> u32_0;
    #[no_mangle]
    fn _Litob(args: *mut _Pft, type_0: u8_0);
    #[no_mangle]
    fn _Ldtob(args: *mut _Pft, type_0: u8_0);
}
pub type __builtin_va_list = *mut libc::c_char;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type s64 = libc::c_longlong;
pub type u64_0 = libc::c_ulonglong;
pub type f64_0 = libc::c_double;
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
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
#[no_mangle]
pub static mut spaces: [libc::c_char; 33] =
    unsafe {
        *::std::mem::transmute::<&[u8; 33],
                                 &mut [libc::c_char; 33]>(b"                                \x00")
    };
#[no_mangle]
pub static mut zeroes: [libc::c_char; 33] =
    unsafe {
        *::std::mem::transmute::<&[u8; 33],
                                 &mut [libc::c_char; 33]>(b"00000000000000000000000000000000\x00")
    };
#[no_mangle]
pub unsafe extern "C" fn _Printf(mut pfn: PrintCallback,
                                 mut arg: *mut libc::c_void,
                                 mut fmt: *const libc::c_char,
                                 mut ap: __builtin_va_list) -> s32 {
    let mut x: _Pft =
        _Pft{v: C2RustUnnamed{ll: 0,},
             s: 0 as *mut libc::c_char,
             n0: 0,
             nz0: 0,
             n1: 0,
             nz1: 0,
             n2: 0,
             nz2: 0,
             prec: 0,
             width: 0,
             nchar: 0,
             flags: 0,
             qual: 0,};
    x.nchar = 0 as libc::c_int as u32_0;
    loop  {
        static mut fchar: [libc::c_char; 6] =
            unsafe {
                *::std::mem::transmute::<&[u8; 6],
                                         &[libc::c_char; 6]>(b" +-#0\x00")
            };
        static mut fbit: [u32_0; 6] =
            [1 as libc::c_int as u32_0, 2 as libc::c_int as u32_0,
             4 as libc::c_int as u32_0, 8 as libc::c_int as u32_0,
             16 as libc::c_int as u32_0, 0 as libc::c_int as u32_0];
        let mut s: *const u8_0 = fmt as *mut u8_0;
        let mut c: u8_0 = 0;
        let mut t: *const libc::c_char = 0 as *const libc::c_char;
        let mut ac: [u8_0; 32] = [0; 32];
        loop  {
            c = *s;
            if !(c as libc::c_int != 0 as libc::c_int &&
                     c as libc::c_int != '%' as i32) {
                break ;
            }
            s = s.offset(1)
        }
        if s.wrapping_offset_from(fmt as *mut u8_0) as libc::c_int >
               0 as libc::c_int {
            arg =
                pfn.expect("non-null function pointer")(arg, fmt,
                                                        s.wrapping_offset_from(fmt
                                                                                   as
                                                                                   *mut u8_0)
                                                            as libc::c_int as
                                                            u32_0);
            if !arg.is_null() {
                x.nchar =
                    (x.nchar as
                         libc::c_uint).wrapping_add(s.wrapping_offset_from(fmt
                                                                               as
                                                                               *mut u8_0)
                                                        as libc::c_int as
                                                        libc::c_uint) as u32_0
                        as u32_0
            } else { return x.nchar as s32 }
        }
        if c as libc::c_int == 0 as libc::c_int { return x.nchar as s32 }
        s = s.offset(1);
        fmt = s as *mut libc::c_char;
        x.flags = 0 as libc::c_int as u32_0;
        loop  {
            t = strchr(fchar.as_ptr(), *s as s32);
            if t.is_null() { break ; }
            x.flags |=
                fbit[t.wrapping_offset_from(fchar.as_ptr()) as libc::c_int as
                         usize];
            s = s.offset(1)
        }
        if *s as libc::c_int == '*' as i32 {
            x.width = ap.arg::<s32>();
            if x.width < 0 as libc::c_int {
                x.width = -x.width;
                x.flags |= 4 as libc::c_int as libc::c_uint
            }
            s = s.offset(1)
        } else {
            x.width = 0 as libc::c_int;
            while *s as libc::c_int >= '0' as i32 &&
                      *s as libc::c_int <= '9' as i32 {
                if x.width < 999 as libc::c_int {
                    x.width =
                        *s as libc::c_int + x.width * 10 as libc::c_int -
                            '0' as i32
                }
                s = s.offset(1)
            }
        }
        if *s as libc::c_int != '.' as i32 {
            x.prec = -(1 as libc::c_int)
        } else {
            s = s.offset(1);
            if *s as libc::c_int == '*' as i32 {
                x.prec = ap.arg::<s32>();
                s = s.offset(1)
            } else {
                x.prec = 0 as libc::c_int;
                while *s as libc::c_int >= '0' as i32 &&
                          *s as libc::c_int <= '9' as i32 {
                    if x.prec < 999 as libc::c_int {
                        x.prec =
                            *s as libc::c_int + x.prec * 10 as libc::c_int -
                                '0' as i32
                    }
                    s = s.offset(1)
                }
            }
        }
        if !strchr(b"hlL\x00" as *const u8 as *const libc::c_char,
                   *s as s32).is_null() {
            let fresh0 = s;
            s = s.offset(1);
            x.qual = *fresh0
        } else { x.qual = 0 as libc::c_int as u8_0 }
        if x.qual as libc::c_int == 'l' as i32 &&
               *s as libc::c_int == 'l' as i32 {
            x.qual = 'L' as i32 as u8_0;
            s = s.offset(1)
        }
        _Putfld(&mut x, &mut ap, *s, ac.as_mut_ptr());
        x.width -= x.n0 + x.nz0 + x.n1 + x.nz1 + x.n2 + x.nz2;
        if x.flags & 4 as libc::c_int as libc::c_uint == 0 &&
               x.width > 0 as libc::c_int {
            let mut i: s32 = 0;
            let mut j: s32 = 0;
            j = x.width;
            while j > 0 as libc::c_int {
                if j as u32_0 > 32 as libc::c_int as libc::c_uint {
                    i = 32 as libc::c_int
                } else { i = j }
                if i > 0 as libc::c_int {
                    arg =
                        pfn.expect("non-null function pointer")(arg,
                                                                spaces.as_mut_ptr(),
                                                                i as u32_0);
                    if !arg.is_null() {
                        x.nchar =
                            (x.nchar as
                                 libc::c_uint).wrapping_add(i as libc::c_uint)
                                as u32_0 as u32_0
                    } else { return x.nchar as s32 }
                }
                j -= i
            }
        }
        if x.n0 > 0 as libc::c_int {
            arg =
                pfn.expect("non-null function pointer")(arg,
                                                        ac.as_mut_ptr() as
                                                            *mut libc::c_char,
                                                        x.n0 as u32_0);
            if !arg.is_null() {
                x.nchar =
                    (x.nchar as
                         libc::c_uint).wrapping_add(x.n0 as libc::c_uint) as
                        u32_0 as u32_0
            } else { return x.nchar as s32 }
        }
        if 1 as libc::c_int != 0 && x.nz0 > 0 as libc::c_int {
            let mut i_0: s32 = 0;
            let mut j_0: s32 = 0;
            j_0 = x.nz0;
            while j_0 > 0 as libc::c_int {
                if j_0 as u32_0 > 32 as libc::c_int as libc::c_uint {
                    i_0 = 32 as libc::c_int
                } else { i_0 = j_0 }
                if i_0 > 0 as libc::c_int {
                    arg =
                        pfn.expect("non-null function pointer")(arg,
                                                                zeroes.as_mut_ptr(),
                                                                i_0 as u32_0);
                    if !arg.is_null() {
                        x.nchar =
                            (x.nchar as
                                 libc::c_uint).wrapping_add(i_0 as
                                                                libc::c_uint)
                                as u32_0 as u32_0
                    } else { return x.nchar as s32 }
                }
                j_0 -= i_0
            }
        }
        if x.n1 > 0 as libc::c_int {
            arg =
                pfn.expect("non-null function pointer")(arg, x.s,
                                                        x.n1 as u32_0);
            if !arg.is_null() {
                x.nchar =
                    (x.nchar as
                         libc::c_uint).wrapping_add(x.n1 as libc::c_uint) as
                        u32_0 as u32_0
            } else { return x.nchar as s32 }
        }
        if 1 as libc::c_int != 0 && x.nz1 > 0 as libc::c_int {
            let mut i_1: s32 = 0;
            let mut j_1: s32 = 0;
            j_1 = x.nz1;
            while j_1 > 0 as libc::c_int {
                if j_1 as u32_0 > 32 as libc::c_int as libc::c_uint {
                    i_1 = 32 as libc::c_int
                } else { i_1 = j_1 }
                if i_1 > 0 as libc::c_int {
                    arg =
                        pfn.expect("non-null function pointer")(arg,
                                                                zeroes.as_mut_ptr(),
                                                                i_1 as u32_0);
                    if !arg.is_null() {
                        x.nchar =
                            (x.nchar as
                                 libc::c_uint).wrapping_add(i_1 as
                                                                libc::c_uint)
                                as u32_0 as u32_0
                    } else { return x.nchar as s32 }
                }
                j_1 -= i_1
            }
        }
        if x.n2 > 0 as libc::c_int {
            arg =
                pfn.expect("non-null function pointer")(arg,
                                                        &mut *x.s.offset(x.n1
                                                                             as
                                                                             isize)
                                                            as
                                                            *mut libc::c_char,
                                                        x.n2 as u32_0);
            if !arg.is_null() {
                x.nchar =
                    (x.nchar as
                         libc::c_uint).wrapping_add(x.n2 as libc::c_uint) as
                        u32_0 as u32_0
            } else { return x.nchar as s32 }
        }
        if 1 as libc::c_int != 0 && x.nz2 > 0 as libc::c_int {
            let mut i_2: s32 = 0;
            let mut j_2: s32 = 0;
            j_2 = x.nz2;
            while j_2 > 0 as libc::c_int {
                if j_2 as u32_0 > 32 as libc::c_int as libc::c_uint {
                    i_2 = 32 as libc::c_int
                } else { i_2 = j_2 }
                if i_2 > 0 as libc::c_int {
                    arg =
                        pfn.expect("non-null function pointer")(arg,
                                                                zeroes.as_mut_ptr(),
                                                                i_2 as u32_0);
                    if !arg.is_null() {
                        x.nchar =
                            (x.nchar as
                                 libc::c_uint).wrapping_add(i_2 as
                                                                libc::c_uint)
                                as u32_0 as u32_0
                    } else { return x.nchar as s32 }
                }
                j_2 -= i_2
            }
        }
        if x.flags & 4 as libc::c_int as libc::c_uint != 0 &&
               x.width > 0 as libc::c_int {
            let mut i_3: s32 = 0;
            let mut j_3: s32 = 0;
            j_3 = x.width;
            while j_3 > 0 as libc::c_int {
                if j_3 as u32_0 > 32 as libc::c_int as libc::c_uint {
                    i_3 = 32 as libc::c_int
                } else { i_3 = j_3 }
                if i_3 > 0 as libc::c_int {
                    arg =
                        pfn.expect("non-null function pointer")(arg,
                                                                spaces.as_mut_ptr(),
                                                                i_3 as u32_0);
                    if !arg.is_null() {
                        x.nchar =
                            (x.nchar as
                                 libc::c_uint).wrapping_add(i_3 as
                                                                libc::c_uint)
                                as u32_0 as u32_0
                    } else { return x.nchar as s32 }
                }
                j_3 -= i_3
            }
        }
        fmt = (s as *mut libc::c_char).offset(1 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn _Putfld(mut px: *mut _Pft,
                                 mut pap: *mut __builtin_va_list,
                                 mut code: u8_0, mut ac: *mut u8_0) {
    (*px).nz2 = 0 as libc::c_int;
    (*px).n2 = (*px).nz2;
    (*px).nz1 = (*px).n2;
    (*px).n1 = (*px).nz1;
    (*px).nz0 = (*px).n1;
    (*px).n0 = (*px).nz0;
    match code as libc::c_int {
        99 => {
            let fresh1 = (*px).n0;
            (*px).n0 = (*px).n0 + 1;
            *ac.offset(fresh1 as isize) = (*pap).arg::<u32_0>() as u8_0
        }
        100 | 105 => {
            if (*px).qual as libc::c_int == 'l' as i32 {
                (*px).v.ll = (*pap).arg::<s32>() as s64
            } else if (*px).qual as libc::c_int == 'L' as i32 {
                (*px).v.ll = (*pap).arg::<s64>()
            } else { (*px).v.ll = (*pap).arg::<s32>() as s64 }
            if (*px).qual as libc::c_int == 'h' as i32 {
                (*px).v.ll = (*px).v.ll as s16 as s64
            }
            if (*px).v.ll < 0 as libc::c_int as libc::c_longlong {
                let fresh2 = (*px).n0;
                (*px).n0 = (*px).n0 + 1;
                *ac.offset(fresh2 as isize) = '-' as i32 as u8_0
            } else if (*px).flags & 2 as libc::c_int as libc::c_uint != 0 {
                let fresh3 = (*px).n0;
                (*px).n0 = (*px).n0 + 1;
                *ac.offset(fresh3 as isize) = '+' as i32 as u8_0
            } else if (*px).flags & 1 as libc::c_int as libc::c_uint != 0 {
                let fresh4 = (*px).n0;
                (*px).n0 = (*px).n0 + 1;
                *ac.offset(fresh4 as isize) = ' ' as i32 as u8_0
            }
            (*px).s =
                &mut *ac.offset((*px).n0 as isize) as *mut u8_0 as
                    *mut libc::c_char;
            _Litob(px, code);
        }
        120 | 88 | 117 | 111 => {
            if (*px).qual as libc::c_int == 'l' as i32 {
                (*px).v.ll = (*pap).arg::<s32>() as s64
            } else if (*px).qual as libc::c_int == 'L' as i32 {
                (*px).v.ll = (*pap).arg::<s64>()
            } else { (*px).v.ll = (*pap).arg::<s32>() as s64 }
            if (*px).qual as libc::c_int == 'h' as i32 {
                (*px).v.ll = (*px).v.ll as u16_0 as s64
            } else if (*px).qual as libc::c_int == 0 as libc::c_int {
                (*px).v.ll = (*px).v.ll as u32_0 as s64
            }
            if (*px).flags & 8 as libc::c_int as libc::c_uint != 0 {
                let fresh5 = (*px).n0;
                (*px).n0 = (*px).n0 + 1;
                *ac.offset(fresh5 as isize) = '0' as i32 as u8_0;
                if code as libc::c_int == 'x' as i32 ||
                       code as libc::c_int == 'X' as i32 {
                    let fresh6 = (*px).n0;
                    (*px).n0 = (*px).n0 + 1;
                    *ac.offset(fresh6 as isize) = code
                }
            }
            (*px).s =
                &mut *ac.offset((*px).n0 as isize) as *mut u8_0 as
                    *mut libc::c_char;
            _Litob(px, code);
        }
        101 | 102 | 103 | 69 | 71 => {
            (*px).v.ld =
                if (*px).qual as libc::c_int == 'L' as i32 {
                    (*pap).arg::<f64_0>()
                } else { (*pap).arg::<f64_0>() };
            if *(&mut (*px).v.ll as *mut s64 as *mut u16_0) as libc::c_int &
                   0x8000 as libc::c_int != 0 {
                let fresh7 = (*px).n0;
                (*px).n0 = (*px).n0 + 1;
                *ac.offset(fresh7 as isize) = '-' as i32 as u8_0
            } else if (*px).flags & 2 as libc::c_int as libc::c_uint != 0 {
                let fresh8 = (*px).n0;
                (*px).n0 = (*px).n0 + 1;
                *ac.offset(fresh8 as isize) = '+' as i32 as u8_0
            } else if (*px).flags & 1 as libc::c_int as libc::c_uint != 0 {
                let fresh9 = (*px).n0;
                (*px).n0 = (*px).n0 + 1;
                *ac.offset(fresh9 as isize) = ' ' as i32 as u8_0
            }
            (*px).s =
                &mut *ac.offset((*px).n0 as isize) as *mut u8_0 as
                    *mut libc::c_char;
            _Ldtob(px, code);
        }
        110 => {
            if (*px).qual as libc::c_int == 'h' as i32 {
                *(*pap).arg::<*mut u16_0>() = (*px).nchar as u16_0
            } else if (*px).qual as libc::c_int == 'l' as i32 {
                *(*pap).arg::<*mut u32_0>() = (*px).nchar
            } else if (*px).qual as libc::c_int == 'L' as i32 {
                *(*pap).arg::<*mut u64_0>() = (*px).nchar as u64_0
            } else { *(*pap).arg::<*mut u32_0>() = (*px).nchar }
        }
        112 => {
            (*px).v.ll = (*pap).arg::<*mut libc::c_void>() as s64;
            (*px).s =
                &mut *ac.offset((*px).n0 as isize) as *mut u8_0 as
                    *mut libc::c_char;
            _Litob(px, 'x' as i32 as u8_0);
        }
        115 => {
            (*px).s = (*pap).arg::<*mut libc::c_char>();
            (*px).n1 = strlen((*px).s) as s32;
            if (*px).prec >= 0 as libc::c_int && (*px).n1 > (*px).prec {
                (*px).n1 = (*px).prec
            }
        }
        37 => {
            let fresh10 = (*px).n0;
            (*px).n0 = (*px).n0 + 1;
            *ac.offset(fresh10 as isize) = '%' as i32 as u8_0
        }
        _ => {
            let fresh11 = (*px).n0;
            (*px).n0 = (*px).n0 + 1;
            *ac.offset(fresh11 as isize) = code
        }
    };
}
