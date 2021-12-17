#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
pub type s16 = libc::c_short;
pub type s32 = libc::c_int;
pub type f32_0 = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vtx_t {
    pub ob: [libc::c_short; 3],
    pub flag: libc::c_ushort,
    pub tc: [libc::c_short; 2],
    pub cn: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vtx_tn {
    pub ob: [libc::c_short; 3],
    pub flag: libc::c_ushort,
    pub tc: [libc::c_short; 2],
    pub n: [libc::c_schar; 3],
    pub a: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vtx {
    pub v: Vtx_t,
    pub n: Vtx_tn,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PauseMapMarkPoint {
    pub chestFlag: s16,
    pub x: f32_0,
    pub y: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PauseMapMarkData {
    pub markType: s16,
    pub unk_04: s32,
    pub vtx: *const Vtx,
    pub vtxCount: s32,
    pub count: s32,
    pub points: [PauseMapMarkPoint; 12],
}
pub type PauseMapMarksData = [PauseMapMarkData; 3];
static mut sMarkBossVtx: [Vtx; 4] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4 as libc::c_int) as libc::c_short,
                                4 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4 as libc::c_int) as libc::c_short,
                                -(4 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                256 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4 as libc::c_int as libc::c_short,
                                4 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4 as libc::c_int as libc::c_short,
                                -(4 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                256 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
static mut sMarkChestVtx: [Vtx; 4] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4 as libc::c_int) as libc::c_short,
                                4 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4 as libc::c_int) as libc::c_short,
                                -(4 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                256 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4 as libc::c_int as libc::c_short,
                                4 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4 as libc::c_int as libc::c_short,
                                -(4 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                256 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
#[no_mangle]
pub static mut gPauseMapMarkDataTable: [PauseMapMarksData; 34] =
    unsafe {
        [[{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 2 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 40.0f32,
                                                                  y:
                                                                      -33.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      6 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 49.0f32,
                                                                  y:
                                                                      -42.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 48.0f32,
                                                                  y:
                                                                      -63.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 84.0f32,
                                                                  y:
                                                                      -39.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 3 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 46.0f32,
                                                                  y:
                                                                      -59.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 77.0f32,
                                                                  y:
                                                                      -26.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 65.0f32,
                                                                  y:
                                                                      -61.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 55.0f32,
                                                                  y: 0.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 3 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 55.0f32,
                                                                  y:
                                                                      -36.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 54.0f32,
                                                                  y:
                                                                      -51.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 13.0f32,
                                                                  y:
                                                                      -61.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 3 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 47.0f32,
                                                                  y:
                                                                      -40.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 51.0f32,
                                                                  y:
                                                                      -3.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 47.0f32,
                                                                  y:
                                                                      -47.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 23.0f32,
                                                                  y:
                                                                      -25.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          }],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 5 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 48.0f32,
                                                                  y:
                                                                      -68.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 50.0f32,
                                                                  y:
                                                                      -66.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      7 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 55.0f32,
                                                                  y:
                                                                      -50.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      9 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 58.0f32,
                                                                  y: 1.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      10 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 62.0f32,
                                                                  y:
                                                                      -45.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 65.0f32,
                                                                  y:
                                                                      -37.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          }],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 6 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 37.0f32,
                                                                  y:
                                                                      -49.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 65.0f32,
                                                                  y:
                                                                      -38.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 52.0f32,
                                                                  y:
                                                                      -48.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 46.0f32,
                                                                  y:
                                                                      -36.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      6 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 59.0f32,
                                                                  y:
                                                                      -41.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      8 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 52.0f32,
                                                                  y:
                                                                      -26.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 6 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 53.0f32,
                                                                  y:
                                                                      -64.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 65.0f32,
                                                                  y:
                                                                      -9.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      12 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 49.0f32,
                                                                  y:
                                                                      -1.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      13 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 40.0f32,
                                                                  y: 0.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      14 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 18.0f32,
                                                                  y:
                                                                      -2.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      15 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 59.0f32,
                                                                  y: 0.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 4 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 49.0f32,
                                                                  y:
                                                                      -1.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 71.0f32,
                                                                  y:
                                                                      -13.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 11.0f32,
                                                                  y:
                                                                      -25.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      6 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 84.0f32,
                                                                  y:
                                                                      -16.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      9 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 65.0f32,
                                                                  y:
                                                                      -30.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      11 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 41.0f32,
                                                                  y:
                                                                      -24.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 50.0f32,
                                                                  y:
                                                                      -11.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          }],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 24.0f32,
                                                                  y:
                                                                      -40.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],},
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 3 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 75.0f32,
                                                                  y:
                                                                      -47.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      6 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 72.0f32,
                                                                  y:
                                                                      -51.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      8 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 65.0f32,
                                                                  y:
                                                                      -12.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      11 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 78.0f32,
                                                                  y:
                                                                      -35.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 5 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 67.0f32,
                                                                  y:
                                                                      -58.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 48.0f32,
                                                                  y:
                                                                      -30.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 63.0f32,
                                                                  y:
                                                                      -14.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      7 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 36.0f32,
                                                                  y:
                                                                      -45.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      12 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 47.0f32,
                                                                  y:
                                                                      -26.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 26.0f32,
                                                                  y:
                                                                      -34.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          }],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 88.0f32,
                                                                  y:
                                                                      -60.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 62.0f32,
                                                                  y:
                                                                      -23.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          }],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 88.0f32,
                                                                  y:
                                                                      -60.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 2 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 88.0f32,
                                                                  y:
                                                                      -60.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 49.0f32,
                                                                  y:
                                                                      -43.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      6 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 75.0f32,
                                                                  y:
                                                                      -65.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      18 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 46.0f32,
                                                                  y:
                                                                      -30.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 5 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 23.0f32,
                                                                  y:
                                                                      -33.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 56.0f32,
                                                                  y:
                                                                      -11.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 83.0f32,
                                                                  y:
                                                                      -25.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      24 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 84.0f32,
                                                                  y:
                                                                      -39.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      25 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 74.0f32,
                                                                  y:
                                                                      -37.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 47.0f32,
                                                                  y: 0.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          }],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 5 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 46.0f32,
                                                                  y:
                                                                      -20.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      6 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 28.0f32,
                                                                  y:
                                                                      -19.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      12 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 25.0f32,
                                                                  y:
                                                                      -25.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      15 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 50.0f32,
                                                                  y:
                                                                      -13.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      28 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 48.0f32,
                                                                  y:
                                                                      -29.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 9 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 14.0f32,
                                                                  y:
                                                                      -24.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 55.0f32,
                                                                  y:
                                                                      -14.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      7 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 78.0f32,
                                                                  y:
                                                                      -2.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      8 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 14.0f32,
                                                                  y:
                                                                      -16.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      26 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 42.0f32,
                                                                  y:
                                                                      -43.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      27 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 50.0f32,
                                                                  y:
                                                                      -43.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      29 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 25.0f32,
                                                                  y:
                                                                      -35.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      30 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 42.0f32,
                                                                  y:
                                                                      -36.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      31 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 50.0f32,
                                                                  y:
                                                                      -36.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 2 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 41.0f32,
                                                                  y:
                                                                      -17.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      7 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 27.0f32,
                                                                  y:
                                                                      -24.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 2 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 81.0f32,
                                                                  y:
                                                                      -20.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 74.0f32,
                                                                  y:
                                                                      -37.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 2 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      12 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 96.0f32,
                                                                  y:
                                                                      -51.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      16 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 46.0f32,
                                                                  y:
                                                                      -42.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      22 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 96.0f32,
                                                                  y:
                                                                      -55.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 12 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 43.0f32,
                                                                  y:
                                                                      -66.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      5 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 37.0f32,
                                                                  y:
                                                                      -66.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      6 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 33.0f32,
                                                                  y:
                                                                      -72.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      8 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 85.0f32,
                                                                  y:
                                                                      -18.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      9 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 61.0f32,
                                                                  y:
                                                                      -42.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      10 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 15.0f32,
                                                                  y:
                                                                      -4.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      11 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 25.0f32,
                                                                  y:
                                                                      -4.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      13 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 19.0f32,
                                                                  y:
                                                                      -29.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      14 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 78.0f32,
                                                                  y:
                                                                      -15.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      15 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 60.0f32,
                                                                  y:
                                                                      -70.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      21 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 92.0f32,
                                                                  y:
                                                                      -29.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      20 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 87.0f32,
                                                                  y:
                                                                      -20.0f32,};
                                            init
                                        }],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: 1 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkBossVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      -(1 as
                                                                            libc::c_int)
                                                                          as
                                                                          s16,
                                                                  x: 31.0f32,
                                                                  y:
                                                                      -45.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          }],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 2 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 84.0f32,
                                                                  y:
                                                                      -38.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 57.0f32,
                                                                  y:
                                                                      -18.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],},
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 1 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 72.0f32,
                                                                  y:
                                                                      -32.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}],
         [{
              let mut init =
                  PauseMapMarkData{markType: 0 as libc::c_int as s16,
                                   unk_04: 23 as libc::c_int,
                                   vtx: sMarkChestVtx.as_ptr(),
                                   vtxCount: 4 as libc::c_int,
                                   count: 3 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 66.0f32,
                                                                  y:
                                                                      -2.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 77.0f32,
                                                                  y:
                                                                      -46.0f32,};
                                            init
                                        },
                                        {
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 27.0f32,
                                                                  y:
                                                                      -45.0f32,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          {
              let mut init =
                  PauseMapMarkData{markType: -(1 as libc::c_int) as s16,
                                   unk_04: 0 as libc::c_int,
                                   vtx: 0 as *const Vtx,
                                   vtxCount: 0 as libc::c_int,
                                   count: 0 as libc::c_int,
                                   points:
                                       [{
                                            let mut init =
                                                PauseMapMarkPoint{chestFlag:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          s16,
                                                                  x: 0.,
                                                                  y: 0.,};
                                            init
                                        },
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,},
                                        PauseMapMarkPoint{chestFlag: 0,
                                                          x: 0.,
                                                          y: 0.,}],};
              init
          },
          PauseMapMarkData{markType: 0,
                           unk_04: 0,
                           vtx: 0 as *const Vtx,
                           vtxCount: 0,
                           count: 0,
                           points:
                               [PauseMapMarkPoint{chestFlag: 0,
                                                  x: 0.,
                                                  y: 0.,}; 12],}]]
    };
