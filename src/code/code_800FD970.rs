#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u32_0 = libc::c_uint;
pub type f32_0 = libc::c_float;
// The latest generated random number, used to generate the next number in the sequence.
static mut sRandInt: u32_0 = 1 as libc::c_int as u32_0;
// Space to store a value to be re-interpreted as a float.
static mut sRandFloat: u32_0 = 0;
/* *
 * Gets the next integer in the sequence of pseudo-random numbers.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_Next() -> u32_0 {
    sRandInt =
        sRandInt.wrapping_mul(1664525 as libc::c_int as
                                  libc::c_uint).wrapping_add(1013904223 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint);
    return sRandInt;
}
/* *
 * Seeds the pseudo-random number generator by providing a starting value.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_Seed(mut seed: u32_0) { sRandInt = seed; }
/* *
 * Returns a pseudo-random floating-point number between 0.0f and 1.0f, by generating
 * the next integer and masking it to an IEEE-754 compliant floating-point number
 * between 1.0f and 2.0f, returning the result subtract 1.0f.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_ZeroOne() -> f32_0 {
    sRandInt =
        sRandInt.wrapping_mul(1664525 as libc::c_int as
                                  libc::c_uint).wrapping_add(1013904223 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint);
    sRandFloat =
        sRandInt >> 9 as libc::c_int |
            0x3f800000 as libc::c_int as libc::c_uint;
    return *(&mut sRandFloat as *mut u32_0 as *mut f32_0) - 1.0f32;
}
/* *
 * Returns a pseudo-random floating-point number between -0.5f and 0.5f by the same
 * manner in which Rand_ZeroOne generates its result.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_Centered() -> f32_0 {
    sRandInt =
        sRandInt.wrapping_mul(1664525 as libc::c_int as
                                  libc::c_uint).wrapping_add(1013904223 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint);
    sRandFloat =
        sRandInt >> 9 as libc::c_int |
            0x3f800000 as libc::c_int as libc::c_uint;
    return *(&mut sRandFloat as *mut u32_0 as *mut f32_0) - 1.5f32;
}
/* *
 * Seeds a pseudo-random number at rndNum with a provided seed.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_Seed_Variable(mut rndNum: *mut u32_0,
                                            mut seed: u32_0) {
    *rndNum = seed;
}
/* *
 * Generates the next pseudo-random integer from the provided rndNum.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_Next_Variable(mut rndNum: *mut u32_0) -> u32_0 {
    *rndNum =
        (*rndNum).wrapping_mul(1664525 as libc::c_int as
                                   libc::c_uint).wrapping_add(1013904223 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint);
    return *rndNum;
}
/* *
 * Generates the next pseudo-random floating-point number between 0.0f and
 * 1.0f from the provided rndNum.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_ZeroOne_Variable(mut rndNum: *mut u32_0)
 -> f32_0 {
    let mut next: u32_0 =
        (*rndNum).wrapping_mul(1664525 as libc::c_int as
                                   libc::c_uint).wrapping_add(1013904223 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint);
    // clang-format off
    *rndNum = next;
    sRandFloat =
        next >> 9 as libc::c_int | 0x3f800000 as libc::c_int as libc::c_uint;
    // clang-format on
    return *(&mut sRandFloat as *mut u32_0 as *mut f32_0) - 1.0f32;
}
/* *
 * Generates the next pseudo-random floating-point number between -0.5f and
 * 0.5f from the provided rndNum.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_Centered_Variable(mut rndNum: *mut u32_0)
 -> f32_0 {
    let mut next: u32_0 =
        (*rndNum).wrapping_mul(1664525 as libc::c_int as
                                   libc::c_uint).wrapping_add(1013904223 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint);
    // clang-format off
    *rndNum = next;
    sRandFloat =
        next >> 9 as libc::c_int | 0x3f800000 as libc::c_int as libc::c_uint;
    // clang-format on
    return *(&mut sRandFloat as *mut u32_0 as *mut f32_0) - 1.5f32;
}
