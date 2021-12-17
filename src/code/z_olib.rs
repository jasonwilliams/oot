#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn fabsf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
}
pub type s16 = libc::c_short;
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
pub struct VecSph {
    pub r: f32_0,
    pub pitch: s16,
    pub yaw: s16,
}
/* *
 * Calculates the distances between `a` and `b`
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDist(mut a: *mut Vec3f, mut b: *mut Vec3f)
 -> f32_0 {
    let mut dx: f32_0 = (*a).x - (*b).x;
    let mut dy: f32_0 = (*a).y - (*b).y;
    let mut dz: f32_0 = (*a).z - (*b).z;
    return sqrtf(dx * dx + dy * dy + dz * dz);
}
/* *
 * Calculates the distances between `a` and `b`, and outputs the vector
 * created by the difference into `dest`
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDistOutDiff(mut a: *mut Vec3f,
                                               mut b: *mut Vec3f,
                                               mut dest: *mut Vec3f)
 -> f32_0 {
    (*dest).x = (*a).x - (*b).x;
    (*dest).y = (*a).y - (*b).y;
    (*dest).z = (*a).z - (*b).z;
    return sqrtf((*dest).x * (*dest).x + (*dest).y * (*dest).y +
                     (*dest).z * (*dest).z);
}
/* *
 * Calculates the distances on the xz plane between `a` and `b`
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDistXZ(mut a: *mut Vec3f,
                                          mut b: *mut Vec3f) -> f32_0 {
    return sqrtf(((*a).x - (*b).x) * ((*a).x - (*b).x) +
                     ((*a).z - (*b).z) * ((*a).z - (*b).z));
}
/* *
 * Clamps `val` to a maximum of -`min` as `val` approaches zero, and a minimum of
 * `min` as `val` approaches zero
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_ClampMinDist(mut val: f32_0, mut min: f32_0)
 -> f32_0 {
    return if min <= fabsf(val) {
               val
           } else if val >= 0 as libc::c_int as libc::c_float {
               min
           } else { -min };
}
/* *
 * Clamps `val` to a minimum of -`max` as `val` approaches -`max`, and a maximum of `max`
 * as `val` approaches `max`
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_ClampMaxDist(mut val: f32_0, mut max: f32_0)
 -> f32_0 {
    return if fabsf(val) <= max {
               val
           } else if val >= 0 as libc::c_int as libc::c_float {
               max
           } else { -max };
}
/* *
 * Takes the difference of points b and a, and creates a normal vector
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDistNormalize(mut dest: *mut Vec3f,
                                                 mut a: *mut Vec3f,
                                                 mut b: *mut Vec3f)
 -> *mut Vec3f {
    let mut v1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut v2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut dist: f32_0 = 0.;
    v1.x = (*b).x - (*a).x;
    v1.y = (*b).y - (*a).y;
    v1.z = (*b).z - (*a).z;
    dist =
        OLib_ClampMinDist(sqrtf(v1.x * v1.x + v1.y * v1.y + v1.z * v1.z),
                          0.01f32);
    v2.x = v1.x / dist;
    v2.y = v1.y / dist;
    v2.z = v1.z / dist;
    *dest = v2;
    return dest;
}
/* *
 * Takes the spherical coordinate `sph`, and converts it into a x,y,z position
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_VecSphToVec3f(mut dest: *mut Vec3f,
                                            mut sph: *mut VecSph)
 -> *mut Vec3f {
    let mut v: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sinPitch: f32_0 = 0.;
    let mut cosPitch: f32_0 = Math_CosS((*sph).pitch);
    let mut sinYaw: f32_0 = 0.;
    let mut cosYaw: f32_0 = Math_CosS((*sph).yaw);
    sinPitch = Math_SinS((*sph).pitch);
    sinYaw = Math_SinS((*sph).yaw);
    v.x = (*sph).r * sinPitch * sinYaw;
    v.y = (*sph).r * cosPitch;
    v.z = (*sph).r * sinPitch * cosYaw;
    *dest = v;
    return dest;
}
/* *
 * Takes the geographic point `sph` and converts it into a x,y,z position
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_VecSphGeoToVec3f(mut dest: *mut Vec3f,
                                               mut sph: *mut VecSph)
 -> *mut Vec3f {
    let mut geo: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    geo.r = (*sph).r;
    geo.pitch = (0x3fff as libc::c_int - (*sph).pitch as libc::c_int) as s16;
    geo.yaw = (*sph).yaw;
    return OLib_VecSphToVec3f(dest, &mut geo);
}
/* *
 * Takes the point `vec`, and converts it into a spherical coordinate
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fToVecSph(mut dest: *mut VecSph,
                                            mut vec: *mut Vec3f)
 -> *mut VecSph {
    let mut sph: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut distSquared: f32_0 = (*vec).x * (*vec).x + (*vec).z * (*vec).z;
    let mut dist: f32_0 = sqrtf(distSquared);
    if dist == 0.0f32 && (*vec).y == 0.0f32 {
        sph.pitch = 0 as libc::c_int as s16
    } else {
        sph.pitch =
            (Math_FAtan2F(dist, (*vec).y) *
                 (180.0f32 / 3.14159265358979323846f32) * 182.04167f32 +
                 0.5f32) as s16
    }
    sph.r = sqrtf((*vec).y * (*vec).y + distSquared);
    if (*vec).x == 0.0f32 && (*vec).z == 0.0f32 {
        sph.yaw = 0 as libc::c_int as s16
    } else {
        sph.yaw =
            (Math_FAtan2F((*vec).x, (*vec).z) *
                 (180.0f32 / 3.14159265358979323846f32) * 182.04167f32 +
                 0.5f32) as s16
    }
    *dest = sph;
    return dest;
}
/* *
 * Takes the point `vec`, and converts it to a geographic coordinate
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fToVecSphGeo(mut dest: *mut VecSph,
                                               mut vec: *mut Vec3f)
 -> *mut VecSph {
    let mut sph: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    OLib_Vec3fToVecSph(&mut sph, vec);
    sph.pitch = (0x3fff as libc::c_int - sph.pitch as libc::c_int) as s16;
    *dest = sph;
    return dest;
}
/* *
 * Takes the differences of positions `a` and `b`, and converts them to spherical coordinates
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDiffToVecSph(mut dest: *mut VecSph,
                                                mut a: *mut Vec3f,
                                                mut b: *mut Vec3f)
 -> *mut VecSph {
    let mut sph: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    sph.x = (*b).x - (*a).x;
    sph.y = (*b).y - (*a).y;
    sph.z = (*b).z - (*a).z;
    return OLib_Vec3fToVecSph(dest, &mut sph);
}
/* *
 * Takes the difference of positions `a` and `b`, and converts them to geographic coordinates
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDiffToVecSphGeo(mut dest: *mut VecSph,
                                                   mut a: *mut Vec3f,
                                                   mut b: *mut Vec3f)
 -> *mut VecSph {
    let mut sph: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    sph.x = (*b).x - (*a).x;
    sph.y = (*b).y - (*a).y;
    sph.z = (*b).z - (*a).z;
    return OLib_Vec3fToVecSphGeo(dest, &mut sph);
}
/* *
 * Gets the pitch/yaw of the vector formed from `b`-`a`, result is in radians
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDiffRad(mut dest: *mut Vec3f,
                                           mut a: *mut Vec3f,
                                           mut b: *mut Vec3f) -> *mut Vec3f {
    let mut anglesRad: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    anglesRad.x = Math_FAtan2F((*b).z - (*a).z, (*b).y - (*a).y);
    anglesRad.y = Math_FAtan2F((*b).x - (*a).x, (*b).z - (*a).z);
    anglesRad.z = 0 as libc::c_int as f32_0;
    *dest = anglesRad;
    return dest;
}
/* *
 * Gets the pitch/yaw of the vector formed from `b`-`a`, result is in degrees
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDiffDegF(mut dest: *mut Vec3f,
                                            mut a: *mut Vec3f,
                                            mut b: *mut Vec3f) -> *mut Vec3f {
    let mut anglesRad: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut anglesDegrees: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    OLib_Vec3fDiffRad(&mut anglesRad, a, b);
    anglesDegrees.x = anglesRad.x * (180.0f32 / 3.14159265358979323846f32);
    anglesDegrees.y = anglesRad.y * (180.0f32 / 3.14159265358979323846f32);
    anglesDegrees.z = 0.0f32;
    *dest = anglesDegrees;
    return dest;
}
/* *
 * Gets the pitch/yaw of the vector formed from `b`-`a`, result is in binary degrees
 */
#[no_mangle]
pub unsafe extern "C" fn OLib_Vec3fDiffBinAng(mut dest: *mut Vec3s,
                                              mut a: *mut Vec3f,
                                              mut b: *mut Vec3f)
 -> *mut Vec3s {
    let mut anglesRad: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut anglesBinAng: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    OLib_Vec3fDiffRad(&mut anglesRad, a, b);
    anglesBinAng.x =
        (anglesRad.x * (180.0f32 / 3.14159265358979323846f32) * 182.04167f32 +
             0.5f32) as s16;
    anglesBinAng.y =
        (anglesRad.y * (180.0f32 / 3.14159265358979323846f32) * 182.04167f32 +
             0.5f32) as s16;
    anglesBinAng.z = 0.0f32 as s16;
    *dest = anglesBinAng;
    return dest;
}
