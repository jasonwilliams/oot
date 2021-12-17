#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    static mut osResetType: u32_0;
}
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type OSTime = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreNmiBuff {
    pub resetting: u32_0,
    pub resetCount: u32_0,
    pub duration: OSTime,
    pub resetTime: OSTime,
}
#[no_mangle]
pub unsafe extern "C" fn PreNmiBuff_Init(mut this: *mut PreNmiBuff) {
    (*this).resetting = 0 as libc::c_int as u32_0;
    if osResetType == 0 as libc::c_int as libc::c_uint {
        (*this).resetCount = 0 as libc::c_int as u32_0;
        (*this).duration = 0 as libc::c_int as OSTime
    } else {
        (*this).resetCount = (*this).resetCount.wrapping_add(1);
        (*this).duration =
            ((*this).duration as
                 libc::c_ulonglong).wrapping_add((*this).resetTime) as OSTime
                as OSTime
    }
    (*this).resetTime = 0 as libc::c_int as OSTime;
}
#[no_mangle]
pub unsafe extern "C" fn PreNmiBuff_SetReset(mut this: *mut PreNmiBuff) {
    (*this).resetting = 1 as libc::c_int as u32_0;
    (*this).resetTime = osGetTime();
}
#[no_mangle]
pub unsafe extern "C" fn PreNmiBuff_IsResetting(mut this: *mut PreNmiBuff)
 -> u32_0 {
    return (*this).resetting;
}
