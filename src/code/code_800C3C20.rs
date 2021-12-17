#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn Audio_StopSfxByBank(bankId: u8_0);
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
#[no_mangle]
pub static mut D_8012D200: [u8_0; 7] =
    [0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     4 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn func_800C3C20() {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while (i <
               (::std::mem::size_of::<[u8_0; 7]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                    as libc::c_ulong) as s32)
              as libc::c_int as libc::c_uint & 0xffffffff as libc::c_uint != 0
          {
        Audio_StopSfxByBank(D_8012D200[i as usize]);
        i += 1
    };
}
