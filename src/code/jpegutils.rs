#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegQuantizationTable {
    pub table: [u16_0; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegHuffmanTable {
    pub codeOffs: [u8_0; 16],
    pub codesA: [u16_0; 16],
    pub codesB: [u16_0; 16],
    pub symbols: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegHuffmanTableOld {
    pub codeOffs: [u8_0; 16],
    pub dcCodes: [u16_0; 120],
    pub acCodes: [u16_0; 256],
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_ProcessQuantizationTable(mut dqt:
                                                                *mut u8_0,
                                                            mut qt:
                                                                *mut JpegQuantizationTable,
                                                            mut count: u8_0) {
    let mut i: u8_0 = 0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < count as libc::c_int {
        let mut j: u8_0 = 0;
        dqt = dqt.offset(1);
        j = 0 as libc::c_int as u8_0;
        while (j as libc::c_int) < 64 as libc::c_int {
            let fresh0 = dqt;
            dqt = dqt.offset(1);
            (*qt.offset(i as isize)).table[j as usize] = *fresh0 as u16_0;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_ParseHuffmanCodesLengths(mut ptr:
                                                                *mut u8_0,
                                                            mut codesLengths:
                                                                *mut u8_0)
 -> s32 {
    let mut off: u8_0 = 1 as libc::c_int as u8_0;
    let mut count: s16 = 0 as libc::c_int as s16;
    let mut idx: s16 = 1 as libc::c_int as s16;
    while (off as libc::c_int) < 0x11 as libc::c_int {
        while idx as libc::c_int <=
                  *ptr.offset((off as libc::c_int - 1 as libc::c_int) as
                                  isize) as libc::c_int {
            let fresh1 = count;
            count = count + 1;
            *codesLengths.offset(fresh1 as isize) = off;
            idx += 1
        }
        idx = 1 as libc::c_int as s16;
        off = off.wrapping_add(1)
    }
    *codesLengths.offset(count as isize) = 0 as libc::c_int as u8_0;
    return count as s32;
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_GetHuffmanCodes(mut codesLengths:
                                                       *mut u8_0,
                                                   mut codes: *mut u16_0)
 -> s32 {
    let mut idx: s16 = 0 as libc::c_int as s16;
    let mut code: u16_0 = 0 as libc::c_int as u16_0;
    let mut lastLen: u8_0 = *codesLengths.offset(0 as libc::c_int as isize);
    loop  {
        loop  {
            let fresh2 = code;
            code = code.wrapping_add(1);
            let fresh3 = idx;
            idx = idx + 1;
            *codes.offset(fresh3 as isize) = fresh2;
            if *codesLengths.offset(idx as isize) as libc::c_int !=
                   lastLen as libc::c_int {
                break ;
            }
        }
        if *codesLengths.offset(idx as isize) as libc::c_int ==
               0 as libc::c_int {
            break ;
        }
        loop  {
            code = ((code as libc::c_int) << 1 as libc::c_int) as u16_0;
            lastLen = lastLen.wrapping_add(1);
            if *codesLengths.offset(idx as isize) as libc::c_int ==
                   lastLen as libc::c_int {
                break ;
            }
        }
    }
    return idx as s32;
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_SetHuffmanTable(mut data: *mut u8_0,
                                                   mut ht:
                                                       *mut JpegHuffmanTable,
                                                   mut codes: *mut u16_0)
 -> s32 {
    let mut idx: u8_0 = 0;
    let mut codeOff: u16_0 = 0 as libc::c_int as u16_0;
    idx = 0 as libc::c_int as u8_0;
    while (idx as libc::c_int) < 0x10 as libc::c_int {
        if *data.offset(idx as isize) != 0 {
            (*ht).codeOffs[idx as usize] = codeOff as u8_0;
            (*ht).codesA[idx as usize] = *codes.offset(codeOff as isize);
            codeOff =
                (codeOff as libc::c_int +
                     (*data.offset(idx as isize) as libc::c_int -
                          1 as libc::c_int)) as u16_0;
            (*ht).codesB[idx as usize] = *codes.offset(codeOff as isize);
            codeOff = codeOff.wrapping_add(1)
        } else { (*ht).codesB[idx as usize] = 0xffff as libc::c_int as u16_0 }
        idx = idx.wrapping_add(1)
    }
    return codeOff as s32;
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_ProcessHuffmanTableImpl(mut data:
                                                               *mut u8_0,
                                                           mut ht:
                                                               *mut JpegHuffmanTable,
                                                           mut codesLengths:
                                                               *mut u8_0,
                                                           mut codes:
                                                               *mut u16_0,
                                                           mut isAc: u8_0)
 -> u32_0 {
    let mut ret: s16 = 0;
    let mut count: s32 =
        JpegUtils_ParseHuffmanCodesLengths(data, codesLengths);
    let mut temp: s32 = 0;
    ret = count as s16;
    if count == 0 as libc::c_int ||
           isAc as libc::c_int != 0 && count > 0x100 as libc::c_int ||
           isAc == 0 && count > 0x10 as libc::c_int {
        return 0 as libc::c_int as u32_0
    }
    if ret as libc::c_int != JpegUtils_GetHuffmanCodes(codesLengths, codes) {
        return 0 as libc::c_int as u32_0
    }
    temp = JpegUtils_SetHuffmanTable(data, ht, codes);
    if temp != ret as libc::c_int { return 0 as libc::c_int as u32_0 }
    return ret as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_ProcessHuffmanTable(mut dht: *mut u8_0,
                                                       mut ht:
                                                           *mut JpegHuffmanTable,
                                                       mut codesLengths:
                                                           *mut u8_0,
                                                       mut codes: *mut u16_0,
                                                       mut count: u8_0)
 -> u32_0 {
    let mut idx: u8_0 = 0;
    let mut codeCount: u32_0 = 0;
    idx = 0 as libc::c_int as u8_0;
    while (idx as libc::c_int) < count as libc::c_int {
        let fresh4 = dht;
        dht = dht.offset(1);
        let mut ac: u32_0 =
            (*fresh4 as libc::c_int >> 4 as libc::c_int) as u32_0;
        codeCount =
            JpegUtils_ProcessHuffmanTableImpl(dht,
                                              &mut *ht.offset(idx as isize),
                                              codesLengths, codes,
                                              ac as u8_0);
        if codeCount == 0 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int as u32_0
        }
        dht = dht.offset(0x10 as libc::c_int as isize);
        let ref mut fresh5 = (*ht.offset(idx as isize)).symbols;
        *fresh5 = dht;
        dht = dht.offset(codeCount as isize);
        idx = idx.wrapping_add(1)
    }
    return 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_SetHuffmanTableOld(mut data: *mut u8_0,
                                                      mut ht:
                                                          *mut JpegHuffmanTableOld,
                                                      mut codesLengths:
                                                          *mut u8_0,
                                                      mut codes: *mut u16_0,
                                                      mut count: s16,
                                                      mut isAc: u8_0) {
    let mut idx: s16 = 0;
    let mut a: u8_0 = 0;
    idx = 0 as libc::c_int as s16;
    while (idx as libc::c_int) < count as libc::c_int {
        a = *data.offset(idx as isize);
        if isAc != 0 {
            (*ht).acCodes[a as usize] = *codes.offset(idx as isize);
            (*ht).codeOffs[a as usize] = *codesLengths.offset(idx as isize)
        } else {
            (*ht).dcCodes[a as usize] = *codes.offset(idx as isize);
            (*ht).codeOffs[a as usize] = *codesLengths.offset(idx as isize)
        }
        idx += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn JpegUtils_ProcessHuffmanTableImplOld(mut dht:
                                                                  *mut u8_0,
                                                              mut ht:
                                                                  *mut JpegHuffmanTableOld,
                                                              mut codesLengths:
                                                                  *mut u8_0,
                                                              mut codes:
                                                                  *mut u16_0)
 -> u32_0 {
    let fresh6 = dht;
    dht = dht.offset(1);
    let mut isAc: u8_0 = (*fresh6 as libc::c_int >> 4 as libc::c_int) as u8_0;
    let mut count2: s16 = 0;
    let mut count: s32 = 0;
    count = JpegUtils_ParseHuffmanCodesLengths(dht, codesLengths);
    count2 = count as s16;
    if count == 0 as libc::c_int ||
           isAc as libc::c_int != 0 && count > 0x100 as libc::c_int ||
           isAc == 0 && count > 0x10 as libc::c_int {
        return 1 as libc::c_int as u32_0
    }
    if JpegUtils_GetHuffmanCodes(codesLengths, codes) != count2 as libc::c_int
       {
        return 1 as libc::c_int as u32_0
    }
    JpegUtils_SetHuffmanTableOld(dht.offset(0x10 as libc::c_int as isize), ht,
                                 codesLengths, codes, count2, isAc);
    return 0 as libc::c_int as u32_0;
}
