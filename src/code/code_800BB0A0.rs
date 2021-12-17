#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s8 = libc::c_schar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type f32_0 = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3f {
    pub x: f32_0,
    pub y: f32_0,
    pub z: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3s {
    pub x: s16,
    pub y: s16,
    pub z: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CutsceneCameraPoint {
    pub continueFlag: s8,
    pub cameraRoll: s8,
    pub nextPointFrame: u16_0,
    pub viewAngle: f32_0,
    pub pos: Vec3s,
}
// The code in this file is very similar to a spline system used in Super Mario 64 for cutscene camera movement
#[no_mangle]
pub unsafe extern "C" fn func_800BB0A0(mut u: f32_0, mut pos: *mut Vec3f,
                                       mut roll: *mut f32_0,
                                       mut viewAngle: *mut f32_0,
                                       mut point0: *mut f32_0,
                                       mut point1: *mut f32_0,
                                       mut point2: *mut f32_0,
                                       mut point3: *mut f32_0) {
    let mut coeff: [f32_0; 4] = [0.; 4];
    u = if u > 1.0f32 { 1.0f32 } else { u };
    coeff[0 as libc::c_int as usize] =
        (1.0f32 - u) * (1.0f32 - u) * (1.0f32 - u) / 6.0f32;
    coeff[1 as libc::c_int as usize] =
        u * u * u / 2.0f32 - u * u + 2.0f32 / 3.0f32;
    coeff[2 as libc::c_int as usize] =
        -u * u * u / 2.0f32 + u * u / 2.0f32 + u / 2.0f32 + 1.0f32 / 6.0f32;
    coeff[3 as libc::c_int as usize] = u * u * u / 6.0f32;
    (*pos).x =
        coeff[0 as libc::c_int as usize] *
            *point0.offset(0 as libc::c_int as isize) +
            coeff[1 as libc::c_int as usize] *
                *point1.offset(0 as libc::c_int as isize) +
            coeff[2 as libc::c_int as usize] *
                *point2.offset(0 as libc::c_int as isize) +
            coeff[3 as libc::c_int as usize] *
                *point3.offset(0 as libc::c_int as isize);
    (*pos).y =
        coeff[0 as libc::c_int as usize] *
            *point0.offset(1 as libc::c_int as isize) +
            coeff[1 as libc::c_int as usize] *
                *point1.offset(1 as libc::c_int as isize) +
            coeff[2 as libc::c_int as usize] *
                *point2.offset(1 as libc::c_int as isize) +
            coeff[3 as libc::c_int as usize] *
                *point3.offset(1 as libc::c_int as isize);
    (*pos).z =
        coeff[0 as libc::c_int as usize] *
            *point0.offset(2 as libc::c_int as isize) +
            coeff[1 as libc::c_int as usize] *
                *point1.offset(2 as libc::c_int as isize) +
            coeff[2 as libc::c_int as usize] *
                *point2.offset(2 as libc::c_int as isize) +
            coeff[3 as libc::c_int as usize] *
                *point3.offset(2 as libc::c_int as isize);
    *roll =
        coeff[0 as libc::c_int as usize] *
            *point0.offset(3 as libc::c_int as isize) +
            coeff[1 as libc::c_int as usize] *
                *point1.offset(3 as libc::c_int as isize) +
            coeff[2 as libc::c_int as usize] *
                *point2.offset(3 as libc::c_int as isize) +
            coeff[3 as libc::c_int as usize] *
                *point3.offset(3 as libc::c_int as isize);
    *viewAngle =
        coeff[0 as libc::c_int as usize] *
            *point0.offset(4 as libc::c_int as isize) +
            coeff[1 as libc::c_int as usize] *
                *point1.offset(4 as libc::c_int as isize) +
            coeff[2 as libc::c_int as usize] *
                *point2.offset(4 as libc::c_int as isize) +
            coeff[3 as libc::c_int as usize] *
                *point3.offset(4 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn func_800BB2B4(mut pos: *mut Vec3f,
                                       mut roll: *mut f32_0,
                                       mut fov: *mut f32_0,
                                       mut point: *mut CutsceneCameraPoint,
                                       mut keyFrame: *mut s16,
                                       mut curFrame: *mut f32_0) -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    let mut pointData: [[f32_0; 5]; 4] = [[0.; 5]; 4];
    let mut i: s32 = 0;
    let mut progress: f32_0 = *curFrame;
    let mut key: s32 = *keyFrame as s32;
    let mut speed1: f32_0 = 0.0f32;
    let mut speed2: f32_0 = 0.0f32;
    let mut advance: f32_0 = 0.;
    if key < 0 as libc::c_int { progress = 0.0f32 }
    if (*point.offset(key as isize)).continueFlag as libc::c_int ==
           -(1 as libc::c_int) ||
           (*point.offset((key + 1 as libc::c_int) as isize)).continueFlag as
               libc::c_int == -(1 as libc::c_int) ||
           (*point.offset((key + 2 as libc::c_int) as isize)).continueFlag as
               libc::c_int == -(1 as libc::c_int) {
        return 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        pointData[i as usize][0 as libc::c_int as usize] =
            (*point.offset((key + i) as isize)).pos.x as f32_0;
        pointData[i as usize][1 as libc::c_int as usize] =
            (*point.offset((key + i) as isize)).pos.y as f32_0;
        pointData[i as usize][2 as libc::c_int as usize] =
            (*point.offset((key + i) as isize)).pos.z as f32_0;
        pointData[i as usize][3 as libc::c_int as usize] =
            (*point.offset((key + i) as isize)).cameraRoll as f32_0;
        pointData[i as usize][4 as libc::c_int as usize] =
            (*point.offset((key + i) as isize)).viewAngle;
        i += 1
    }
    func_800BB0A0(progress, pos, roll, fov,
                  pointData[0 as libc::c_int as usize].as_mut_ptr(),
                  pointData[1 as libc::c_int as usize].as_mut_ptr(),
                  pointData[2 as libc::c_int as usize].as_mut_ptr(),
                  pointData[3 as libc::c_int as usize].as_mut_ptr());
    if (*point.offset((*keyFrame as libc::c_int + 1 as libc::c_int) as
                          isize)).nextPointFrame as libc::c_int !=
           0 as libc::c_int {
        speed1 =
            1.0f32 /
                (*point.offset((*keyFrame as libc::c_int + 1 as libc::c_int)
                                   as isize)).nextPointFrame as libc::c_int as
                    libc::c_float
    }
    if (*point.offset((*keyFrame as libc::c_int + 2 as libc::c_int) as
                          isize)).nextPointFrame as libc::c_int !=
           0 as libc::c_int {
        speed2 =
            1.0f32 /
                (*point.offset((*keyFrame as libc::c_int + 2 as libc::c_int)
                                   as isize)).nextPointFrame as libc::c_int as
                    libc::c_float
    }
    advance = *curFrame * (speed2 - speed1) + speed1;
    if advance < 0.0f32 { advance = 0 as libc::c_int as f32_0 }
    *curFrame += advance;
    if *curFrame >= 1 as libc::c_int as libc::c_float {
        *keyFrame += 1;
        if (*point.offset((*keyFrame as libc::c_int + 3 as libc::c_int) as
                              isize)).continueFlag as libc::c_int ==
               -(1 as libc::c_int) {
            *keyFrame = 0 as libc::c_int as s16;
            ret = 1 as libc::c_int
        }
        *curFrame -= 1 as libc::c_int as libc::c_float
    }
    return ret;
}
