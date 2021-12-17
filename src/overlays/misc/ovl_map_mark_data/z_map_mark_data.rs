#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapMarkPoint {
    pub chestFlag: s8,
    pub x: u8_0,
    pub y: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapMarkIconData {
    pub markType: s8,
    pub count: u8_0,
    pub points: [MapMarkPoint; 12],
}
pub type MapMarkData = [MapMarkIconData; 3];
static mut sMapMarkDekuTree: [MapMarkData; 13] =
    [[{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            50 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            64 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            4 as libc::c_int
                                                                as s8,
                                                        x:
                                                            76 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            37 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x:
                                                            46 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            50 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            76 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            52 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            50 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            23 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            46 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            50 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            6 as libc::c_int
                                                                as s8,
                                                        x:
                                                            58 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            60 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkDodongosCavern: [MapMarkData; 19] =
    [[{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x:
                                                            69 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            14 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            4 as libc::c_int
                                                                as s8,
                                                        x:
                                                            69 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            30 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            54 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            69 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            37 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            49 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            59 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            53 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            68 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkJabuJabuBelly: [MapMarkData; 16] =
    [[{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            66 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            50 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            72 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            47 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            7 as libc::c_int
                                                                as s8,
                                                        x:
                                                            72 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            4 as libc::c_int
                                                                as s8,
                                                        x:
                                                            64 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            8 as libc::c_int
                                                                as s8,
                                                        x:
                                                            79 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            38 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            10 as libc::c_int
                                                                as s8,
                                                        x:
                                                            64 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            45 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            67 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            32 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      }],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            9 as libc::c_int
                                                                as s8,
                                                        x:
                                                            68 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            45 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            79 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            33 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            6 as libc::c_int
                                                                as s8,
                                                        x:
                                                            61 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            41 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x:
                                                            48 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            57 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            55 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkForestTemple: [MapMarkData; 27] =
    [[{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x:
                                                            72 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            57 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            69 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            39 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            9 as libc::c_int
                                                                as s8,
                                                        x:
                                                            62 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            65 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            9 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            59 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            13 as libc::c_int
                                                                as s8,
                                                        x:
                                                            80 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            53 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            15 as libc::c_int
                                                                as s8,
                                                        x:
                                                            49 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            50 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            6 as libc::c_int
                                                                as s8,
                                                        x:
                                                            65 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            53 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            11 as libc::c_int
                                                                as s8,
                                                        x:
                                                            39 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            35 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            53 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            5 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      }],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            65 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            14 as libc::c_int
                                                                as s8,
                                                        x:
                                                            64 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            31 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            75 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            53 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            12 as libc::c_int
                                                                as s8,
                                                        x:
                                                            69 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            52 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            58 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            27 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkFireTemple: [MapMarkData; 38] =
    [[{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            7 as libc::c_int
                                                                as s8,
                                                        x:
                                                            53 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            70 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            40 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            47 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      }],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            11 as libc::c_int
                                                                as s8,
                                                        x:
                                                            57 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            48 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            67 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            73 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            6 as libc::c_int
                                                                as s8,
                                                        x:
                                                            58 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            76 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            78 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            12 as libc::c_int
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            58 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            4 as libc::c_int
                                                                as s8,
                                                        x:
                                                            60 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            72 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            68 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            8 as libc::c_int
                                                                as s8,
                                                        x:
                                                            66 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            57 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            51 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            61 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkWaterTemple: [MapMarkData; 44] =
    [[{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            6 as libc::c_int
                                                                as s8,
                                                        x:
                                                            81 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            68 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            75 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            55 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            40 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            74 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            61 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            73 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            65 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x:
                                                            73 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            63 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkSpiritTemple: [MapMarkData; 33] =
    [[{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 4 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            26 as libc::c_int
                                                                as s8,
                                                        x:
                                                            27 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            35 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            27 as libc::c_int
                                                                as s8,
                                                        x:
                                                            36 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            35 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            30 as libc::c_int
                                                                as s8,
                                                        x:
                                                            27 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            28 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            31 as libc::c_int
                                                                as s8,
                                                        x:
                                                            36 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            28 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            29 as libc::c_int
                                                                as s8,
                                                        x:
                                                            67 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            63 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            8 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            48 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            56 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            15 as libc::c_int
                                                                as s8,
                                                        x:
                                                            69 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            42 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            28 as libc::c_int
                                                                as s8,
                                                        x:
                                                            60 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            76 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            40 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            7 as libc::c_int
                                                                as s8,
                                                        x:
                                                            70 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            53 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            4 as libc::c_int
                                                                as s8,
                                                        x:
                                                            68 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            42 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            25 as libc::c_int
                                                                as s8,
                                                        x:
                                                            78 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            58 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            24 as libc::c_int
                                                                as s8,
                                                        x:
                                                            78 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            58 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            55 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            18 as libc::c_int
                                                                as s8,
                                                        x:
                                                            75 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            54 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            6 as libc::c_int
                                                                as s8,
                                                        x:
                                                            78 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            55 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            12 as libc::c_int
                                                                as s8,
                                                        x:
                                                            70 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            70 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            76 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            37 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            57 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            23 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      }],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkShadowTemple: [MapMarkData; 27] =
    [[{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            64 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 1 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            -(1 as
                                                                  libc::c_int)
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            76 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            7 as libc::c_int
                                                                as s8,
                                                        x:
                                                            76 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            65 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            83 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            67 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            14 as libc::c_int
                                                                as s8,
                                                        x:
                                                            84 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            59 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            76 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            67 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 3 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            4 as libc::c_int
                                                                as s8,
                                                        x:
                                                            78 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            5 as libc::c_int
                                                                as s8,
                                                        x:
                                                            74 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            6 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            68 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            9 as libc::c_int
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            64 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            10 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            65 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            11 as libc::c_int
                                                                as s8,
                                                        x:
                                                            80 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            65 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            16 as libc::c_int
                                                                as s8,
                                                        x:
                                                            73 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            64 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            12 as libc::c_int
                                                                as s8,
                                                        x:
                                                            87 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            64 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            22 as libc::c_int
                                                                as s8,
                                                        x:
                                                            87 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            68 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            13 as libc::c_int
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            66 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            21 as libc::c_int
                                                                as s8,
                                                        x:
                                                            78 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            66 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 2 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            8 as libc::c_int
                                                                as s8,
                                                        x:
                                                            76 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            66 as libc::c_int
                                                                as u8_0,};
                                       init
                                   },
                                   {
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            20 as libc::c_int
                                                                as s8,
                                                        x:
                                                            78 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            68 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            14 as libc::c_int
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            15 as libc::c_int
                                                                as s8,
                                                        x:
                                                            56 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            67 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            16 as libc::c_int
                                                                as s8,
                                                        x:
                                                            73 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            64 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            14 as libc::c_int
                                                                as s8,
                                                        x:
                                                            77 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            62 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkBottomWell: [MapMarkData; 10] =
    [[{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            3 as libc::c_int
                                                                as s8,
                                                        x:
                                                            60 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            18 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            73 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            61 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            74 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            66 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
static mut sMapMarkIceCavern: [MapMarkData; 12] =
    [[{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            2 as libc::c_int
                                                                as s8,
                                                        x:
                                                            71 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            59 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x:
                                                            48 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            36 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points: [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],},
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}],
     [{
          let mut init =
              MapMarkIconData{markType: 0 as libc::c_int as s8,
                              count: 1 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            1 as libc::c_int
                                                                as s8,
                                                        x:
                                                            73 as libc::c_int
                                                                as u8_0,
                                                        y:
                                                            67 as libc::c_int
                                                                as u8_0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      {
          let mut init =
              MapMarkIconData{markType: -(1 as libc::c_int) as s8,
                              count: 0 as libc::c_int as u8_0,
                              points:
                                  [{
                                       let mut init =
                                           MapMarkPoint{chestFlag:
                                                            0 as libc::c_int
                                                                as s8,
                                                        x: 0,
                                                        y: 0,};
                                       init
                                   }, MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,},
                                   MapMarkPoint{chestFlag: 0, x: 0, y: 0,}],};
          init
      },
      MapMarkIconData{markType: 0,
                      count: 0,
                      points:
                          [MapMarkPoint{chestFlag: 0, x: 0, y: 0,}; 12],}]];
#[no_mangle]
pub static mut gMapMarkDataTable: [*mut MapMarkData; 10] =
    unsafe {
        [sMapMarkDekuTree.as_ptr() as *mut _,
         sMapMarkDodongosCavern.as_ptr() as *mut _,
         sMapMarkJabuJabuBelly.as_ptr() as *mut _,
         sMapMarkForestTemple.as_ptr() as *mut _,
         sMapMarkFireTemple.as_ptr() as *mut _,
         sMapMarkWaterTemple.as_ptr() as *mut _,
         sMapMarkSpiritTemple.as_ptr() as *mut _,
         sMapMarkShadowTemple.as_ptr() as *mut _,
         sMapMarkBottomWell.as_ptr() as *mut _,
         sMapMarkIceCavern.as_ptr() as *mut _]
    };
