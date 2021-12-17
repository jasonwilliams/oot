#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn DmaMgr_DmaRomToRam(rom: u32_0, ram: u32_0, size: u32_0) -> s32;
    #[no_mangle]
    fn bcopy(__src: *mut libc::c_void, __dest: *mut libc::c_void, __n: u32_0)
     -> *mut libc::c_void;
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Yaz0Header {
    pub magic: [libc::c_char; 4],
    pub decSize: u32_0,
    pub compInfoOffset: u32_0,
    pub uncompDataOffset: u32_0,
    pub data: [u32_0; 1],
}
#[no_mangle]
pub static mut sYaz0DataBuffer: [u8_0; 1024] = [0; 1024];
#[no_mangle]
pub static mut sYaz0CurDataEnd: u32_0 = 0;
#[no_mangle]
pub static mut sYaz0CurRomStart: u32_0 = 0;
#[no_mangle]
pub static mut sYaz0CurSize: u32_0 = 0;
#[no_mangle]
pub static mut sYaz0MaxPtr: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn Yaz0_FirstDMA() -> *mut libc::c_void {
    let mut pad0: u32_0 = 0; // compressed
    let mut pad1: u32_0 = 0;
    let mut dmaSize: u32_0 = 0;
    let mut curSize: u32_0 = 0;
    sYaz0MaxPtr =
        sYaz0CurDataEnd.wrapping_sub(0x19 as libc::c_int as libc::c_uint);
    curSize =
        sYaz0CurDataEnd.wrapping_sub(sYaz0DataBuffer.as_mut_ptr() as u32_0);
    dmaSize = if curSize > sYaz0CurSize { sYaz0CurSize } else { curSize };
    DmaMgr_DmaRomToRam(sYaz0CurRomStart,
                       sYaz0DataBuffer.as_mut_ptr() as u32_0, dmaSize);
    sYaz0CurRomStart =
        (sYaz0CurRomStart as libc::c_uint).wrapping_add(dmaSize) as u32_0 as
            u32_0;
    sYaz0CurSize =
        (sYaz0CurSize as libc::c_uint).wrapping_sub(dmaSize) as u32_0 as
            u32_0;
    return sYaz0DataBuffer.as_mut_ptr() as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Yaz0_NextDMA(mut curSrcPos: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut dst: *mut u8_0 = 0 as *mut u8_0;
    let mut restSize: u32_0 = 0;
    let mut dmaSize: u32_0 = 0;
    restSize = sYaz0CurDataEnd.wrapping_sub(curSrcPos as u32_0);
    dst =
        if restSize & 7 as libc::c_int as libc::c_uint != 0 {
            sYaz0DataBuffer.as_mut_ptr().offset(-((restSize &
                                                       7 as libc::c_int as
                                                           libc::c_uint) as
                                                      isize)).offset(8 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
        } else { sYaz0DataBuffer.as_mut_ptr() };
    bcopy(curSrcPos, dst as *mut libc::c_void, restSize);
    dmaSize =
        sYaz0CurDataEnd.wrapping_sub(dst as u32_0).wrapping_sub(restSize);
    if sYaz0CurSize < dmaSize { dmaSize = sYaz0CurSize }
    if dmaSize != 0 as libc::c_int as libc::c_uint {
        DmaMgr_DmaRomToRam(sYaz0CurRomStart,
                           (dst as u32_0).wrapping_add(restSize), dmaSize);
        sYaz0CurRomStart =
            (sYaz0CurRomStart as libc::c_uint).wrapping_add(dmaSize) as u32_0
                as u32_0;
        sYaz0CurSize =
            (sYaz0CurSize as libc::c_uint).wrapping_sub(dmaSize) as u32_0 as
                u32_0;
        if sYaz0CurSize == 0 {
            sYaz0MaxPtr =
                (dst as u32_0).wrapping_add(restSize).wrapping_add(dmaSize)
        }
    }
    return dst as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Yaz0_DecompressImpl(mut hdr: *mut Yaz0Header,
                                             mut dst: *mut u8_0) {
    let mut bitIdx: u32_0 = 0 as libc::c_int as u32_0;
    let mut src: *mut u8_0 = (*hdr).data.as_mut_ptr() as *mut u8_0;
    let mut dstEnd: *mut u8_0 = dst.offset((*hdr).decSize as isize);
    let mut chunkHeader: u32_0 = 0;
    let mut nibble: u32_0 = 0;
    let mut backPtr: *mut u8_0 = 0 as *mut u8_0;
    let mut chunkSize: u32_0 = 0;
    let mut off: u32_0 = 0;
    loop  {
        if bitIdx == 0 as libc::c_int as libc::c_uint {
            if sYaz0MaxPtr < src as u32_0 &&
                   sYaz0CurSize != 0 as libc::c_int as libc::c_uint {
                src = Yaz0_NextDMA(src as *mut libc::c_void) as *mut u8_0
            }
            let fresh0 = src;
            src = src.offset(1);
            chunkHeader = *fresh0 as u32_0;
            bitIdx = 8 as libc::c_int as u32_0
        }
        if chunkHeader &
               ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint != 0 {
            // uncompressed
            *dst = *src; // 2 bytes NB BB
            dst = dst.offset(1);
            src = src.offset(1)
        } else {
            off =
                ((*src as libc::c_int & 0xf as libc::c_int) <<
                     8 as libc::c_int |
                     *src.offset(1 as libc::c_int as isize) as libc::c_int) as
                    u32_0;
            nibble = (*src as libc::c_int >> 4 as libc::c_int) as u32_0;
            backPtr = dst.offset(-(off as isize));
            src = src.offset(2 as libc::c_int as isize);
            chunkSize =
                if nibble == 0 as libc::c_int as libc::c_uint {
                    let fresh1 = src;
                    src = src.offset(1);
                    (*fresh1 as libc::c_int + 0x12 as libc::c_int) as u32_0
                } else {
                    nibble.wrapping_add(2 as libc::c_int as libc::c_uint)
                };
            loop  {
                let fresh2 = backPtr;
                backPtr = backPtr.offset(1);
                let fresh3 = dst;
                dst = dst.offset(1);
                *fresh3 = *fresh2.offset(-(1 as libc::c_int as isize));
                chunkSize = chunkSize.wrapping_sub(1);
                if !(chunkSize != 0 as libc::c_int as libc::c_uint) {
                    break ;
                }
            }
        }
        chunkHeader <<= 1 as libc::c_int;
        bitIdx = bitIdx.wrapping_sub(1);
        if !(dst != dstEnd) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Yaz0_Decompress(mut romStart: u32_0,
                                         mut dst: *mut libc::c_void,
                                         mut size: u32_0) {
    sYaz0CurRomStart = romStart;
    sYaz0CurSize = size;
    sYaz0CurDataEnd =
        sYaz0DataBuffer.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1024]>()
                                                as libc::c_ulong as isize) as
            u32_0;
    Yaz0_DecompressImpl(Yaz0_FirstDMA() as *mut Yaz0Header, dst as *mut u8_0);
}
