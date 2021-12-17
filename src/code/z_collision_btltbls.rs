#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DamageTable {
    pub table: [u8_0; 32],
}
static mut sDamageTablePresets: [DamageTable; 23] =
    [{
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0xe as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0xe as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (3 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (8 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (8 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (3 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (6 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (6 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (6 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (3 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (6 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (6 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (6 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (6 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (3 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (8 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0xe as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (9 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (8 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (4 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (8 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x2 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0xe as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0xd as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x3 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0xf as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (1 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     },
     {
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0],};
         init
     }];
// Gets the pointer to one of the 23 preset damage tables. Returns NULL if index is out of range.
#[no_mangle]
pub unsafe extern "C" fn DamageTable_Get(mut index: s32) -> *mut DamageTable {
    if !(0 as libc::c_int <= index &&
             index <
                 (::std::mem::size_of::<[DamageTable; 23]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<DamageTable>()
                                                      as libc::c_ulong) as
                     s32) {
        osSyncPrintf(b"CollisionBtlTbl_get():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\n\x00"
                         as *const u8 as *const libc::c_char); // "Index over"
        return 0 as *mut DamageTable
    }
    return &mut *sDamageTablePresets.as_mut_ptr().offset(index as isize) as
               *mut DamageTable;
}
// Sets all entries in the damage table to 0x00
#[no_mangle]
pub unsafe extern "C" fn DamageTable_Clear(mut table: *mut DamageTable) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        (*table).table[i as usize] = 0 as libc::c_int as u8_0;
        i += 1
    };
}
