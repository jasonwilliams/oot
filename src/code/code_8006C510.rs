#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type f32_0 = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransformData {
    pub unk_00: u16_0,
    pub unk_02: s16,
    pub unk_04: s16,
    pub unk_06: s16,
    pub unk_08: f32_0,
}
#[no_mangle]
pub unsafe extern "C" fn func_8006C510(mut arg0: f32_0, mut arg1: f32_0,
                                       mut arg2: f32_0, mut arg3: f32_0,
                                       mut arg4: f32_0, mut arg5: f32_0)
 -> f32_0 {
    let mut pad: [libc::c_char; 28] = [0; 28];
    let mut sq: f32_0 = arg0 * arg0;
    let mut cube: f32_0 = sq * arg0;
    return (cube + cube - sq * 3.0f32 + 1.0f32) * arg2 +
               (sq * 3.0f32 - (cube + cube)) * arg3 +
               (cube - (sq + sq) + arg0) * arg4 * arg1 +
               (cube - sq) * arg5 * arg1;
}
#[no_mangle]
pub unsafe extern "C" fn func_8006C5A8(mut target: f32_0,
                                       mut transData: *mut TransformData,
                                       mut refIdx: s32) -> f32_0 {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    if target <= (*transData).unk_02 as libc::c_int as libc::c_float {
        return (*transData).unk_08
    }
    if target >=
           (*transData.offset((refIdx - 1 as libc::c_int) as isize)).unk_02 as
               libc::c_int as libc::c_float {
        return (*transData.offset((refIdx - 1 as libc::c_int) as
                                      isize)).unk_08
    }
    i = 0 as libc::c_int;
    loop  {
        j = i + 1 as libc::c_int;
        if (*transData.offset(j as isize)).unk_02 as libc::c_int as
               libc::c_float > target {
            if (*transData.offset(i as isize)).unk_00 as libc::c_int &
                   1 as libc::c_int != 0 {
                return (*transData.offset(i as isize)).unk_08
            } else if (*transData.offset(i as isize)).unk_00 as libc::c_int &
                          2 as libc::c_int != 0 {
                return (*transData.offset(i as isize)).unk_08 +
                           (target -
                                (*transData.offset(i as isize)).unk_02 as
                                    f32_0) /
                               ((*transData.offset(j as isize)).unk_02 as
                                    f32_0 -
                                    (*transData.offset(i as isize)).unk_02 as
                                        f32_0) *
                               ((*transData.offset(j as isize)).unk_08 -
                                    (*transData.offset(i as isize)).unk_08)
            } else {
                let mut diff: f32_0 =
                    (*transData.offset(j as isize)).unk_02 as f32_0 -
                        (*transData.offset(i as isize)).unk_02 as f32_0;
                return func_8006C510((target -
                                          (*transData.offset(i as
                                                                 isize)).unk_02
                                              as libc::c_int as libc::c_float)
                                         /
                                         ((*transData.offset(j as
                                                                 isize)).unk_02
                                              as f32_0 -
                                              (*transData.offset(i as
                                                                     isize)).unk_02
                                                  as libc::c_int as
                                                  libc::c_float),
                                     diff * (1.0f32 / 30.0f32),
                                     (*transData.offset(i as isize)).unk_08,
                                     (*transData.offset(j as isize)).unk_08,
                                     (*transData.offset(i as isize)).unk_06 as
                                         f32_0,
                                     (*transData.offset(j as isize)).unk_04 as
                                         f32_0)
            }
        }
        i += 1
    };
}
