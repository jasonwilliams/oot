#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut gOverlayLogSeverity: s32;
}
pub type s16 = libc::c_short;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OverlayRelocationSection {
    pub textSize: u32_0,
    pub dataSize: u32_0,
    pub rodataSize: u32_0,
    pub bssSize: u32_0,
    pub nRelocations: u32_0,
    pub relocations: [u32_0; 1],
}
#[no_mangle]
pub unsafe extern "C" fn Overlay_Relocate(mut allocatedVRamAddress:
                                              *mut libc::c_void,
                                          mut overlayInfo:
                                              *mut OverlayRelocationSection,
                                          mut vRamAddress:
                                              *mut libc::c_void) {
    let mut sections: [u32_0; 4] = [0; 4];
    let mut relocatedValue: u32_0 = 0;
    let mut dbg: u32_0 = 0;
    let mut relocOffset: u32_0 = 0;
    let mut relocData: u32_0 = 0;
    let mut unrelocatedAddress: u32_0 = 0;
    let mut i: u32_0 = 0;
    let mut relocDataP: *mut u32_0 = 0 as *mut u32_0;
    let mut luiRefs: [*mut u32_0; 32] = [0 as *mut u32_0; 32];
    let mut luiVals: [u32_0; 32] = [0; 32];
    let mut relocatedAddress: u32_0 = 0;
    let mut reloc: u32_0 = 0;
    let mut vaddr: u32_0 = 0;
    let mut luiInstRef: *mut u32_0 = 0 as *mut u32_0;
    let mut allocu32: u32_0 = allocatedVRamAddress as u32_0;
    let mut regValP: *mut u32_0 = 0 as *mut u32_0;
    let mut isLoNeg: u32_0 = 0;
    relocOffset = 0 as libc::c_int as u32_0;
    relocatedValue = 0 as libc::c_int as u32_0;
    unrelocatedAddress = 0 as libc::c_int as u32_0;
    relocatedAddress = 0 as libc::c_int as u32_0;
    if gOverlayLogSeverity >= 3 as libc::c_int {
        osSyncPrintf(b"DoRelocation(%08x, %08x, %08x)\n\x00" as *const u8 as
                         *const libc::c_char, allocatedVRamAddress,
                     overlayInfo, vRamAddress);
        osSyncPrintf(b"text=%08x, data=%08x, rodata=%08x, bss=%08x\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*overlayInfo).textSize, (*overlayInfo).dataSize,
                     (*overlayInfo).rodataSize, (*overlayInfo).bssSize);
    }
    sections[0 as libc::c_int as usize] = 0 as libc::c_int as u32_0;
    sections[1 as libc::c_int as usize] = allocu32;
    sections[2 as libc::c_int as usize] =
        allocu32.wrapping_add((*overlayInfo).textSize);
    sections[3 as libc::c_int as usize] =
        sections[2 as libc::c_int as
                     usize].wrapping_add((*overlayInfo).dataSize);
    i = 0 as libc::c_int as u32_0;
    while i < (*overlayInfo).nRelocations {
        reloc = *(*overlayInfo).relocations.as_mut_ptr().offset(i as isize);
        relocDataP =
            sections[(reloc >> 0x1e as libc::c_int) as
                         usize].wrapping_add(reloc &
                                                 0xffffff as libc::c_int as
                                                     libc::c_uint) as
                *mut u32_0;
        relocData = *relocDataP;
        match reloc & 0x3f000000 as libc::c_int as libc::c_uint {
            33554432 => {
                /* R_MIPS_32
                 * Handles 32-bit address relocation.  Used in things such as
                 * jump tables.
                 */
                if *relocDataP & 0xf000000 as libc::c_int as libc::c_uint ==
                       0 as libc::c_int as libc::c_uint {
                    luiInstRef = vRamAddress as *mut u32_0;
                    relocOffset =
                        (*relocDataP).wrapping_sub(luiInstRef as u32_0);
                    relocatedValue = relocOffset.wrapping_add(allocu32);
                    relocatedAddress = relocatedValue;
                    unrelocatedAddress = relocData;
                    *relocDataP = relocatedAddress
                }
            }
            67108864 => {
                /* R_MIPS_26
                 * Handles 26-bit address relocation, used for jumps and jals
                 */
                unrelocatedAddress =
                    (*relocDataP & 0x3ffffff as libc::c_int as libc::c_uint)
                        << 2 as libc::c_int | 0x80000000 as libc::c_uint;
                relocOffset =
                    unrelocatedAddress.wrapping_sub(vRamAddress as u32_0);
                relocatedValue =
                    *relocDataP & 0xfc000000 as libc::c_uint |
                        (allocu32.wrapping_add(relocOffset) &
                             0xfffffff as libc::c_int as libc::c_uint) >>
                            2 as libc::c_int;
                relocatedAddress =
                    (relocatedValue &
                         0x3ffffff as libc::c_int as libc::c_uint) <<
                        2 as libc::c_int | 0x80000000 as libc::c_uint;
                *relocDataP = relocatedValue
            }
            83886080 => {
                /* R_MIPS_HI16
                 * Handles relocation for a lui instruciton, store the reference to
                 * the instruction, and will update it in the R_MIPS_LO16 section.
                 */
                luiRefs[(*relocDataP >> 0x10 as libc::c_int &
                             0x1f as libc::c_int as libc::c_uint) as usize] =
                    relocDataP;
                luiVals[(*relocDataP >> 0x10 as libc::c_int &
                             0x1f as libc::c_int as libc::c_uint) as usize] =
                    *relocDataP
            }
            100663296 => {
                /* R_MIPS_LO16
                 * Updates the LUI instruction to reflect the relocated address.
                 * The full address is calculated from the LUI and lo parts, and then updated.
                 * if the lo part is negative, add 1 to the lui.
                 */
                regValP =
                    &mut *luiVals.as_mut_ptr().offset((*relocDataP >>
                                                           0x15 as libc::c_int
                                                           &
                                                           0x1f as libc::c_int
                                                               as
                                                               libc::c_uint)
                                                          as isize) as
                        *mut u32_0;
                vaddr =
                    (*regValP <<
                         0x10 as
                             libc::c_int).wrapping_add(*relocDataP as s16 as
                                                           libc::c_uint);
                luiInstRef =
                    luiRefs[(*relocDataP >> 0x15 as libc::c_int &
                                 0x1f as libc::c_int as libc::c_uint) as
                                usize];
                if vaddr & 0xf000000 as libc::c_int as libc::c_uint ==
                       0 as libc::c_int as libc::c_uint {
                    relocOffset = vaddr.wrapping_sub(vRamAddress as u32_0);
                    vaddr = relocData as s16 as u32_0;
                    isLoNeg =
                        if relocOffset.wrapping_add(allocu32) &
                               0x8000 as libc::c_int as libc::c_uint != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int } as u32_0;
                    unrelocatedAddress =
                        (*luiInstRef <<
                             0x10 as libc::c_int).wrapping_add(vaddr);
                    *luiInstRef =
                        *luiInstRef & 0xffff0000 as libc::c_uint |
                            (relocOffset.wrapping_add(allocu32) >>
                                 0x10 as libc::c_int &
                                 0xffff as libc::c_int as
                                     libc::c_uint).wrapping_add(isLoNeg);
                    relocatedValue =
                        *relocDataP & 0xffff0000 as libc::c_uint |
                            relocOffset.wrapping_add(allocu32) &
                                0xffff as libc::c_int as libc::c_uint;
                    relocatedAddress =
                        (*luiInstRef <<
                             0x10 as
                                 libc::c_int).wrapping_add(relocatedValue as
                                                               s16 as
                                                               libc::c_uint);
                    *relocDataP = relocatedValue
                }
            }
            _ => { }
        }
        dbg = 0x10 as libc::c_int as u32_0;
        let mut current_block_51: u64;
        match reloc & 0x3f000000 as libc::c_int as libc::c_uint {
            33554432 => {
                dbg = 0x16 as libc::c_int as u32_0;
                current_block_51 = 18221575213305925356;
            }
            67108864 => { current_block_51 = 18221575213305925356; }
            100663296 => { current_block_51 = 5794969972171895831; }
            _ => { current_block_51 = 7252614138838059896; }
        }
        match current_block_51 {
            18221575213305925356 => {
                dbg =
                    (dbg as
                         libc::c_uint).wrapping_add(0xa as libc::c_int as
                                                        libc::c_uint) as u32_0
                        as u32_0;
                current_block_51 = 5794969972171895831;
            }
            _ => { }
        }
        match current_block_51 {
            5794969972171895831 => {
                if gOverlayLogSeverity >= 3 as libc::c_int {
                    osSyncPrintf(b"%02d %08x %08x %08x \x00" as *const u8 as
                                     *const libc::c_char, dbg, relocDataP,
                                 relocatedValue, relocatedAddress);
                    osSyncPrintf(b" %08x %08x %08x %08x\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 (relocDataP as
                                      u32_0).wrapping_add(vRamAddress as
                                                              u32_0).wrapping_sub(allocu32),
                                 relocData, unrelocatedAddress, relocOffset);
                }
            }
            _ => { }
        }
        i = i.wrapping_add(1)
    };
}
