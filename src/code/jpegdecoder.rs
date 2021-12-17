#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
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
pub struct JpegDecoder {
    pub imageData: *mut libc::c_void,
    pub mode: u8_0,
    pub unk_05: u8_0,
    pub hTablePtrs: [*mut JpegHuffmanTable; 4],
    pub unk_18: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JpegDecoderState {
    pub byteIdx: u32_0,
    pub bitIdx: u8_0,
    pub dontSkip: u8_0,
    pub curWord: u32_0,
    pub unk_0C: s16,
    pub unk_0E: s16,
    pub unk_10: s16,
}
#[no_mangle]
pub static mut sJpegBitStreamPtr: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
#[no_mangle]
pub static mut sJpegBitStreamByteIdx: u32_0 = 0;
#[no_mangle]
pub static mut sJpegBitStreamBitIdx: u8_0 = 0;
#[no_mangle]
pub static mut sJpegBitStreamDontSkip: u8_0 = 0;
#[no_mangle]
pub static mut sJpegBitStreamCurWord: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn JpegDecoder_Decode(mut decoder: *mut JpegDecoder,
                                            mut mcuBuff: *mut u16_0,
                                            mut count: s32,
                                            mut isFollowing: u8_0,
                                            mut state: *mut JpegDecoderState)
 -> s32 {
    let mut pad: s16 = 0; // this is required for some reason
    let mut unk0: s16 = 0;
    let mut unk1: s16 = 0;
    let mut unk2: s16 = 0;
    let mut idx: u32_0 = 0;
    let mut inc: s32 = 0;
    let mut unkCount: u16_0 = 0;
    let mut hTable0: *mut JpegHuffmanTable = 0 as *mut JpegHuffmanTable;
    let mut hTable1: *mut JpegHuffmanTable = 0 as *mut JpegHuffmanTable;
    let mut hTable2: *mut JpegHuffmanTable = 0 as *mut JpegHuffmanTable;
    let mut hTable3: *mut JpegHuffmanTable = 0 as *mut JpegHuffmanTable;
    inc = 0 as libc::c_int;
    sJpegBitStreamPtr = (*decoder).imageData as *mut u8_0;
    if (*decoder).mode as libc::c_int == 0 as libc::c_int {
        unkCount = 2 as libc::c_int as u16_0
    } else {
        unkCount = 4 as libc::c_int as u16_0;
        if (*decoder).unk_05 as libc::c_int == 1 as libc::c_int {
            inc = 8 as libc::c_int * 8 as libc::c_int * 2 as libc::c_int
        }
    }
    hTable0 = (*decoder).hTablePtrs[0 as libc::c_int as usize];
    hTable1 = (*decoder).hTablePtrs[1 as libc::c_int as usize];
    hTable2 = (*decoder).hTablePtrs[2 as libc::c_int as usize];
    hTable3 = (*decoder).hTablePtrs[3 as libc::c_int as usize];
    if isFollowing == 0 {
        sJpegBitStreamByteIdx = 0 as libc::c_int as u32_0;
        sJpegBitStreamBitIdx = 32 as libc::c_int as u8_0;
        sJpegBitStreamCurWord = 0 as libc::c_int as u32_0;
        sJpegBitStreamDontSkip = 0 as libc::c_int as u8_0;
        unk0 = 0 as libc::c_int as s16;
        unk1 = 0 as libc::c_int as s16;
        unk2 = 0 as libc::c_int as s16
    } else {
        sJpegBitStreamByteIdx = (*state).byteIdx;
        sJpegBitStreamBitIdx = (*state).bitIdx;
        sJpegBitStreamCurWord = (*state).curWord;
        sJpegBitStreamDontSkip = (*state).dontSkip;
        unk0 = (*state).unk_0C;
        unk1 = (*state).unk_0E;
        unk2 = (*state).unk_10
    }
    while count != 0 as libc::c_int {
        idx = 0 as libc::c_int as u32_0;
        while idx < unkCount as libc::c_uint {
            if JpegDecoder_ProcessMcu(hTable0, hTable1, mcuBuff, &mut unk0) !=
                   0 {
                return 2 as libc::c_int
            }
            mcuBuff =
                mcuBuff.offset((8 as libc::c_int * 8 as libc::c_int) as
                                   isize);
            idx = idx.wrapping_add(1)
        }
        if JpegDecoder_ProcessMcu(hTable2, hTable3, mcuBuff, &mut unk1) != 0 {
            return 2 as libc::c_int
        }
        mcuBuff =
            mcuBuff.offset((8 as libc::c_int * 8 as libc::c_int) as isize);
        if JpegDecoder_ProcessMcu(hTable2, hTable3, mcuBuff, &mut unk2) != 0 {
            return 2 as libc::c_int
        }
        count -= 1;
        mcuBuff =
            mcuBuff.offset((8 as libc::c_int * 8 as libc::c_int) as isize);
        mcuBuff = mcuBuff.offset(inc as isize)
    }
    (*state).byteIdx = sJpegBitStreamByteIdx;
    (*state).bitIdx = sJpegBitStreamBitIdx;
    (*state).curWord = sJpegBitStreamCurWord;
    (*state).dontSkip = sJpegBitStreamDontSkip;
    (*state).unk_0C = unk0;
    (*state).unk_0E = unk1;
    (*state).unk_10 = unk2;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn JpegDecoder_ProcessMcu(mut hTable0:
                                                    *mut JpegHuffmanTable,
                                                mut hTable1:
                                                    *mut JpegHuffmanTable,
                                                mut mcu: *mut u16_0,
                                                mut unk: *mut s16) -> s32 {
    let mut i: s8 = 0 as libc::c_int as s8;
    let mut zeroCount: s8 = 0;
    let mut coeff: s16 = 0;
    if JpegDecoder_ParseNextSymbol(hTable0, &mut coeff, &mut zeroCount) != 0 {
        return 1 as libc::c_int
    }
    *unk = (*unk as libc::c_int + coeff as libc::c_int) as s16;
    let fresh0 = i;
    i = i + 1;
    *mcu.offset(fresh0 as isize) = *unk as u16_0;
    while (i as libc::c_int) < 8 as libc::c_int * 8 as libc::c_int {
        if JpegDecoder_ParseNextSymbol(hTable1, &mut coeff, &mut zeroCount) !=
               0 as libc::c_int {
            return 1 as libc::c_int
        }
        if coeff as libc::c_int == 0 as libc::c_int {
            if zeroCount as libc::c_int == 0xf as libc::c_int {
                loop  {
                    let fresh1 = zeroCount;
                    zeroCount = zeroCount - 1;
                    if !(fresh1 as libc::c_int >= 0 as libc::c_int) {
                        break ;
                    }
                    let fresh2 = i;
                    i = i + 1;
                    *mcu.offset(fresh2 as isize) = 0 as libc::c_int as u16_0
                }
            } else {
                while (i as libc::c_int) < 8 as libc::c_int * 8 as libc::c_int
                      {
                    let fresh3 = i;
                    i = i + 1;
                    *mcu.offset(fresh3 as isize) = 0 as libc::c_int as u16_0
                }
                break ;
            }
        } else {
            loop  {
                let fresh4 = zeroCount;
                zeroCount = zeroCount - 1;
                if !((0 as libc::c_int) < fresh4 as libc::c_int) { break ; }
                let fresh5 = i;
                i = i + 1;
                *mcu.offset(fresh5 as isize) = 0 as libc::c_int as u16_0
            }
            let fresh6 = i;
            i = i + 1;
            *mcu.offset(fresh6 as isize) = coeff as u16_0
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn JpegDecoder_ParseNextSymbol(mut hTable:
                                                         *mut JpegHuffmanTable,
                                                     mut outCoeff: *mut s16,
                                                     mut outZeroCount:
                                                         *mut s8) -> s32 {
    let mut codeIdx: u8_0 = 0;
    let mut sym: u8_0 = 0;
    let mut codeOff: u16_0 = 0 as libc::c_int as u16_0;
    let mut buff: u16_0 = JpegDecoder_ReadBits(16 as libc::c_int as u8_0);
    codeIdx = 0 as libc::c_int as u8_0;
    while (codeIdx as libc::c_int) < 16 as libc::c_int {
        if !((*hTable).codesB[codeIdx as usize] as libc::c_int ==
                 0xffff as libc::c_int) {
            codeOff =
                (buff as libc::c_int >>
                     15 as libc::c_int - codeIdx as libc::c_int) as u16_0;
            if codeOff as libc::c_int <=
                   (*hTable).codesB[codeIdx as usize] as libc::c_int {
                break ;
            }
        }
        codeIdx = codeIdx.wrapping_add(1)
    }
    if codeIdx as libc::c_int >= 16 as libc::c_int { return 1 as libc::c_int }
    sym =
        *(*hTable).symbols.offset(((*hTable).codeOffs[codeIdx as usize] as
                                       libc::c_int + codeOff as libc::c_int -
                                       (*hTable).codesA[codeIdx as usize] as
                                           libc::c_int) as isize);
    *outZeroCount = (sym as libc::c_int >> 4 as libc::c_int) as s8;
    sym = (sym as libc::c_int & 0xf as libc::c_int) as u8_0;
    sJpegBitStreamBitIdx =
        (sJpegBitStreamBitIdx as libc::c_int +
             (codeIdx as libc::c_int - 15 as libc::c_int)) as u8_0;
    *outCoeff = 0 as libc::c_int as s16;
    if sym != 0 {
        *outCoeff = JpegDecoder_ReadBits(sym) as s16;
        if (*outCoeff as libc::c_int) <
               (1 as libc::c_int) << sym as libc::c_int - 1 as libc::c_int {
            *outCoeff =
                (*outCoeff as libc::c_int +
                     (((-(1 as libc::c_int)) << sym as libc::c_int) +
                          1 as libc::c_int)) as s16
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn JpegDecoder_ReadBits(mut len: u8_0) -> u16_0 {
    let mut byteCount: u8_0 = 0;
    let mut data: u8_0 = 0;
    let mut ret: s32 = 0;
    let mut temp: u32_0 = 0;
    ret = 0 as libc::c_int;
    byteCount =
        (sJpegBitStreamBitIdx as libc::c_int >> 3 as libc::c_int) as u8_0;
    while byteCount as libc::c_int > 0 as libc::c_int {
        let fresh7 = sJpegBitStreamByteIdx;
        sJpegBitStreamByteIdx = sJpegBitStreamByteIdx.wrapping_add(1);
        data = *sJpegBitStreamPtr.offset(fresh7 as isize);
        if sJpegBitStreamDontSkip != 0 {
            if data as libc::c_int == 0 as libc::c_int {
                let fresh8 = sJpegBitStreamByteIdx;
                sJpegBitStreamByteIdx = sJpegBitStreamByteIdx.wrapping_add(1);
                data = *sJpegBitStreamPtr.offset(fresh8 as isize)
            }
        }
        sJpegBitStreamDontSkip =
            if data as libc::c_int == 0xff as libc::c_int {
                1 as libc::c_int
            } else { 0 as libc::c_int } as u8_0;
        sJpegBitStreamCurWord <<= 8 as libc::c_int;
        sJpegBitStreamCurWord |= data as libc::c_uint;
        sJpegBitStreamBitIdx =
            (sJpegBitStreamBitIdx as libc::c_int - 8 as libc::c_int) as u8_0;
        byteCount = byteCount.wrapping_sub(1)
    }
    ret =
        (sJpegBitStreamCurWord << sJpegBitStreamBitIdx as libc::c_int) as s32;
    temp = ret as u32_0;
    ret = (temp >> -(len as libc::c_int)) as s32;
    sJpegBitStreamBitIdx =
        (sJpegBitStreamBitIdx as libc::c_int + len as libc::c_int) as u8_0;
    return ret as u16_0;
}
