#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    static mut _gameplay_keepSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _gameplay_keepSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _gameplay_field_keepSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _gameplay_field_keepSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _gameplay_dangeon_keepSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _gameplay_dangeon_keepSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_humanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_humanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_okutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_okutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_crowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_crowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_pohSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_pohSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dy_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dy_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_wallmasterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_wallmasterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dodongoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dodongoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fireflySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fireflySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_boxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_boxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bubbleSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bubbleSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_niwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_niwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_link_boySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_link_boySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_link_childSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_link_childSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_titeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_titeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_reebaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_reebaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_peehatSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_peehatSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_kingdodongoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_kingdodongoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_horseSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_horseSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zfSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_zfSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gomaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gomaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_golSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_golSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dodojrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dodojrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_torch2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_torch2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_blSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_blSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_tpSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_tpSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_stSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_stSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_eiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_eiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_normalSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_normalSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_o_animeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_o_animeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot04_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot04_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ddan_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ddan_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_hidan_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_hidan_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_ganonSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_ganonSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot00_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot00_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bombfSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bombfSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_sk2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_sk2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE_animeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE_animeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ydan_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ydan_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gndSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gndSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_amSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_amSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dekubabaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dekubabaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA5SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA5SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA6SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA6SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA7SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA7SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_jjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_jjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA8SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA8SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA9SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA9SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oB4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_zeldaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_zeldaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_opening_demo1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_opening_demo1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_warp1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_warp1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_b_heartSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_b_heartSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dekunutsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dekunutsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_menkuri_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_menkuri_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE5SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE5SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE6SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE6SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE7SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE7SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE8SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE8SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE9SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE9SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE10SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE10SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE11SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE11SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE12SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE12SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_valiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_valiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA10SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA10SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA11SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oA11SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mizu_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mizu_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fhgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fhgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ossanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ossanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_BbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_BbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_toki_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_toki_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_yukabyunSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_yukabyunSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjinSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjinSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_flashSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_flashSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_darkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_darkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_flameSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_flameSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_iceSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_iceSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_soulSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_soulSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_windSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_windSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_okaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mjin_okaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_haka_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_haka_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot06_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot06_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ice_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ice_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_relay_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_relay_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_po_fieldSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_po_fieldSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_po_composerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_po_composerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri1aSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri1aSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri2aSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_hineri2aSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_texSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mori_texSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot08_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot08_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_warp2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_warp2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_hataSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_hataSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_birdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_birdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_wood02SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_wood02SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_lightboxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_lightboxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_pu_boxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_pu_boxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_trapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_trapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_vaseSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_vaseSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_imSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_imSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_taSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_taSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_tkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_tkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_xcSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_xcSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_vmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_vmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bvSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bvSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_hakach_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_hakach_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_crystal_lightSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_crystal_lightSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_fire_ballSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_fire_ballSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_flashSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_flashSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_lgt_showerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_lgt_showerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_star_fieldSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_star_fieldSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_god_lgtSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_god_lgtSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_light_ringSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    fn DmaMgr_SendRequest2(req: *mut DmaRequest, ram: u32_0, vrom: u32_0,
                           size: u32_0, unk5: u32_0, queue: *mut OSMesgQueue,
                           msg: OSMesg, file: *const libc::c_char, line: s32)
     -> s32;
    #[no_mangle]
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn func_80031A28(globalCtx: *mut GlobalContext,
                     actorCtx: *mut ActorContext);
    #[no_mangle]
    fn BgCheck_Allocate(colCtx: *mut CollisionContext,
                        globalCtx: *mut GlobalContext,
                        colHeader: *mut CollisionHeader);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn LightContext_InsertLight(globalCtx: *mut GlobalContext,
                                lightCtx: *mut LightContext,
                                info: *mut LightInfo) -> *mut LightNode;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut _object_zl4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_timeblockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_timeblockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ouke_hakaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ouke_hakaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_door_killerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_door_killerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_sword_1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_sword_1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_cobSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_cobSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_cowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_cowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bwallSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bwallSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_psSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_psSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_haka_doorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_haka_doorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_geffSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_geffSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_skbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_skbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_wfSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_wfSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_muSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_muSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot01_matoyabSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot01_matoyabSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot01_matoyaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot01_matoyaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_rupySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_rupySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_anime3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_anime3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_anime2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_anime2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_anime1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_anime1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_dekupouchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_dekupouchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_doughnutSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_doughnutSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_demo_kekkaiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_demo_kekkaiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bowlSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bowlSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_soulSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_soulSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ghostSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ghostSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_butterflySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_butterflySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_insectSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_insectSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_fireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_fireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dnkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dnkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dnsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dnsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_kibako2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_kibako2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot11_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot11_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_jya_doorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_jya_doorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_jya_ironSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_jya_ironSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dogSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dogSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_grSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_grSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_geldbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_geldbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_shopnutsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_shopnutsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_glaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_glaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot00_breakSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot00_breakSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_rsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_rsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_hintnutsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_hintnutsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bombiwaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bombiwaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot12_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot12_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot05_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot05_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bigokutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bigokutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_sshSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_sshSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_goddessSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_goddessSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_sutaruSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_sutaruSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fishSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fishSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ecSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ecSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ds2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ds2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_m_arrowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_m_arrowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_hoverbootsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_hoverbootsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_zgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_tsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_tsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_kaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_kaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_gerudomaskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_gerudomaskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_zoramaskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_zoramaskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_golonmaskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_golonmaskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl2_anime2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl2_anime2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl2_anime1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zl2_anime1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_erupcSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_erupcSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gtSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gtSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_door_gerudoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_door_gerudoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_magSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_magSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_frogSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_frogSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_soldoutSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_soldoutSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_braceletSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_braceletSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_prescriptionSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_prescriptionSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_csSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_csSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_jsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_jsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_brokenswordSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_brokenswordSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ticketstoneSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ticketstoneSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_mushroomSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_mushroomSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_powderSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_powderSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_eye_lotionSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_eye_lotionSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_osSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_osSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_faSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_faSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_streamSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_streamSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_siofukiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_siofukiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganon_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_truth_maskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_truth_maskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_rabit_maskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_rabit_maskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_skj_maskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_skj_maskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_redead_maskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_redead_maskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ki_tan_maskSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ki_tan_maskSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_owlSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_owlSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gjyo_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gjyo_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_kanbanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_kanbanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_coinSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_coinSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_glovesSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_glovesSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_tsuboSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_tsuboSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_kusaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_kusaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_lightswitchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_lightswitchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ingateSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ingateSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_hsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_hsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_msSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_msSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_blkobjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_blkobjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_nwcSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_nwcSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_daikuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_daikuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_toryoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_toryoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_goroiwaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_goroiwaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mamenokiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mamenokiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_d_liftSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_d_liftSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_d_hsblockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_d_hsblockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_d_elevatorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_d_elevatorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gnd_magicSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gnd_magicSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_seedSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_seedSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_boots_2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_boots_2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_yabusame_pointSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_yabusame_pointSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ge1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ge1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bobSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bobSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fzSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fzSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot07_objectSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot07_objectSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot03_objectSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot03_objectSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bojSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bojSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_aneSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_aneSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ocarina_0SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ocarina_0SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bbaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bbaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bjiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bjiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bottle_letterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bottle_letterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_skjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_skjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_niwatoriSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_niwatoriSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_cneSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_cneSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ahgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ahgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ikSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ikSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_aobSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_aobSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_masterzooraSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_masterzooraSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mastergolonSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mastergolonSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_masterkokiriheadSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_masterkokiriheadSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_masterkokiriSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_masterkokiriSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_umajumpSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_umajumpSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_kzSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_kzSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_zoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_zoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_kw1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_kw1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_km1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_km1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot01_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot01_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_longswordSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_longswordSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_grassSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_grassSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_hammerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_hammerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_sawSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_sawSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_fishSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_fishSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_beanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_beanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_clothesSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_clothesSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_jya_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_jya_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot15_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot15_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_letterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_letterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_shield_3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_shield_3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_demo_6kSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_demo_6kSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_aniSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_aniSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_liquidSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_liquidSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_glassesSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_glassesSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_boomerangSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_boomerangSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_pachinkoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_pachinkoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_frSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_frSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_nySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_nySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_sstSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_sstSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganonSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ganonSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ma1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ma1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_milkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_milkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ocarinaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_ocarinaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_hookshotSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_hookshotSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_shield_2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_shield_2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_scaleSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_scaleSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_eggSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_eggSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bomb_2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bomb_2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_arrowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_arrowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_gerudoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_gerudoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_anubiceSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_anubiceSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bxaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bxaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_rrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_rrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_twSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_twSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_hniSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_hniSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_purseSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_purseSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ma2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ma2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oF1sSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oF1sSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bomb_1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bomb_1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_magicpotSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_magicpotSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dekujrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dekujrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_shield_1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_shield_1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ru2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ru2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oF1d_mapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oF1d_mapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_mapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_mapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_stickSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_stickSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bottleSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bottleSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_os_animeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_os_animeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE4sSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE4sSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE1sSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_oE1sSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot16_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot16_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_trSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_trSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_inSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_inSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bombpouchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bombpouchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_arrowcaseSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_arrowcaseSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_heartsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_heartsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_saSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_saSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_nutsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_nutsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_medalSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_medalSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bosskeySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_bosskeySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_compassSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_compassSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_heartSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_heartSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_melodySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_melodySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_sbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_sbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_moSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_moSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_nbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_nbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_shop_dungenSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_shop_dungenSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot17_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot17_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bdoorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bdoorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot18_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot18_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot09_objSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot09_objSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_jewelSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_jewelSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_brobSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_brobSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_mir_raySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_mir_raySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_keySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gi_keySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_demo_tre_lgtSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_demo_tre_lgtSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_twSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_efc_twSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_rlSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_rlSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_dhSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_dhSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fd2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fd2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_syokudaiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_syokudaiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_ru1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_ru1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_hakaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_hakaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot02_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_spot02_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_link_childSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_horse_link_childSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_medalSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_medalSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_duSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_duSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_fdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_fdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_gnddSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_gnddSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_heavy_objectSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_heavy_objectSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_po_sistersSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_po_sistersSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_rdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_rdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_sdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_sdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_bdan_objectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_bdan_objectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_triforce_spotSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _object_triforce_spotSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _object_light_ringSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    fn GameState_Alloc(gameState: *mut GameState, size: size_t,
                       file: *mut libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gBitFlags: [u32_0; 32];
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    static mut gTimeIncrement: u16_0;
    #[no_mangle]
    static mut _elf_message_ydanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _elf_message_ydanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _elf_message_fieldSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _elf_message_fieldSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    fn Gameplay_LoadFile(globalCtx: *mut GlobalContext, file: *mut RomFile)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut gActorOverlayTable: [ActorOverlay; 471];
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type Mtx_t = [[libc::c_long; 4]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub union Mtx {
    pub m: Mtx_t,
    pub c2rust_unnamed: C2RustUnnamed,
    pub forc_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub intPart: [[u16_0; 4]; 4],
    pub fracPart: [[u16_0; 4]; 4],
}
pub type MtxF_t = [[libc::c_float; 4]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub union MtxF {
    pub mf: MtxF_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub xx: libc::c_float,
    pub yx: libc::c_float,
    pub zx: libc::c_float,
    pub wx: libc::c_float,
    pub xy: libc::c_float,
    pub yy: libc::c_float,
    pub zy: libc::c_float,
    pub wy: libc::c_float,
    pub xz: libc::c_float,
    pub yz: libc::c_float,
    pub zz: libc::c_float,
    pub wz: libc::c_float,
    pub xw: libc::c_float,
    pub yw: libc::c_float,
    pub zw: libc::c_float,
    pub ww: libc::c_float,
}
pub type size_t = libc::c_ulong;
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub f_odd: f32_0,
    pub f_even: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadContext {
    pub at: u64_0,
    pub v0: u64_0,
    pub v1: u64_0,
    pub a0: u64_0,
    pub a1: u64_0,
    pub a2: u64_0,
    pub a3: u64_0,
    pub t0: u64_0,
    pub t1: u64_0,
    pub t2: u64_0,
    pub t3: u64_0,
    pub t4: u64_0,
    pub t5: u64_0,
    pub t6: u64_0,
    pub t7: u64_0,
    pub s0: u64_0,
    pub s1: u64_0,
    pub s2: u64_0,
    pub s3: u64_0,
    pub s4: u64_0,
    pub s5: u64_0,
    pub s6: u64_0,
    pub s7: u64_0,
    pub t8: u64_0,
    pub t9: u64_0,
    pub gp: u64_0,
    pub sp: u64_0,
    pub s8: u64_0,
    pub ra: u64_0,
    pub lo: u64_0,
    pub hi: u64_0,
    pub sr: u32_0,
    pub pc: u32_0,
    pub cause: u32_0,
    pub badvaddr: u32_0,
    pub rcp: u32_0,
    pub fpcsr: u32_0,
    pub fp0: __OSfp,
    pub fp2: __OSfp,
    pub fp4: __OSfp,
    pub fp6: __OSfp,
    pub fp8: __OSfp,
    pub fp10: __OSfp,
    pub fp12: __OSfp,
    pub fp14: __OSfp,
    pub fp16: __OSfp,
    pub fp18: __OSfp,
    pub fp20: __OSfp,
    pub fp22: __OSfp,
    pub fp24: __OSfp,
    pub fp26: __OSfp,
    pub fp28: __OSfp,
    pub fp30: __OSfp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadprofile {
    pub flag: u32_0,
    pub count: u32_0,
    pub time: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSThread {
    pub next: *mut OSThread,
    pub priority: OSPri,
    pub queue: *mut *mut OSThread,
    pub tlnext: *mut OSThread,
    pub state: u16_0,
    pub flags: u16_0,
    pub id: OSId,
    pub fp: s32,
    pub thprof: *mut __OSThreadprofile,
    pub context: __OSThreadContext,
}
pub type OSMesg = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSMesgQueue {
    pub mtqueue: *mut OSThread,
    pub fullqueue: *mut OSThread,
    pub validCount: s32,
    pub first: s32,
    pub msgCount: s32,
    pub msg: *mut OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSTask_t {
    pub type_0: u32_0,
    pub flags: u32_0,
    pub ucode_boot: *mut u64_0,
    pub ucode_boot_size: u32_0,
    pub ucode: *mut u64_0,
    pub ucode_size: u32_0,
    pub ucode_data: *mut u64_0,
    pub ucode_data_size: u32_0,
    pub dram_stack: *mut u64_0,
    pub dram_stack_size: u32_0,
    pub output_buff: *mut u64_0,
    pub output_buff_size: *mut u64_0,
    pub data_ptr: *mut u64_0,
    pub data_size: u32_0,
    pub yield_data_ptr: *mut u64_0,
    pub yield_data_size: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OSTask {
    pub t: OSTask_t,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViCommonRegs {
    pub ctrl: u32_0,
    pub width: u32_0,
    pub burst: u32_0,
    pub vSync: u32_0,
    pub hSync: u32_0,
    pub leap: u32_0,
    pub hStart: u32_0,
    pub xScale: u32_0,
    pub vCurrent: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViFieldRegs {
    pub origin: u32_0,
    pub yScale: u32_0,
    pub vStart: u32_0,
    pub vBurst: u32_0,
    pub vIntr: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViMode {
    pub type_0: u8_0,
    pub comRegs: OSViCommonRegs,
    pub fldRegs: [OSViFieldRegs; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSContPad {
    pub button: u16_0,
    pub stick_x: s8,
    pub stick_y: s8,
    pub errno: u8_0,
}
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
pub struct Tri {
    pub flag: libc::c_uchar,
    pub v: [libc::c_uchar; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp_t {
    pub vscale: [libc::c_short; 4],
    pub vtrans: [libc::c_short; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vp {
    pub vp: Vp_t,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Light_t {
    pub col: [libc::c_uchar; 3],
    pub pad1: libc::c_char,
    pub colc: [libc::c_uchar; 3],
    pub pad2: libc::c_char,
    pub dir: [libc::c_schar; 3],
    pub pad3: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ambient_t {
    pub col: [libc::c_uchar; 3],
    pub pad1: libc::c_char,
    pub colc: [libc::c_uchar; 3],
    pub pad2: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Light {
    pub l: Light_t,
    pub force_structure_alignment: [libc::c_longlong; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Ambient {
    pub l: Ambient_t,
    pub force_structure_alignment: [libc::c_longlong; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lightsn {
    pub a: Ambient,
    pub l: [Light; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gdma {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "par", ty = "libc::c_uint", bits = "8..=15")]
    #[bitfield(name = "len", ty = "libc::c_uint", bits = "16..=31")]
    pub cmd_par_len: [u8; 4],
    pub addr: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gtri {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad: [u8; 4],
    pub tri: Tri,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gpopmtx {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "8..=31")]
    #[bitfield(name = "pad2", ty = "libc::c_int", bits = "32..=55")]
    #[bitfield(name = "param", ty = "libc::c_uchar", bits = "56..=63")]
    pub cmd_pad1_pad2_param: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsegment {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "mw_index", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "number", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "32..=39")]
    #[bitfield(name = "base", ty = "libc::c_int", bits = "40..=63")]
    pub cmd_pad0_mw_index_number_pad1_base: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetothermodeL {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "sft", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "len", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "data", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_pad0_sft_len_data: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetothermodeH {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "sft", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "len", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "data", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_pad0_sft_len_data: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gtexture {
    pub cmd: libc::c_uchar,
    pub lodscale: libc::c_uchar,
    pub tile: libc::c_uchar,
    pub on: libc::c_uchar,
    pub s: libc::c_ushort,
    pub t: libc::c_ushort,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gline3D {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad: [u8; 4],
    pub line: Tri,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gperspnorm {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad1: [u8; 4],
    pub pad2: libc::c_short,
    pub scale: libc::c_short,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetimg {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "fmt", ty = "libc::c_uint", bits = "8..=10")]
    #[bitfield(name = "siz", ty = "libc::c_uint", bits = "11..=12")]
    #[bitfield(name = "pad", ty = "libc::c_uint", bits = "13..=19")]
    #[bitfield(name = "wd", ty = "libc::c_uint", bits = "20..=31")]
    pub cmd_fmt_siz_pad_wd: [u8; 4],
    pub dram: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetcombine {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "muxs0", ty = "libc::c_uint", bits = "8..=31")]
    #[bitfield(name = "muxs1", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_muxs0_muxs1: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetcolor {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    pub cmd: [u8; 1],
    pub pad: libc::c_uchar,
    pub prim_min_level: libc::c_uchar,
    pub prim_level: libc::c_uchar,
    pub color: libc::c_ulong,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gfillrect {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "x0", ty = "libc::c_int", bits = "8..=17")]
    #[bitfield(name = "x0frac", ty = "libc::c_int", bits = "18..=19")]
    #[bitfield(name = "y0", ty = "libc::c_int", bits = "20..=29")]
    #[bitfield(name = "y0frac", ty = "libc::c_int", bits = "30..=31")]
    #[bitfield(name = "pad", ty = "libc::c_uint", bits = "32..=39")]
    #[bitfield(name = "x1", ty = "libc::c_int", bits = "40..=49")]
    #[bitfield(name = "x1frac", ty = "libc::c_int", bits = "50..=51")]
    #[bitfield(name = "y1", ty = "libc::c_int", bits = "52..=61")]
    #[bitfield(name = "y1frac", ty = "libc::c_int", bits = "62..=63")]
    pub cmd_x0_x0frac_y0_y0frac_pad_x1_x1frac_y1_y1frac: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsettile {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "fmt", ty = "libc::c_uint", bits = "8..=10")]
    #[bitfield(name = "siz", ty = "libc::c_uint", bits = "11..=12")]
    #[bitfield(name = "pad0", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "line", ty = "libc::c_uint", bits = "14..=22")]
    #[bitfield(name = "tmem", ty = "libc::c_uint", bits = "23..=31")]
    #[bitfield(name = "pad1", ty = "libc::c_uint", bits = "32..=36")]
    #[bitfield(name = "tile", ty = "libc::c_uint", bits = "37..=39")]
    #[bitfield(name = "palette", ty = "libc::c_uint", bits = "40..=43")]
    #[bitfield(name = "ct", ty = "libc::c_uint", bits = "44..=44")]
    #[bitfield(name = "mt", ty = "libc::c_uint", bits = "45..=45")]
    #[bitfield(name = "maskt", ty = "libc::c_uint", bits = "46..=49")]
    #[bitfield(name = "shiftt", ty = "libc::c_uint", bits = "50..=53")]
    #[bitfield(name = "cs", ty = "libc::c_uint", bits = "54..=54")]
    #[bitfield(name = "ms", ty = "libc::c_uint", bits = "55..=55")]
    #[bitfield(name = "masks", ty = "libc::c_uint", bits = "56..=59")]
    #[bitfield(name = "shifts", ty = "libc::c_uint", bits = "60..=63")]
    pub cmd_fmt_siz_pad0_line_tmem_pad1_tile_palette_ct_mt_maskt_shiftt_cs_ms_masks_shifts: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gloadtile {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "sl", ty = "libc::c_uint", bits = "8..=19")]
    #[bitfield(name = "tl", ty = "libc::c_uint", bits = "20..=31")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "32..=36")]
    #[bitfield(name = "tile", ty = "libc::c_uint", bits = "37..=39")]
    #[bitfield(name = "sh", ty = "libc::c_uint", bits = "40..=51")]
    #[bitfield(name = "th", ty = "libc::c_uint", bits = "52..=63")]
    pub cmd_sl_tl_pad_tile_sh_th: [u8; 8],
}
pub type Gsettilesize = Gloadtile;
pub type Gloadtlut = Gloadtile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gwords {
    pub w0: libc::c_uint,
    pub w1: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Gfx {
    pub words: Gwords,
    pub dma: Gdma,
    pub tri: Gtri,
    pub line: Gline3D,
    pub popmtx: Gpopmtx,
    pub segment: Gsegment,
    pub setothermodeH: GsetothermodeH,
    pub setothermodeL: GsetothermodeL,
    pub texture: Gtexture,
    pub perspnorm: Gperspnorm,
    pub setimg: Gsetimg,
    pub setcombine: Gsetcombine,
    pub setcolor: Gsetcolor,
    pub fillrect: Gfillrect,
    pub settile: Gsettile,
    pub loadtile: Gloadtile,
    pub settilesize: Gsettilesize,
    pub loadtlut: Gloadtlut,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3f {
    pub x: f32_0,
    pub y: f32_0,
    pub z: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3us {
    pub x: u16_0,
    pub y: u16_0,
    pub z: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3s {
    pub x: s16,
    pub y: s16,
    pub z: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3i {
    pub x: s32,
    pub y: s32,
    pub z: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sphere16 {
    pub center: Vec3s,
    pub radius: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cylinder16 {
    pub radius: s16,
    pub height: s16,
    pub yShift: s16,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Linef {
    pub a: Vec3f,
    pub b: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ItemEquips {
    pub buttonItems: [u8_0; 4],
    pub cButtonSlots: [u8_0; 3],
    pub equipment: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Inventory {
    pub items: [u8_0; 24],
    pub ammo: [s8; 16],
    pub equipment: u16_0,
    pub upgrades: u32_0,
    pub questItems: u32_0,
    pub dungeonItems: [u8_0; 20],
    pub dungeonKeys: [s8; 19],
    pub defenseHearts: s8,
    pub gsTokens: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SavedSceneFlags {
    pub chest: u32_0,
    pub swch: u32_0,
    pub clear: u32_0,
    pub collect: u32_0,
    pub unk: u32_0,
    pub rooms: u32_0,
    pub floors: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HorseData {
    pub scene: s16,
    pub pos: Vec3s,
    pub angle: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RespawnData {
    pub pos: Vec3f,
    pub yaw: s16,
    pub playerParams: s16,
    pub entranceIndex: s16,
    pub roomIndex: u8_0,
    pub data: s8,
    pub tempSwchFlags: u32_0,
    pub tempCollectFlags: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaroresWindData {
    pub pos: Vec3i,
    pub yaw: s32,
    pub playerParams: s32,
    pub entranceIndex: s32,
    pub roomIndex: s32,
    pub set: s32,
    pub tempSwchFlags: s32,
    pub tempCollectFlags: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SaveContext {
    pub entranceIndex: s32,
    pub linkAge: s32,
    pub cutsceneIndex: s32,
    pub dayTime: u16_0,
    pub nightFlag: s32,
    pub totalDays: s32,
    pub bgsDayCount: s32,
    pub newf: [libc::c_char; 6],
    pub deaths: u16_0,
    pub playerName: [libc::c_char; 8],
    pub n64ddFlag: s16,
    pub healthCapacity: s16,
    pub health: s16,
    pub magicLevel: s8,
    pub magic: s8,
    pub rupees: s16,
    pub swordHealth: u16_0,
    pub naviTimer: u16_0,
    pub magicAcquired: u8_0,
    pub unk_3B: [libc::c_char; 1],
    pub doubleMagic: u8_0,
    pub doubleDefense: u8_0,
    pub bgsFlag: u8_0,
    pub ocarinaGameRoundNum: u8_0,
    pub childEquips: ItemEquips,
    pub adultEquips: ItemEquips,
    pub unk_54: u32_0,
    pub unk_58: [libc::c_char; 14],
    pub savedSceneNum: s16,
    pub equips: ItemEquips,
    pub inventory: Inventory,
    pub sceneFlags: [SavedSceneFlags; 124],
    pub fw: FaroresWindData,
    pub unk_E8C: [libc::c_char; 16],
    pub gsFlags: [s32; 6],
    pub unk_EB4: [libc::c_char; 4],
    pub highScores: [s32; 7],
    pub eventChkInf: [u16_0; 14],
    pub itemGetInf: [u16_0; 4],
    pub infTable: [u16_0; 30],
    pub unk_F34: [libc::c_char; 4],
    pub worldMapAreaData: u32_0,
    pub unk_F3C: [libc::c_char; 4],
    pub scarecrowCustomSongSet: u8_0,
    pub scarecrowCustomSong: [u8_0; 864],
    pub unk_12A1: [libc::c_char; 36],
    pub scarecrowSpawnSongSet: u8_0,
    pub scarecrowSpawnSong: [u8_0; 128],
    pub unk_1346: [libc::c_char; 2],
    pub horseData: HorseData,
    pub checksum: u16_0,
    pub fileNum: s32,
    pub unk_1358: [libc::c_char; 4],
    pub gameMode: s32,
    pub sceneSetupIndex: s32,
    pub respawnFlag: s32,
    pub respawn: [RespawnData; 3],
    pub entranceSpeed: f32_0,
    pub entranceSound: u16_0,
    pub unk_13C2: [libc::c_char; 1],
    pub unk_13C3: u8_0,
    pub dogParams: s16,
    pub textTriggerFlags: u8_0,
    pub showTitleCard: u8_0,
    pub nayrusLoveTimer: s16,
    pub unk_13CA: [libc::c_char; 2],
    pub rupeeAccumulator: s16,
    pub timer1State: s16,
    pub timer1Value: s16,
    pub timer2State: s16,
    pub timer2Value: s16,
    pub timerX: [s16; 2],
    pub timerY: [s16; 2],
    pub unk_13DE: [libc::c_char; 2],
    pub seqId: u8_0,
    pub natureAmbienceId: u8_0,
    pub buttonStatus: [u8_0; 5],
    pub unk_13E7: u8_0,
    pub unk_13E8: u16_0,
    pub unk_13EA: u16_0,
    pub unk_13EC: u16_0,
    pub unk_13EE: u16_0,
    pub unk_13F0: s16,
    pub unk_13F2: s16,
    pub unk_13F4: s16,
    pub unk_13F6: s16,
    pub unk_13F8: s16,
    pub eventInf: [u16_0; 4],
    pub mapIndex: u16_0,
    pub minigameState: u16_0,
    pub minigameScore: u16_0,
    pub unk_1408: [libc::c_char; 1],
    pub language: u8_0,
    pub audioSetting: u8_0,
    pub unk_140B: [libc::c_char; 1],
    pub zTargetSetting: u8_0,
    pub forcedSeqId: u16_0,
    pub unk_1410: u8_0,
    pub unk_1411: [libc::c_char; 1],
    pub nextCutsceneIndex: u16_0,
    pub cutsceneTrigger: u8_0,
    pub chamberCutsceneNum: u8_0,
    pub nextDayTime: u16_0,
    pub fadeDuration: u8_0,
    pub unk_1419: u8_0,
    pub skyboxTime: u16_0,
    pub dogIsLost: u8_0,
    pub nextTransition: u8_0,
    pub unk_141E: [libc::c_char; 2],
    pub worldMapArea: s16,
    pub sunsSongState: s16,
    pub healthAccumulator: s16,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SUNSSONG_SPECIAL: C2RustUnnamed_2 = 3;
pub const SUNSSONG_SPEED_TIME: C2RustUnnamed_2 = 2;
pub const SUNSSONG_START: C2RustUnnamed_2 = 1;
pub const SUNSSONG_INACTIVE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color_RGB8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Color_RGBA8_u32 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color_RGBAf {
    pub r: f32_0,
    pub g: f32_0,
    pub b: f32_0,
    pub a: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightPoint {
    pub x: s16,
    pub y: s16,
    pub z: s16,
    pub color: [u8_0; 3],
    pub drawGlow: u8_0,
    pub radius: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightDirectional {
    pub x: s8,
    pub y: s8,
    pub z: s8,
    pub color: [u8_0; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LightParams {
    pub point: LightPoint,
    pub dir: LightDirectional,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightInfo {
    pub type_0: u8_0,
    pub params: LightParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lights {
    pub numLights: u8_0,
    pub l: Lightsn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightNode {
    pub info: *mut LightInfo,
    pub prev: *mut LightNode,
    pub next: *mut LightNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightContext {
    pub listHead: *mut LightNode,
    pub ambientColor: [u8_0; 3],
    pub fogColor: [u8_0; 3],
    pub fogNear: s16,
    pub fogFar: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlobalContext {
    pub state: GameState,
    pub sceneNum: s16,
    pub sceneConfig: u8_0,
    pub unk_A7: [libc::c_char; 9],
    pub sceneSegment: *mut libc::c_void,
    pub view: View,
    pub mainCamera: Camera,
    pub subCameras: [Camera; 3],
    pub cameraPtrs: [*mut Camera; 4],
    pub activeCamera: s16,
    pub nextCamera: s16,
    pub sequenceCtx: SequenceContext,
    pub lightCtx: LightContext,
    pub frameAdvCtx: FrameAdvanceContext,
    pub colCtx: CollisionContext,
    pub actorCtx: ActorContext,
    pub csCtx: CutsceneContext,
    pub soundSources: [SoundSource; 16],
    pub sramCtx: SramContext,
    pub skyboxCtx: SkyboxContext,
    pub msgCtx: MessageContext,
    pub interfaceCtx: InterfaceContext,
    pub pauseCtx: PauseContext,
    pub gameOverCtx: GameOverContext,
    pub envCtx: EnvironmentContext,
    pub animationCtx: AnimationContext,
    pub objectCtx: ObjectContext,
    pub roomCtx: RoomContext,
    pub transiActorCtx: TransitionActorContext,
    pub playerInit: Option<unsafe extern "C" fn(_: *mut Player,
                                                _: *mut GlobalContext,
                                                _: *mut FlexSkeletonHeader)
                               -> ()>,
    pub playerUpdate: Option<unsafe extern "C" fn(_: *mut Player,
                                                  _: *mut GlobalContext,
                                                  _: *mut Input) -> ()>,
    pub isPlayerDroppingFish: Option<unsafe extern "C" fn(_:
                                                              *mut GlobalContext)
                                         -> s32>,
    pub startPlayerFishing: Option<unsafe extern "C" fn(_: *mut GlobalContext)
                                       -> s32>,
    pub grabPlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                _: *mut Player) -> s32>,
    pub startPlayerCutscene: Option<unsafe extern "C" fn(_:
                                                             *mut GlobalContext,
                                                         _: *mut Actor,
                                                         _: s32) -> s32>,
    pub func_11D54: Option<unsafe extern "C" fn(_: *mut Player,
                                                _: *mut GlobalContext) -> ()>,
    pub damagePlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                  _: s32) -> s32>,
    pub talkWithPlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                    _: *mut Actor) -> ()>,
    pub viewProjectionMtxF: MtxF,
    pub billboardMtxF: MtxF,
    pub billboardMtx: *mut Mtx,
    pub gameplayFrames: u32_0,
    pub linkAgeOnLoad: u8_0,
    pub unk_11DE9: u8_0,
    pub curSpawn: u8_0,
    pub numSetupActors: u8_0,
    pub numRooms: u8_0,
    pub roomList: *mut RomFile,
    pub linkActorEntry: *mut ActorEntry,
    pub setupActorList: *mut ActorEntry,
    pub unk_11DFC: *mut libc::c_void,
    pub setupEntranceList: *mut EntranceEntry,
    pub setupExitList: *mut s16,
    pub setupPathList: *mut Path,
    pub cUpElfMsgs: *mut ElfMessage,
    pub specialEffects: *mut libc::c_void,
    pub skyboxId: u8_0,
    pub sceneLoadFlag: s8,
    pub unk_11E16: s16,
    pub unk_11E18: s16,
    pub nextEntranceIndex: s16,
    pub unk_11E1C: [libc::c_char; 64],
    pub shootingGalleryStatus: s8,
    pub bombchuBowlingStatus: s8,
    pub fadeTransition: u8_0,
    pub colChkCtx: CollisionCheckContext,
    pub envFlags: [u16_0; 20],
    pub pauseBgPreRender: PreRender,
    pub unk_12174: [libc::c_char; 83],
    pub unk_121C7: s8,
    pub transitionCtx: TransitionContext,
    pub unk_12418: [libc::c_char; 3],
    pub transitionMode: u8_0,
    pub transitionFade: TransitionFade,
    pub unk_12428: [libc::c_char; 3],
    pub unk_1242B: u8_0,
    pub loadedScene: *mut SceneTableEntry,
    pub unk_12430: [libc::c_char; 232],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SceneTableEntry {
    pub sceneFile: RomFile,
    pub titleFile: RomFile,
    pub unk_10: u8_0,
    pub config: u8_0,
    pub unk_12: u8_0,
    pub unk_13: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RomFile {
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionFade {
    pub fadeType: u8_0,
    pub isDone: u8_0,
    pub fadeDirection: u8_0,
    pub fadeColor: Color_RGBA8_u32,
    pub fadeTimer: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionContext {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub transitionType: s32,
    pub init: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                         -> *mut libc::c_void>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub update: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                           -> ()>,
    pub draw: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut *mut Gfx) -> ()>,
    pub start: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub setType: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                            -> ()>,
    pub setColor: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0)
                             -> ()>,
    pub setEnvColor: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: u32_0) -> ()>,
    pub isDone: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> s32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub fade: TransitionFade,
    pub circle: TransitionCircle,
    pub triforce: TransitionTriforce,
    pub wipe: TransitionWipe,
    pub data: [libc::c_char; 552],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionWipe {
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub direction: u8_0,
    pub frame: u8_0,
    pub isDone: u8_0,
    pub texX: u16_0,
    pub texY: u16_0,
    pub normal: u16_0,
    pub projection: Mtx,
    pub lookAt: Mtx,
    pub modelView: [[Mtx; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionTriforce {
    pub color: Color_RGBA8_u32,
    pub transPos: f32_0,
    pub step: f32_0,
    pub state: s32,
    pub fadeDirection: s32,
    pub projection: Mtx,
    pub frame: s32,
    pub modelView: [[Mtx; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionCircle {
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub texX: s32,
    pub texY: s32,
    pub step: s32,
    pub unk_14: u8_0,
    pub typeColor: u8_0,
    pub speed: u8_0,
    pub effect: u8_0,
    pub isDone: u8_0,
    pub frame: u8_0,
    pub normal: u16_0,
    pub projection: Mtx,
    pub lookAt: Mtx,
    pub texture: *mut libc::c_void,
    pub modelView: [[Mtx; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreRender {
    pub width: s32,
    pub height: s32,
    pub widthSave: s32,
    pub heightSave: s32,
    pub fbuf: *mut u16_0,
    pub fbufSave: *mut u16_0,
    pub cvgSave: *mut u8_0,
    pub zbuf: *mut u16_0,
    pub zbufSave: *mut u16_0,
    pub ulxSave: s32,
    pub ulySave: s32,
    pub lrxSave: s32,
    pub lrySave: s32,
    pub ulx: s32,
    pub uly: s32,
    pub lrx: s32,
    pub lry: s32,
    pub alloc: ListAlloc,
    pub unk_4C: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListAlloc {
    pub prev: *mut ListAlloc,
    pub next: *mut ListAlloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckContext {
    pub colATCount: s16,
    pub sacFlags: u16_0,
    pub colAT: [*mut Collider; 50],
    pub colACCount: s32,
    pub colAC: [*mut Collider; 60],
    pub colOCCount: s32,
    pub colOC: [*mut Collider; 50],
    pub colLineCount: s32,
    pub colLine: [*mut OcLine; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcLine {
    pub line: Linef,
    pub ocFlags: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Collider {
    pub actor: *mut Actor,
    pub at: *mut Actor,
    pub ac: *mut Actor,
    pub oc: *mut Actor,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
    pub ocFlags2: u8_0,
    pub colType: u8_0,
    pub shape: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Actor {
    pub id: s16,
    pub category: u8_0,
    pub room: s8,
    pub flags: u32_0,
    pub home: PosRot,
    pub params: s16,
    pub objBankIndex: s8,
    pub targetMode: s8,
    pub sfx: u16_0,
    pub world: PosRot,
    pub focus: PosRot,
    pub targetArrowOffset: f32_0,
    pub scale: Vec3f,
    pub velocity: Vec3f,
    pub speedXZ: f32_0,
    pub gravity: f32_0,
    pub minVelocityY: f32_0,
    pub wallPoly: *mut CollisionPoly,
    pub floorPoly: *mut CollisionPoly,
    pub wallBgId: u8_0,
    pub floorBgId: u8_0,
    pub wallYaw: s16,
    pub floorHeight: f32_0,
    pub yDistToWater: f32_0,
    pub bgCheckFlags: u16_0,
    pub yawTowardsPlayer: s16,
    pub xyzDistToPlayerSq: f32_0,
    pub xzDistToPlayer: f32_0,
    pub yDistToPlayer: f32_0,
    pub colChkInfo: CollisionCheckInfo,
    pub shape: ActorShape,
    pub projectedPos: Vec3f,
    pub projectedW: f32_0,
    pub uncullZoneForward: f32_0,
    pub uncullZoneScale: f32_0,
    pub uncullZoneDownward: f32_0,
    pub prevPos: Vec3f,
    pub isTargeted: u8_0,
    pub targetPriority: u8_0,
    pub textId: u16_0,
    pub freezeTimer: u16_0,
    pub colorFilterParams: u16_0,
    pub colorFilterTimer: u8_0,
    pub isDrawn: u8_0,
    pub dropFlag: u8_0,
    pub naviEnemyId: u8_0,
    pub parent: *mut Actor,
    pub child: *mut Actor,
    pub prev: *mut Actor,
    pub next: *mut Actor,
    pub init: ActorFunc,
    pub destroy: ActorFunc,
    pub update: ActorFunc,
    pub draw: ActorFunc,
    pub overlayEntry: *mut ActorOverlay,
    pub dbgPad: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorOverlay {
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
    pub vramStart: *mut libc::c_void,
    pub vramEnd: *mut libc::c_void,
    pub loadedRamAddr: *mut libc::c_void,
    pub initInfo: *mut ActorInit,
    pub name: *mut libc::c_char,
    pub allocType: u16_0,
    pub numLoaded: s8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorInit {
    pub id: s16,
    pub category: u8_0,
    pub flags: u32_0,
    pub objectId: s16,
    pub instanceSize: u32_0,
    pub init: ActorFunc,
    pub destroy: ActorFunc,
    pub update: ActorFunc,
    pub draw: ActorFunc,
}
pub type ActorFunc
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorShape {
    pub rot: Vec3s,
    pub face: s16,
    pub yOffset: f32_0,
    pub shadowDraw: ActorShadowFunc,
    pub shadowScale: f32_0,
    pub shadowAlpha: u8_0,
    pub feetFloorFlags: u8_0,
    pub feetPos: [Vec3f; 2],
}
pub type ActorShadowFunc
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut Lights,
                                _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckInfo {
    pub damageTable: *mut DamageTable,
    pub displacement: Vec3f,
    pub cylRadius: s16,
    pub cylHeight: s16,
    pub cylYShift: s16,
    pub mass: u8_0,
    pub health: u8_0,
    pub damage: u8_0,
    pub damageEffect: u8_0,
    pub atHitEffect: u8_0,
    pub acHitEffect: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DamageTable {
    pub table: [u8_0; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionPoly {
    pub type_0: u16_0,
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub flags_vIA: u16_0,
    pub flags_vIB: u16_0,
    pub vIC: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosRot {
    pub pos: Vec3f,
    pub rot: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ElfMessage {
    pub byte0: u8_0,
    pub byte1: u8_0,
    pub byte2: u8_0,
    pub byte3: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Path {
    pub count: u8_0,
    pub points: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntranceEntry {
    pub spawn: u8_0,
    pub room: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorEntry {
    pub id: s16,
    pub pos: Vec3s,
    pub rot: Vec3s,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Player {
    pub actor: Actor,
    pub currentTunic: s8,
    pub currentSword: s8,
    pub currentShield: s8,
    pub currentBoots: s8,
    pub heldItemButton: s8,
    pub heldItemActionParam: s8,
    pub heldItemId: u8_0,
    pub prevBoots: s8,
    pub itemActionParam: s8,
    pub unk_155: [libc::c_char; 3],
    pub modelGroup: u8_0,
    pub nextModelGroup: u8_0,
    pub unk_15A: s8,
    pub modelAnimType: u8_0,
    pub leftHandType: u8_0,
    pub rightHandType: u8_0,
    pub sheathType: u8_0,
    pub currentMask: u8_0,
    pub rightHandDLists: *mut *mut Gfx,
    pub leftHandDLists: *mut *mut Gfx,
    pub sheathDLists: *mut *mut Gfx,
    pub waistDLists: *mut *mut Gfx,
    pub giObjectLoading: u8_0,
    pub giObjectDmaRequest: DmaRequest,
    pub giObjectLoadQueue: OSMesgQueue,
    pub giObjectLoadMsg: OSMesg,
    pub giObjectSegment: *mut libc::c_void,
    pub skelAnime: SkelAnime,
    pub jointTable: [Vec3s; 24],
    pub morphTable: [Vec3s; 24],
    pub blendTable: [Vec3s; 24],
    pub unk_3A8: [s16; 2],
    pub heldActor: *mut Actor,
    pub leftHandPos: Vec3f,
    pub unk_3BC: Vec3s,
    pub unk_3C4: *mut Actor,
    pub unk_3C8: Vec3f,
    pub unk_3D4: [libc::c_char; 88],
    pub doorType: s8,
    pub doorDirection: s8,
    pub doorTimer: s16,
    pub doorActor: *mut Actor,
    pub getItemId: s8,
    pub getItemDirection: u16_0,
    pub interactRangeActor: *mut Actor,
    pub mountSide: s8,
    pub unk_43D: [libc::c_char; 3],
    pub rideActor: *mut Actor,
    pub csMode: u8_0,
    pub prevCsMode: u8_0,
    pub unk_446: u8_0,
    pub unk_447: u8_0,
    pub unk_448: *mut Actor,
    pub unk_44C: [libc::c_char; 4],
    pub unk_450: Vec3f,
    pub unk_45C: Vec3f,
    pub unk_468: [libc::c_char; 2],
    pub unk_46A: s16,
    pub unk_46C: s16,
    pub unk_46E: [libc::c_char; 42],
    pub cylinder: ColliderCylinder,
    pub swordQuads: [ColliderQuad; 2],
    pub shieldQuad: ColliderQuad,
    pub unk_664: *mut Actor,
    pub unk_668: [libc::c_char; 4],
    pub unk_66C: s32,
    pub swordEffectIndex: s32,
    pub func_674: PlayerFunc674,
    pub ageProperties: *mut PlayerAgeProperties,
    pub stateFlags1: u32_0,
    pub stateFlags2: u32_0,
    pub unk_684: *mut Actor,
    pub boomerangActor: *mut Actor,
    pub naviActor: *mut Actor,
    pub naviTextId: s16,
    pub stateFlags3: u8_0,
    pub exchangeItemId: s8,
    pub targetActor: *mut Actor,
    pub targetActorDistance: f32_0,
    pub unk_69C: [libc::c_char; 4],
    pub unk_6A0: f32_0,
    pub unk_6A4: f32_0,
    pub unk_6A8: *mut Actor,
    pub unk_6AC: s8,
    pub unk_6AD: u8_0,
    pub unk_6AE: u16_0,
    pub unk_6B0: s16,
    pub unk_6B4: [libc::c_char; 4],
    pub unk_6B6: s16,
    pub unk_6B8: s16,
    pub unk_6BA: s16,
    pub unk_6BC: s16,
    pub unk_6BE: s16,
    pub unk_6C0: s16,
    pub unk_6C2: s16,
    pub unk_6C4: f32_0,
    pub skelAnime2: SkelAnime,
    pub jointTable2: [Vec3s; 24],
    pub morphTable2: [Vec3s; 24],
    pub func_82C: PlayerFunc82C,
    pub unk_830: f32_0,
    pub unk_834: s16,
    pub unk_836: s8,
    pub unk_837: u8_0,
    pub linearVelocity: f32_0,
    pub currentYaw: s16,
    pub targetYaw: s16,
    pub unk_840: u16_0,
    pub swordAnimation: s8,
    pub swordState: s8,
    pub unk_844: s8,
    pub unk_845: u8_0,
    pub unk_846: u8_0,
    pub unk_847: [s8; 4],
    pub unk_84B: [s8; 4],
    pub unk_84F: s8,
    pub unk_850: s16,
    pub unk_854: f32_0,
    pub unk_858: f32_0,
    pub unk_85C: f32_0,
    pub unk_860: s16,
    pub unk_862: s8,
    pub unk_864: f32_0,
    pub unk_868: f32_0,
    pub unk_86C: f32_0,
    pub unk_870: f32_0,
    pub unk_874: f32_0,
    pub unk_878: f32_0,
    pub unk_87C: s16,
    pub unk_87E: s16,
    pub unk_880: f32_0,
    pub wallHeight: f32_0,
    pub wallDistance: f32_0,
    pub unk_88C: u8_0,
    pub unk_88D: u8_0,
    pub unk_88E: u8_0,
    pub unk_88F: u8_0,
    pub unk_890: u8_0,
    pub shockTimer: u8_0,
    pub unk_892: u8_0,
    pub hoverBootsTimer: u8_0,
    pub fallStartHeight: s16,
    pub fallDistance: s16,
    pub unk_898: s16,
    pub unk_89A: s16,
    pub unk_89C: s16,
    pub unk_89E: u16_0,
    pub unk_8A0: u8_0,
    pub unk_8A1: u8_0,
    pub unk_8A2: s16,
    pub unk_8A4: f32_0,
    pub unk_8A8: f32_0,
    pub windSpeed: f32_0,
    pub windDirection: s16,
    pub swordInfo: [WeaponInfo; 3],
    pub bodyPartsPos: [Vec3f; 18],
    pub mf_9E0: MtxF,
    pub shieldMf: MtxF,
    pub isBurning: u8_0,
    pub flameTimers: [u8_0; 18],
    pub unk_A73: u8_0,
    pub func_A74: PlayerFuncA74,
    pub invincibilityTimer: s8,
    pub unk_A79: u8_0,
    pub unk_A7A: u8_0,
    pub unk_A7B: u8_0,
    pub unk_A7C: f32_0,
    pub unk_A80: s16,
    pub unk_A82: u16_0,
    pub unk_A84: s16,
    pub unk_A86: s8,
    pub unk_A87: u8_0,
    pub unk_A88: Vec3f,
}
pub type PlayerFuncA74
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut Player) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WeaponInfo {
    pub active: s32,
    pub tip: Vec3f,
    pub base: Vec3f,
}
pub type PlayerFunc82C
    =
    Option<unsafe extern "C" fn(_: *mut Player, _: *mut GlobalContext)
               -> s32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelAnime {
    pub limbCount: u8_0,
    pub mode: u8_0,
    pub dListCount: u8_0,
    pub taper: s8,
    pub skeleton: *mut *mut libc::c_void,
    pub animation: *mut libc::c_void,
    pub startFrame: f32_0,
    pub endFrame: f32_0,
    pub animLength: f32_0,
    pub curFrame: f32_0,
    pub playSpeed: f32_0,
    pub jointTable: *mut Vec3s,
    pub morphTable: *mut Vec3s,
    pub morphWeight: f32_0,
    pub morphRate: f32_0,
    pub update: Option<unsafe extern "C" fn() -> s32>,
    pub initFlags: s8,
    pub moveFlags: u8_0,
    pub prevRot: s16,
    pub prevTransl: Vec3s,
    pub baseTransl: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerAgeProperties {
    pub unk_00: f32_0,
    pub unk_04: f32_0,
    pub unk_08: f32_0,
    pub unk_0C: f32_0,
    pub unk_10: f32_0,
    pub unk_14: f32_0,
    pub unk_18: f32_0,
    pub unk_1C: f32_0,
    pub unk_20: f32_0,
    pub unk_24: f32_0,
    pub unk_28: f32_0,
    pub unk_2C: f32_0,
    pub unk_30: f32_0,
    pub unk_34: f32_0,
    pub unk_38: f32_0,
    pub unk_3C: f32_0,
    pub unk_40: f32_0,
    pub unk_44: Vec3s,
    pub unk_4A: [Vec3s; 4],
    pub unk_62: [Vec3s; 4],
    pub unk_7A: [Vec3s; 2],
    pub unk_86: [Vec3s; 2],
    pub unk_92: u16_0,
    pub unk_94: u16_0,
    pub unk_98: *mut LinkAnimationHeader,
    pub unk_9C: *mut LinkAnimationHeader,
    pub unk_A0: *mut LinkAnimationHeader,
    pub unk_A4: *mut LinkAnimationHeader,
    pub unk_A8: *mut LinkAnimationHeader,
    pub unk_AC: [*mut LinkAnimationHeader; 4],
    pub unk_BC: [*mut LinkAnimationHeader; 2],
    pub unk_C4: [*mut LinkAnimationHeader; 2],
    pub unk_CC: [*mut LinkAnimationHeader; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkAnimationHeader {
    pub common: AnimationHeaderCommon,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationHeaderCommon {
    pub frameCount: s16,
}
pub type PlayerFunc674
    =
    Option<unsafe extern "C" fn(_: *mut Player, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuad {
    pub base: Collider,
    pub info: ColliderInfo,
    pub dim: ColliderQuadDim,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuadDim {
    pub quad: [Vec3f; 4],
    pub dcMid: Vec3s,
    pub baMid: Vec3s,
    pub acDist: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderInfo {
    pub toucher: ColliderTouch,
    pub bumper: ColliderBump,
    pub elemType: u8_0,
    pub toucherFlags: u8_0,
    pub bumperFlags: u8_0,
    pub ocElemFlags: u8_0,
    pub atHit: *mut Collider,
    pub acHit: *mut Collider,
    pub atHitInfo: *mut ColliderInfo,
    pub acHitInfo: *mut ColliderInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderBump {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub defense: u8_0,
    pub hitPos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTouch {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub damage: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderCylinder {
    pub base: Collider,
    pub info: ColliderInfo,
    pub dim: Cylinder16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DmaRequest {
    pub vromAddr: u32_0,
    pub dramAddr: *mut libc::c_void,
    pub size: u32_0,
    pub filename: *const libc::c_char,
    pub line: s32,
    pub unk_14: s32,
    pub notifyQueue: *mut OSMesgQueue,
    pub notifyMsg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FlexSkeletonHeader {
    pub sh: SkeletonHeader,
    pub dListCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkeletonHeader {
    pub segment: *mut *mut libc::c_void,
    pub limbCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionActorContext {
    pub numActors: u8_0,
    pub list: *mut TransitionActorEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionActorEntry {
    pub sides: [C2RustUnnamed_7; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub room: s8,
    pub effects: s8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RoomContext {
    pub curRoom: Room,
    pub prevRoom: Room,
    pub bufPtrs: [*mut libc::c_void; 2],
    pub unk_30: u8_0,
    pub status: s8,
    pub unk_34: *mut libc::c_void,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub unk_74: [s16; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Room {
    pub num: s8,
    pub unk_01: u8_0,
    pub unk_02: u8_0,
    pub unk_03: u8_0,
    pub echo: s8,
    pub showInvisActors: u8_0,
    pub mesh: *mut Mesh,
    pub segment: *mut libc::c_void,
    pub unk_10: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Mesh {
    pub polygon: Polygon,
    pub polygon0: PolygonType0,
    pub polygon1: PolygonType1,
    pub polygon2: PolygonType2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType2 {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType1 {
    pub type_0: u8_0,
    pub format: u8_0,
    pub dlist: *mut Gfx,
    pub c2rust_unnamed: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub single: C2RustUnnamed_10,
    pub multi: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub count: u8_0,
    pub list: *mut BgImage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgImage {
    pub unk_00: u16_0,
    pub id: u8_0,
    pub source: u32_0,
    pub unk_0C: u32_0,
    pub tlut: u32_0,
    pub width: u16_0,
    pub height: u16_0,
    pub fmt: u8_0,
    pub siz: u8_0,
    pub mode0: u16_0,
    pub tlutCount: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub source: *mut libc::c_void,
    pub unk_0C: u32_0,
    pub tlut: *mut libc::c_void,
    pub width: u16_0,
    pub height: u16_0,
    pub fmt: u8_0,
    pub siz: u8_0,
    pub mode0: u16_0,
    pub tlutCount: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType0 {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Polygon {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectContext {
    pub spaceStart: *mut libc::c_void,
    pub spaceEnd: *mut libc::c_void,
    pub num: u8_0,
    pub unk_09: u8_0,
    pub mainKeepIndex: u8_0,
    pub subKeepIndex: u8_0,
    pub status: [ObjectStatus; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectStatus {
    pub id: s16,
    pub segment: *mut libc::c_void,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationContext {
    pub animationCount: s16,
    pub entries: [AnimationEntry; 50],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationEntry {
    pub type_0: u8_0,
    pub data: AnimationEntryData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union AnimationEntryData {
    pub load: AnimEntryLoadFrame,
    pub copy: AnimEntryCopyAll,
    pub interp: AnimEntryInterp,
    pub copy1: AnimEntryCopyTrue,
    pub copy0: AnimEntryCopyFalse,
    pub move_0: AnimEntryMoveActor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryMoveActor {
    pub actor: *mut Actor,
    pub skelAnime: *mut SkelAnime,
    pub unk_08: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyFalse {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
    pub copyFlag: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyTrue {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
    pub copyFlag: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryInterp {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub base: *mut Vec3s,
    pub mod_0: *mut Vec3s,
    pub weight: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyAll {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryLoadFrame {
    pub req: DmaRequest,
    pub msgQueue: OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnvironmentContext {
    pub unk_00: [libc::c_char; 2],
    pub timeIncrement: u16_0,
    pub sunPos: Vec3f,
    pub skybox1Index: u8_0,
    pub skybox2Index: u8_0,
    pub unk_12: [libc::c_char; 1],
    pub skyboxBlend: u8_0,
    pub unk_14: [libc::c_char; 1],
    pub skyboxDisabled: u8_0,
    pub sunMoonDisabled: u8_0,
    pub unk_17: u8_0,
    pub unk_18: u8_0,
    pub unk_19: u8_0,
    pub unk_1A: u16_0,
    pub unk_1C: [libc::c_char; 2],
    pub indoors: u8_0,
    pub unk_1F: u8_0,
    pub unk_20: u8_0,
    pub unk_21: u8_0,
    pub unk_22: u16_0,
    pub unk_24: u16_0,
    pub unk_26: [libc::c_char; 2],
    pub dirLight1: LightInfo,
    pub dirLight2: LightInfo,
    pub skyboxDmaState: s8,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub unk_84: f32_0,
    pub unk_88: f32_0,
    pub adjAmbientColor: [s16; 3],
    pub adjLight1Color: [s16; 3],
    pub adjFogColor: [s16; 3],
    pub adjFogNear: s16,
    pub adjFogFar: s16,
    pub unk_A2: [libc::c_char; 6],
    pub windDirection: Vec3s,
    pub windSpeed: f32_0,
    pub numLightSettings: u8_0,
    pub lightSettingsList: *mut EnvLightSettings,
    pub blendIndoorLights: u8_0,
    pub unk_BD: u8_0,
    pub unk_BE: u8_0,
    pub unk_BF: u8_0,
    pub lightSettings: EnvLightSettings,
    pub unk_D6: u16_0,
    pub unk_D8: f32_0,
    pub unk_DC: u8_0,
    pub gloomySkyMode: u8_0,
    pub unk_DE: u8_0,
    pub lightningMode: u8_0,
    pub unk_E0: u8_0,
    pub fillScreen: u8_0,
    pub screenFillColor: [u8_0; 4],
    pub sandstormState: u8_0,
    pub sandstormPrimA: u8_0,
    pub sandstormEnvA: u8_0,
    pub customSkyboxFilter: u8_0,
    pub skyboxFilterColor: [u8_0; 4],
    pub unk_EE: [u8_0; 4],
    pub unk_F2: [u8_0; 4],
    pub unk_F6: [libc::c_char; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnvLightSettings {
    pub ambientColor: [u8_0; 3],
    pub light1Dir: [s8; 3],
    pub light1Color: [u8_0; 3],
    pub light2Dir: [s8; 3],
    pub light2Color: [u8_0; 3],
    pub fogColor: [u8_0; 3],
    pub fogNear: s16,
    pub fogFar: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameOverContext {
    pub state: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PauseContext {
    pub view: View,
    pub iconItemSegment: *mut u8_0,
    pub iconItem24Segment: *mut u8_0,
    pub iconItemAltSegment: *mut u8_0,
    pub iconItemLangSegment: *mut u8_0,
    pub nameSegment: *mut u8_0,
    pub playerSegment: *mut u8_0,
    pub unk_140: [libc::c_char; 4],
    pub itemPageVtx: *mut Vtx,
    pub equipPageVtx: *mut Vtx,
    pub mapPageVtx: *mut Vtx,
    pub questPageVtx: *mut Vtx,
    pub infoPanelVtx: *mut Vtx,
    pub itemVtx: *mut Vtx,
    pub equipVtx: *mut Vtx,
    pub unk_160: [libc::c_char; 4],
    pub questVtx: *mut Vtx,
    pub cursorVtx: *mut Vtx,
    pub saveVtx: *mut Vtx,
    pub unk_170: [libc::c_char; 36],
    pub ocarinaStaff: *mut OcarinaStaff,
    pub unk_198: [libc::c_char; 32],
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub state: u16_0,
    pub debugState: u16_0,
    pub eye: Vec3f,
    pub unk_1E4: u16_0,
    pub mode: u16_0,
    pub pageIndex: u16_0,
    pub unk_1EA: u16_0,
    pub unk_1EC: u16_0,
    pub unk_1F0: f32_0,
    pub unk_1F4: f32_0,
    pub unk_1F8: f32_0,
    pub unk_1FC: f32_0,
    pub unk_200: f32_0,
    pub unk_204: f32_0,
    pub alpha: u16_0,
    pub offsetY: s16,
    pub unk_20C: [libc::c_char; 8],
    pub stickRelX: s16,
    pub stickRelY: s16,
    pub cursorPoint: [s16; 5],
    pub cursorX: [s16; 5],
    pub cursorY: [s16; 5],
    pub dungeonMapSlot: s16,
    pub cursorSpecialPos: s16,
    pub pageSwitchTimer: s16,
    pub namedItem: u16_0,
    pub cursorItem: [u16_0; 4],
    pub cursorSlot: [u16_0; 4],
    pub equipTargetItem: u16_0,
    pub equipTargetSlot: u16_0,
    pub equipTargetCBtn: u16_0,
    pub equipAnimX: s16,
    pub equipAnimY: s16,
    pub equipAnimAlpha: s16,
    pub infoPanelOffsetY: s16,
    pub nameDisplayTimer: u16_0,
    pub nameColorSet: u16_0,
    pub cursorColorSet: s16,
    pub promptChoice: s16,
    pub ocarinaSongIdx: s16,
    pub worldMapPoints: [u8_0; 20],
    pub tradeQuestLocation: u8_0,
    pub playerSkelAnime: SkelAnime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcarinaStaff {
    pub noteIdx: u8_0,
    pub state: u8_0,
    pub pos: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct View {
    pub magic: s32,
    pub gfxCtx: *mut GraphicsContext,
    pub viewport: Viewport,
    pub fovy: f32_0,
    pub zNear: f32_0,
    pub zFar: f32_0,
    pub scale: f32_0,
    pub eye: Vec3f,
    pub lookAt: Vec3f,
    pub up: Vec3f,
    pub vp: Vp,
    pub projection: Mtx,
    pub viewing: Mtx,
    pub projectionPtr: *mut Mtx,
    pub viewingPtr: *mut Mtx,
    pub unk_E8: Vec3f,
    pub unk_F4: Vec3f,
    pub unk_100: f32_0,
    pub unk_104: Vec3f,
    pub unk_110: Vec3f,
    pub normal: u16_0,
    pub flags: s32,
    pub unk_124: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Viewport {
    pub topY: s32,
    pub bottomY: s32,
    pub leftX: s32,
    pub rightX: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GraphicsContext {
    pub polyOpaBuffer: *mut Gfx,
    pub polyXluBuffer: *mut Gfx,
    pub unk_008: [libc::c_char; 8],
    pub overlayBuffer: *mut Gfx,
    pub unk_014: u32_0,
    pub unk_018: [libc::c_char; 32],
    pub msgBuff: [OSMesg; 8],
    pub schedMsgQ: *mut OSMesgQueue,
    pub queue: OSMesgQueue,
    pub unk_074: [libc::c_char; 4],
    pub task: OSScTask,
    pub unk_0D0: [libc::c_char; 224],
    pub workBuffer: *mut Gfx,
    pub work: TwoHeadGfxArena,
    pub unk_01C4: [libc::c_char; 192],
    pub viMode: *mut OSViMode,
    pub unk_0288: [libc::c_char; 32],
    pub overlay: TwoHeadGfxArena,
    pub polyOpa: TwoHeadGfxArena,
    pub polyXlu: TwoHeadGfxArena,
    pub gfxPoolIdx: u32_0,
    pub curFrameBuffer: *mut u16_0,
    pub unk_2E0: [libc::c_char; 4],
    pub viFeatures: u32_0,
    pub fbIdx: s32,
    pub callback: Option<unsafe extern "C" fn(_: *mut GraphicsContext,
                                              _: *mut libc::c_void) -> ()>,
    pub callbackParam: *mut libc::c_void,
    pub xScale: f32_0,
    pub yScale: f32_0,
    pub unk_2FC: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TwoHeadGfxArena {
    pub size: u32_0,
    pub bufp: *mut Gfx,
    pub p: *mut Gfx,
    pub d: *mut Gfx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSScTask {
    pub next: *mut OSScTask,
    pub state: u32_0,
    pub flags: u32_0,
    pub framebuffer: *mut CfbInfo,
    pub list: OSTask,
    pub msgQ: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CfbInfo {
    pub fb1: *mut u16_0,
    pub swapBuffer: *mut u16_0,
    pub viMode: *mut OSViMode,
    pub features: u32_0,
    pub unk_10: u8_0,
    pub updateRate: s8,
    pub updateRate2: s8,
    pub unk_13: u8_0,
    pub xScale: f32_0,
    pub yScale: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterfaceContext {
    pub view: View,
    pub actionVtx: *mut Vtx,
    pub beatingHeartVtx: *mut Vtx,
    pub parameterSegment: *mut u8_0,
    pub doActionSegment: *mut u8_0,
    pub iconItemSegment: *mut u8_0,
    pub mapSegment: *mut u8_0,
    pub mapPalette: [u8_0; 32],
    pub dmaRequest_160: DmaRequest,
    pub dmaRequest_180: DmaRequest,
    pub unk_1A0: [libc::c_char; 32],
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub viewport: Viewport,
    pub unk_1EC: s16,
    pub unk_1EE: u16_0,
    pub unk_1F0: u16_0,
    pub unk_1F4: f32_0,
    pub naviCalling: s16,
    pub unk_1FA: s16,
    pub unk_1FC: s16,
    pub unk_1FE: s16,
    pub unk_200: s16,
    pub beatingHeartPrim: [s16; 3],
    pub beatingHeartEnv: [s16; 3],
    pub heartsPrimR: [s16; 2],
    pub heartsPrimG: [s16; 2],
    pub heartsPrimB: [s16; 2],
    pub heartsEnvR: [s16; 2],
    pub heartsEnvG: [s16; 2],
    pub heartsEnvB: [s16; 2],
    pub unk_226: s16,
    pub unk_228: s16,
    pub unk_22A: s16,
    pub unk_22C: s16,
    pub unk_22E: s16,
    pub unk_230: s16,
    pub counterDigits: [s16; 4],
    pub numHorseBoosts: u8_0,
    pub unk_23C: u16_0,
    pub hbaAmmo: u16_0,
    pub unk_240: u16_0,
    pub unk_242: u16_0,
    pub unk_244: u16_0,
    pub aAlpha: u16_0,
    pub bAlpha: u16_0,
    pub cLeftAlpha: u16_0,
    pub cDownAlpha: u16_0,
    pub cRightAlpha: u16_0,
    pub healthAlpha: u16_0,
    pub magicAlpha: u16_0,
    pub minimapAlpha: u16_0,
    pub startAlpha: s16,
    pub unk_258: s16,
    pub unk_25A: s16,
    pub mapRoomNum: s16,
    pub mapPaletteIndex: s16,
    pub unk_260: u8_0,
    pub unk_261: u8_0,
    pub restrictions: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub hGauge: u8_0,
    pub bButton: u8_0,
    pub aButton: u8_0,
    pub bottles: u8_0,
    pub tradeItems: u8_0,
    pub hookshot: u8_0,
    pub ocarina: u8_0,
    pub warpSongs: u8_0,
    pub sunsSong: u8_0,
    pub farores: u8_0,
    pub dinsNayrus: u8_0,
    pub all: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MessageContext {
    pub view: View,
    pub font: Font,
    pub textboxSegment: *mut libc::c_void,
    pub unk_E2B4: [libc::c_char; 4],
    pub ocarinaStaff: *mut OcarinaStaff,
    pub unk_E2BC: [libc::c_char; 60],
    pub textId: u16_0,
    pub choiceTextId: u16_0,
    pub textBoxProperties: u8_0,
    pub textBoxType: u8_0,
    pub textBoxPos: u8_0,
    pub msgLength: s32,
    pub msgMode: u8_0,
    pub unk_E305: [libc::c_char; 1],
    pub msgBufDecoded: [u8_0; 200],
    pub msgBufPos: u16_0,
    pub unk_E3D0: u16_0,
    pub textDrawPos: u16_0,
    pub decodedTextLen: u16_0,
    pub textUnskippable: u16_0,
    pub textPosX: s16,
    pub textPosY: s16,
    pub textColorR: s16,
    pub textColorG: s16,
    pub textColorB: s16,
    pub textColorAlpha: s16,
    pub textboxEndType: u8_0,
    pub choiceIndex: u8_0,
    pub choiceNum: u8_0,
    pub stateTimer: u8_0,
    pub textDelayTimer: u16_0,
    pub textDelay: u16_0,
    pub lastPlayedSong: u16_0,
    pub ocarinaMode: u16_0,
    pub ocarinaAction: u16_0,
    pub unk_E3F2: u16_0,
    pub unk_E3F4: u16_0,
    pub textboxBackgroundIdx: u16_0,
    pub textboxBackgroundForeColorIdx: u8_0,
    pub textboxBackgroundBackColorIdx: u8_0,
    pub textboxBackgroundYOffsetIdx: u8_0,
    pub textboxBackgroundUnkArg: u8_0,
    pub unk_E3FC: [libc::c_char; 2],
    pub textboxColorRed: s16,
    pub textboxColorGreen: s16,
    pub textboxColorBlue: s16,
    pub textboxColorAlphaTarget: s16,
    pub textboxColorAlphaCurrent: s16,
    pub talkActor: *mut Actor,
    pub disableWarpSongs: s16,
    pub unk_E40E: s16,
    pub lastOcaNoteIdx: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub msgOffset: u32_0,
    pub msgLength: u32_0,
    pub charTexBuf: [u8_0; 15360],
    pub iconBuf: [u8_0; 128],
    pub fontBuf: [u8_0; 40960],
    pub c2rust_unnamed: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub msgBuf: [libc::c_char; 1280],
    pub msgBufWide: [u16_0; 640],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkyboxContext {
    pub unk_00: [libc::c_char; 296],
    pub staticSegments: [*mut libc::c_void; 2],
    pub palettes: *mut [u16_0; 256],
    pub dListBuf: *mut [Gfx; 150],
    pub unk_138: *mut Gfx,
    pub roomVtx: *mut Vtx,
    pub unk_140: s16,
    pub rot: Vec3f,
    pub unk_150: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SramContext {
    pub readBuff: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundSource {
    pub countdown: u16_0,
    pub originPos: Vec3f,
    pub relativePos: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CutsceneContext {
    pub unk_00: [libc::c_char; 4],
    pub segment: *mut libc::c_void,
    pub state: u8_0,
    pub unk_0C: f32_0,
    pub frames: u16_0,
    pub unk_12: u16_0,
    pub unk_14: s32,
    pub unk_18: u16_0,
    pub unk_1A: u8_0,
    pub unk_1B: u8_0,
    pub cameraFocus: *mut CutsceneCameraPoint,
    pub cameraPosition: *mut CutsceneCameraPoint,
    pub linkAction: *mut CsCmdActorAction,
    pub npcActions: [*mut CsCmdActorAction; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdActorAction {
    pub action: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub rot: Vec3s,
    pub urot: Vec3us,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CutsceneCameraPoint {
    pub continueFlag: s8,
    pub cameraRoll: s8,
    pub nextPointFrame: u16_0,
    pub viewAngle: f32_0,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorContext {
    pub freezeFlashTimer: u8_0,
    pub unk_01: [libc::c_char; 1],
    pub unk_02: u8_0,
    pub unk_03: u8_0,
    pub unk_04: [libc::c_char; 4],
    pub total: u8_0,
    pub unk_09: [libc::c_char; 3],
    pub actorLists: [ActorListEntry; 12],
    pub targetCtx: TargetContext,
    pub flags: C2RustUnnamed_14,
    pub titleCtx: TitleCardContext,
    pub unk_138: [libc::c_char; 4],
    pub absoluteSpace: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TitleCardContext {
    pub texture: *mut libc::c_void,
    pub x: s16,
    pub y: s16,
    pub width: u8_0,
    pub height: u8_0,
    pub durationTimer: u8_0,
    pub delayTimer: u8_0,
    pub alpha: s16,
    pub intensity: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub swch: u32_0,
    pub tempSwch: u32_0,
    pub unk0: u32_0,
    pub unk1: u32_0,
    pub chest: u32_0,
    pub clear: u32_0,
    pub tempClear: u32_0,
    pub collect: u32_0,
    pub tempCollect: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetContext {
    pub naviRefPos: Vec3f,
    pub targetCenterPos: Vec3f,
    pub naviInner: Color_RGBAf,
    pub naviOuter: Color_RGBAf,
    pub arrowPointedActor: *mut Actor,
    pub targetedActor: *mut Actor,
    pub unk_40: f32_0,
    pub unk_44: f32_0,
    pub unk_48: s16,
    pub activeCategory: u8_0,
    pub unk_4B: u8_0,
    pub unk_4C: s8,
    pub unk_4D: [libc::c_char; 3],
    pub arr_50: [TargetContextEntry; 3],
    pub unk_8C: *mut Actor,
    pub bgmEnemy: *mut Actor,
    pub unk_94: *mut Actor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetContextEntry {
    pub pos: Vec3f,
    pub unk_0C: f32_0,
    pub color: Color_RGB8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorListEntry {
    pub length: s32,
    pub head: *mut Actor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionContext {
    pub colHeader: *mut CollisionHeader,
    pub minBounds: Vec3f,
    pub maxBounds: Vec3f,
    pub subdivAmount: Vec3i,
    pub subdivLength: Vec3f,
    pub subdivLengthInv: Vec3f,
    pub lookupTbl: *mut StaticLookup,
    pub polyNodes: SSNodeList,
    pub dyna: DynaCollisionContext,
    pub memSize: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaCollisionContext {
    pub bitFlag: u8_0,
    pub bgActors: [BgActor; 50],
    pub bgActorFlags: [u16_0; 50],
    pub polyList: *mut CollisionPoly,
    pub vtxList: *mut Vec3s,
    pub polyNodes: DynaSSNodeList,
    pub polyNodesMax: s32,
    pub polyListMax: s32,
    pub vtxListMax: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaSSNodeList {
    pub tbl: *mut SSNode,
    pub count: s32,
    pub max: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSNode {
    pub polyId: s16,
    pub next: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgActor {
    pub actor: *mut Actor,
    pub colHeader: *mut CollisionHeader,
    pub dynaLookup: DynaLookup,
    pub vtxStartIndex: u16_0,
    pub prevTransform: ScaleRotPos,
    pub curTransform: ScaleRotPos,
    pub boundingSphere: Sphere16,
    pub minY: f32_0,
    pub maxY: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScaleRotPos {
    pub scale: Vec3f,
    pub rot: Vec3s,
    pub pos: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaLookup {
    pub polyStartIndex: u16_0,
    pub ceiling: SSList,
    pub wall: SSList,
    pub floor: SSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSList {
    pub head: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionHeader {
    pub minBounds: Vec3s,
    pub maxBounds: Vec3s,
    pub numVertices: u16_0,
    pub vtxList: *mut Vec3s,
    pub numPolygons: u16_0,
    pub polyList: *mut CollisionPoly,
    pub surfaceTypeList: *mut SurfaceType,
    pub cameraDataList: *mut CamData,
    pub numWaterBoxes: u16_0,
    pub waterBoxes: *mut WaterBox,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaterBox {
    pub xMin: s16,
    pub ySurface: s16,
    pub zMin: s16,
    pub xLength: s16,
    pub zLength: s16,
    pub properties: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CamData {
    pub cameraSType: u16_0,
    pub numCameras: s16,
    pub camPosData: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceType {
    pub data: [u32_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSNodeList {
    pub max: u16_0,
    pub count: u16_0,
    pub tbl: *mut SSNode,
    pub polyCheckTbl: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StaticLookup {
    pub floor: SSList,
    pub wall: SSList,
    pub ceiling: SSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameAdvanceContext {
    pub enabled: s32,
    pub timer: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SequenceContext {
    pub seqId: u8_0,
    pub natureAmbienceId: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Camera {
    pub paramData: [libc::c_char; 80],
    pub at: Vec3f,
    pub eye: Vec3f,
    pub up: Vec3f,
    pub eyeNext: Vec3f,
    pub skyboxOffset: Vec3f,
    pub globalCtx: *mut GlobalContext,
    pub player: *mut Player,
    pub playerPosRot: PosRot,
    pub target: *mut Actor,
    pub targetPosRot: PosRot,
    pub rUpdateRateInv: f32_0,
    pub pitchUpdateRateInv: f32_0,
    pub yawUpdateRateInv: f32_0,
    pub xzOffsetUpdateRate: f32_0,
    pub yOffsetUpdateRate: f32_0,
    pub fovUpdateRate: f32_0,
    pub xzSpeed: f32_0,
    pub dist: f32_0,
    pub speedRatio: f32_0,
    pub posOffset: Vec3f,
    pub playerPosDelta: Vec3f,
    pub fov: f32_0,
    pub atLERPStepScale: f32_0,
    pub playerGroundY: f32_0,
    pub floorNorm: Vec3f,
    pub waterYPos: f32_0,
    pub waterPrevCamIdx: s32,
    pub waterPrevCamSetting: s32,
    pub waterQuakeId: s32,
    pub data0: *mut libc::c_void,
    pub data1: *mut libc::c_void,
    pub data2: s16,
    pub data3: s16,
    pub uid: s16,
    pub unk_132: [libc::c_char; 2],
    pub inputDir: Vec3s,
    pub camDir: Vec3s,
    pub status: s16,
    pub setting: s16,
    pub mode: s16,
    pub bgCheckId: s16,
    pub camDataIdx: s16,
    pub unk_14A: s16,
    pub unk_14C: s16,
    pub childCamIdx: s16,
    pub unk_150: s16,
    pub unk_152: s16,
    pub prevSetting: s16,
    pub nextCamDataIdx: s16,
    pub nextBGCheckId: s16,
    pub roll: s16,
    pub paramFlags: s16,
    pub animState: s16,
    pub timer: s16,
    pub parentCamIdx: s16,
    pub thisIdx: s16,
    pub prevCamDataIdx: s16,
    pub csId: s16,
    pub unk_16A: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub gfxCtx: *mut GraphicsContext,
    pub main: GameStateFunc,
    pub destroy: GameStateFunc,
    pub init: GameStateFunc,
    pub size: u32_0,
    pub input: [Input; 4],
    pub tha: TwoHeadArena,
    pub alloc: GameAlloc,
    pub running: u32_0,
    pub frames: u32_0,
    pub unk_A0: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAlloc {
    pub base: GameAllocEntry,
    pub head: *mut GameAllocEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAllocEntry {
    pub next: *mut GameAllocEntry,
    pub prev: *mut GameAllocEntry,
    pub size: u32_0,
    pub unk_0C: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TwoHeadArena {
    pub size: u32_0,
    pub bufp: *mut libc::c_void,
    pub head: *mut libc::c_void,
    pub tail: *mut libc::c_void,
}
pub type GameStateFunc
    =
    Option<unsafe extern "C" fn(_: *mut GameState) -> ()>;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_15 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_15 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_15 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_15 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_15 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_15 = 397;
pub const OBJECT_COB: C2RustUnnamed_15 = 396;
pub const OBJECT_COW: C2RustUnnamed_15 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_15 = 394;
pub const OBJECT_PS: C2RustUnnamed_15 = 393;
pub const OBJECT_GS: C2RustUnnamed_15 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_15 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_15 = 390;
pub const OBJECT_GJ: C2RustUnnamed_15 = 389;
pub const OBJECT_SKB: C2RustUnnamed_15 = 388;
pub const OBJECT_WF: C2RustUnnamed_15 = 387;
pub const OBJECT_MU: C2RustUnnamed_15 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_15 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_15 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_15 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_15 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_15 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_15 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_15 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_15 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_15 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_15 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_15 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_15 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_15 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_15 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_15 = 371;
pub const OBJECT_DNK: C2RustUnnamed_15 = 370;
pub const OBJECT_DNS: C2RustUnnamed_15 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_15 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_15 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_15 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_15 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_15 = 364;
pub const OBJECT_DOG: C2RustUnnamed_15 = 363;
pub const OBJECT_GR: C2RustUnnamed_15 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_15 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_15 = 360;
pub const OBJECT_GLA: C2RustUnnamed_15 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_15 = 358;
pub const OBJECT_RS: C2RustUnnamed_15 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_15 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_15 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_15 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_15 = 353;
pub const OBJECT_BG: C2RustUnnamed_15 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_15 = 351;
pub const OBJECT_SSH: C2RustUnnamed_15 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_15 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_15 = 348;
pub const OBJECT_FISH: C2RustUnnamed_15 = 347;
pub const OBJECT_EC: C2RustUnnamed_15 = 346;
pub const OBJECT_DS2: C2RustUnnamed_15 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_15 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_15 = 343;
pub const OBJECT_ZG: C2RustUnnamed_15 = 342;
pub const OBJECT_TS: C2RustUnnamed_15 = 341;
pub const OBJECT_KA: C2RustUnnamed_15 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_15 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_15 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_15 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_15 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_15 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_15 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_15 = 333;
pub const OBJECT_GT: C2RustUnnamed_15 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_15 = 331;
pub const OBJECT_MAG: C2RustUnnamed_15 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_15 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_15 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_15 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_15 = 326;
pub const OBJECT_CS: C2RustUnnamed_15 = 325;
pub const OBJECT_JS: C2RustUnnamed_15 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_15 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_15 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_15 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_15 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_15 = 319;
pub const OBJECT_OS: C2RustUnnamed_15 = 318;
pub const OBJECT_FA: C2RustUnnamed_15 = 317;
pub const OBJECT_MM: C2RustUnnamed_15 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_15 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_15 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_15 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_15 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_15 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_15 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_15 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_15 = 308;
pub const OBJECT_FU: C2RustUnnamed_15 = 307;
pub const OBJECT_MK: C2RustUnnamed_15 = 306;
pub const OBJECT_OWL: C2RustUnnamed_15 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_15 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_15 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_15 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_15 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_15 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_15 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_15 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_15 = 297;
pub const OBJECT_HS: C2RustUnnamed_15 = 296;
pub const OBJECT_MS: C2RustUnnamed_15 = 295;
pub const OBJECT_GM: C2RustUnnamed_15 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_15 = 293;
pub const OBJECT_NWC: C2RustUnnamed_15 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_15 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_15 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_15 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_15 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_15 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_15 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_15 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_15 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_15 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_15 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_15 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_15 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_15 = 279;
pub const OBJECT_GE1: C2RustUnnamed_15 = 278;
pub const OBJECT_BOB: C2RustUnnamed_15 = 277;
pub const OBJECT_FZ: C2RustUnnamed_15 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_15 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_15 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_15 = 273;
pub const OBJECT_ANE: C2RustUnnamed_15 = 272;
pub const OBJECT_DS: C2RustUnnamed_15 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_15 = 270;
pub const OBJECT_BBA: C2RustUnnamed_15 = 269;
pub const OBJECT_BJI: C2RustUnnamed_15 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_15 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_15 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_15 = 265;
pub const OBJECT_CNE: C2RustUnnamed_15 = 264;
pub const OBJECT_AHG: C2RustUnnamed_15 = 263;
pub const OBJECT_IK: C2RustUnnamed_15 = 262;
pub const OBJECT_AOB: C2RustUnnamed_15 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_15 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_15 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_15 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_15 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_15 = 256;
pub const OBJECT_KZ: C2RustUnnamed_15 = 255;
pub const OBJECT_ZO: C2RustUnnamed_15 = 254;
pub const OBJECT_KW1: C2RustUnnamed_15 = 253;
pub const OBJECT_KM1: C2RustUnnamed_15 = 252;
pub const OBJECT_MD: C2RustUnnamed_15 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_15 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_15 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_15 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_15 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_15 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_15 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_15 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_15 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_15 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_15 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_15 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_15 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_15 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_15 = 237;
pub const OBJECT_ANI: C2RustUnnamed_15 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_15 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_15 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_15 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_15 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_15 = 231;
pub const OBJECT_FR: C2RustUnnamed_15 = 230;
pub const OBJECT_NY: C2RustUnnamed_15 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_15 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_15 = 227;
pub const OBJECT_SST: C2RustUnnamed_15 = 226;
pub const OBJECT_GANON: C2RustUnnamed_15 = 225;
pub const OBJECT_MA1: C2RustUnnamed_15 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_15 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_15 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_15 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_15 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_15 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_15 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_15 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_15 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_15 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_15 = 214;
pub const OBJECT_BXA: C2RustUnnamed_15 = 213;
pub const OBJECT_RR: C2RustUnnamed_15 = 212;
pub const OBJECT_TW: C2RustUnnamed_15 = 211;
pub const OBJECT_HNI: C2RustUnnamed_15 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_15 = 209;
pub const OBJECT_MA2: C2RustUnnamed_15 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_15 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_15 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_15 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_15 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_15 = 203;
pub const OBJECT_RU2: C2RustUnnamed_15 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_15 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_15 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_15 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_15 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_15 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_15 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_15 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_15 = 194;
pub const OBJECT_TR: C2RustUnnamed_15 = 193;
pub const OBJECT_IN: C2RustUnnamed_15 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_15 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_15 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_15 = 189;
pub const OBJECT_SA: C2RustUnnamed_15 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_15 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_15 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_15 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_15 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_15 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_15 = 182;
pub const OBJECT_SB: C2RustUnnamed_15 = 181;
pub const OBJECT_MO: C2RustUnnamed_15 = 180;
pub const OBJECT_NB: C2RustUnnamed_15 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_15 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_15 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_15 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_15 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_15 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_15 = 173;
pub const OBJECT_BROB: C2RustUnnamed_15 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_15 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_15 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_15 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_15 = 168;
pub const OBJECT_RL: C2RustUnnamed_15 = 167;
pub const OBJECT_DH: C2RustUnnamed_15 = 166;
pub const OBJECT_FD2: C2RustUnnamed_15 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_15 = 164;
pub const OBJECT_RU1: C2RustUnnamed_15 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_15 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_15 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_15 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_15 = 159;
pub const OBJECT_FW: C2RustUnnamed_15 = 158;
pub const OBJECT_DU: C2RustUnnamed_15 = 157;
pub const OBJECT_FD: C2RustUnnamed_15 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_15 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_15 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_15 = 153;
pub const OBJECT_RD: C2RustUnnamed_15 = 152;
pub const OBJECT_SD: C2RustUnnamed_15 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_15 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_15 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_15 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_15 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_15 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_15 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_15 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_15 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_15 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_15 = 141;
pub const OBJECT_BV: C2RustUnnamed_15 = 140;
pub const OBJECT_VM: C2RustUnnamed_15 = 139;
pub const OBJECT_XC: C2RustUnnamed_15 = 138;
pub const OBJECT_TK: C2RustUnnamed_15 = 137;
pub const OBJECT_TA: C2RustUnnamed_15 = 136;
pub const OBJECT_IM: C2RustUnnamed_15 = 135;
pub const OBJECT_VASE: C2RustUnnamed_15 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_15 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_15 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_15 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_15 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_15 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_15 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_15 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_15 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_15 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_15 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_15 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_15 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_15 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_15 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_15 = 119;
pub const OBJECT_HATA: C2RustUnnamed_15 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_15 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_15 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_15 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_15 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_15 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_15 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_15 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_15 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_15 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_15 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_15 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_15 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_15 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_15 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_15 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_15 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_15 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_15 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_15 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_15 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_15 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_15 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_15 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_15 = 94;
pub const OBJECT_BB: C2RustUnnamed_15 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_15 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_15 = 91;
pub const OBJECT_FHG: C2RustUnnamed_15 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_15 = 89;
pub const OBJECT_OA11: C2RustUnnamed_15 = 88;
pub const OBJECT_OA10: C2RustUnnamed_15 = 87;
pub const OBJECT_VALI: C2RustUnnamed_15 = 86;
pub const OBJECT_OE12: C2RustUnnamed_15 = 85;
pub const OBJECT_OE11: C2RustUnnamed_15 = 84;
pub const OBJECT_OE10: C2RustUnnamed_15 = 83;
pub const OBJECT_OE9: C2RustUnnamed_15 = 82;
pub const OBJECT_OE8: C2RustUnnamed_15 = 81;
pub const OBJECT_OE7: C2RustUnnamed_15 = 80;
pub const OBJECT_OE6: C2RustUnnamed_15 = 79;
pub const OBJECT_OE5: C2RustUnnamed_15 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_15 = 77;
pub const OBJECT_OE4: C2RustUnnamed_15 = 76;
pub const OBJECT_OE3: C2RustUnnamed_15 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_15 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_15 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_15 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_15 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_15 = 70;
pub const OBJECT_OB4: C2RustUnnamed_15 = 69;
pub const OBJECT_OB3: C2RustUnnamed_15 = 68;
pub const OBJECT_OB2: C2RustUnnamed_15 = 67;
pub const OBJECT_OA9: C2RustUnnamed_15 = 66;
pub const OBJECT_OA8: C2RustUnnamed_15 = 65;
pub const OBJECT_JJ: C2RustUnnamed_15 = 64;
pub const OBJECT_OA7: C2RustUnnamed_15 = 63;
pub const OBJECT_OA6: C2RustUnnamed_15 = 62;
pub const OBJECT_OA5: C2RustUnnamed_15 = 61;
pub const OBJECT_OA4: C2RustUnnamed_15 = 60;
pub const OBJECT_OA3: C2RustUnnamed_15 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_15 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_15 = 57;
pub const OBJECT_AM: C2RustUnnamed_15 = 56;
pub const OBJECT_GND: C2RustUnnamed_15 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_15 = 54;
pub const OBJECT_OE2: C2RustUnnamed_15 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_15 = 52;
pub const OBJECT_OE1: C2RustUnnamed_15 = 51;
pub const OBJECT_SK2: C2RustUnnamed_15 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_15 = 49;
pub const OBJECT_MB: C2RustUnnamed_15 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_15 = 47;
pub const OBJECT_OA2: C2RustUnnamed_15 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_15 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_15 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_15 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_15 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_15 = 41;
pub const OBJECT_OB1: C2RustUnnamed_15 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_15 = 39;
pub const OBJECT_EI: C2RustUnnamed_15 = 38;
pub const OBJECT_BW: C2RustUnnamed_15 = 37;
pub const OBJECT_ST: C2RustUnnamed_15 = 36;
pub const OBJECT_OA1: C2RustUnnamed_15 = 35;
pub const OBJECT_TP: C2RustUnnamed_15 = 34;
pub const OBJECT_BL: C2RustUnnamed_15 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_15 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_15 = 31;
pub const OBJECT_GOL: C2RustUnnamed_15 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_15 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_15 = 28;
pub const OBJECT_ZF: C2RustUnnamed_15 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_15 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_15 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_15 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_15 = 23;
pub const OBJECT_TITE: C2RustUnnamed_15 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_15 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_15 = 20;
pub const OBJECT_NIW: C2RustUnnamed_15 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_15 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_15 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_15 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_15 = 15;
pub const OBJECT_BOX: C2RustUnnamed_15 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_15 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_15 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_15 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_15 = 10;
pub const OBJECT_POH: C2RustUnnamed_15 = 9;
pub const OBJECT_CROW: C2RustUnnamed_15 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_15 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_15 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_15 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_15 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_15 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_15 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_15 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdBase {
    pub code: u8_0,
    pub data1: u8_0,
    pub data2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSpawnList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdActorList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdUnused02 {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdColHeader {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdRoomList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdWindSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub x: u8_0,
    pub y: u8_0,
    pub z: u8_0,
    pub unk_07: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdEntranceList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSpecialFiles {
    pub code: u8_0,
    pub cUpElfMsgNum: u8_0,
    pub keepObjectId: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdRoomBehavior {
    pub code: u8_0,
    pub gpFlag1: u8_0,
    pub gpFlag2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdMesh {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdObjectList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdLightList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdPathList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdTransiActorList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdLightSettingList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdTimeSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub hour: u8_0,
    pub min: u8_0,
    pub unk_06: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSkyboxSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub skyboxId: u8_0,
    pub unk_05: u8_0,
    pub unk_06: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSkyboxDisables {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub unk_04: u8_0,
    pub unk_05: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdEndMarker {
    pub code: u8_0,
    pub data1: u8_0,
    pub data2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdExitList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSoundSettings {
    pub code: u8_0,
    pub specId: u8_0,
    pub pad: [libc::c_char; 4],
    pub natureAmbienceId: u8_0,
    pub seqId: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdEchoSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 5],
    pub echo: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdCutsceneData {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdAltHeaders {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdMiscSettings {
    pub code: u8_0,
    pub cameraMovement: u8_0,
    pub area: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SceneCmd {
    pub base: SCmdBase,
    pub spawnList: SCmdSpawnList,
    pub actorList: SCmdActorList,
    pub unused02: SCmdUnused02,
    pub roomList: SCmdRoomList,
    pub entranceList: SCmdEntranceList,
    pub objectList: SCmdObjectList,
    pub lightList: SCmdLightList,
    pub pathList: SCmdPathList,
    pub transiActorList: SCmdTransiActorList,
    pub lightSettingList: SCmdLightSettingList,
    pub exitList: SCmdExitList,
    pub colHeader: SCmdColHeader,
    pub mesh: SCmdMesh,
    pub specialFiles: SCmdSpecialFiles,
    pub cutsceneData: SCmdCutsceneData,
    pub roomBehavior: SCmdRoomBehavior,
    pub windSettings: SCmdWindSettings,
    pub timeSettings: SCmdTimeSettings,
    pub skyboxSettings: SCmdSkyboxSettings,
    pub skyboxDisables: SCmdSkyboxDisables,
    pub endMarker: SCmdEndMarker,
    pub soundSettings: SCmdSoundSettings,
    pub echoSettings: SCmdEchoSettings,
    pub miscSettings: SCmdMiscSettings,
    pub altHeaders: SCmdAltHeaders,
}
pub type C2RustUnnamed_16 = libc::c_uint;
pub const SCENE_ID_MAX: C2RustUnnamed_16 = 110;
pub const SCENE_TESTROOM: C2RustUnnamed_16 = 109;
pub const SCENE_SASATEST: C2RustUnnamed_16 = 108;
pub const SCENE_HAIRAL_NIWA2: C2RustUnnamed_16 = 107;
pub const SCENE_SUTARU: C2RustUnnamed_16 = 106;
pub const SCENE_SYOTES2: C2RustUnnamed_16 = 105;
pub const SCENE_SYOTES: C2RustUnnamed_16 = 104;
pub const SCENE_DEPTH_TEST: C2RustUnnamed_16 = 103;
pub const SCENE_BESITU: C2RustUnnamed_16 = 102;
pub const SCENE_TEST01: C2RustUnnamed_16 = 101;
pub const SCENE_GANON_TOU: C2RustUnnamed_16 = 100;
pub const SCENE_SPOT20: C2RustUnnamed_16 = 99;
pub const SCENE_SPOT18: C2RustUnnamed_16 = 98;
pub const SCENE_SPOT17: C2RustUnnamed_16 = 97;
pub const SCENE_SPOT16: C2RustUnnamed_16 = 96;
pub const SCENE_SPOT15: C2RustUnnamed_16 = 95;
pub const SCENE_SPOT13: C2RustUnnamed_16 = 94;
pub const SCENE_SPOT12: C2RustUnnamed_16 = 93;
pub const SCENE_SPOT11: C2RustUnnamed_16 = 92;
pub const SCENE_SPOT10: C2RustUnnamed_16 = 91;
pub const SCENE_SPOT09: C2RustUnnamed_16 = 90;
pub const SCENE_SPOT08: C2RustUnnamed_16 = 89;
pub const SCENE_SPOT07: C2RustUnnamed_16 = 88;
pub const SCENE_SPOT06: C2RustUnnamed_16 = 87;
pub const SCENE_SPOT05: C2RustUnnamed_16 = 86;
pub const SCENE_SPOT04: C2RustUnnamed_16 = 85;
pub const SCENE_SPOT03: C2RustUnnamed_16 = 84;
pub const SCENE_SPOT02: C2RustUnnamed_16 = 83;
pub const SCENE_SPOT01: C2RustUnnamed_16 = 82;
pub const SCENE_SPOT00: C2RustUnnamed_16 = 81;
pub const SCENE_KINSUTA: C2RustUnnamed_16 = 80;
pub const SCENE_GANON_DEMO: C2RustUnnamed_16 = 79;
pub const SCENE_MAHOUYA: C2RustUnnamed_16 = 78;
pub const SCENE_MIHARIGOYA: C2RustUnnamed_16 = 77;
pub const SCENE_SOUKO: C2RustUnnamed_16 = 76;
pub const SCENE_BOWLING: C2RustUnnamed_16 = 75;
pub const SCENE_NAKANIWA: C2RustUnnamed_16 = 74;
pub const SCENE_TURIBORI: C2RustUnnamed_16 = 73;
pub const SCENE_HAKASITARELAY: C2RustUnnamed_16 = 72;
pub const SCENE_HIRAL_DEMO: C2RustUnnamed_16 = 71;
pub const SCENE_HAIRAL_NIWA_N: C2RustUnnamed_16 = 70;
pub const SCENE_HAIRAL_NIWA: C2RustUnnamed_16 = 69;
pub const SCENE_KENJYANOMA: C2RustUnnamed_16 = 68;
pub const SCENE_TOKINOMA: C2RustUnnamed_16 = 67;
pub const SCENE_SYATEKIJYOU: C2RustUnnamed_16 = 66;
pub const SCENE_HAKAANA_OUKE: C2RustUnnamed_16 = 65;
pub const SCENE_HAKAANA2: C2RustUnnamed_16 = 64;
pub const SCENE_HAKAANA: C2RustUnnamed_16 = 63;
pub const SCENE_KAKUSIANA: C2RustUnnamed_16 = 62;
pub const SCENE_YOUSEI_IZUMI_YOKO: C2RustUnnamed_16 = 61;
pub const SCENE_YOUSEI_IZUMI_TATE: C2RustUnnamed_16 = 60;
pub const SCENE_DAIYOUSEI_IZUMI: C2RustUnnamed_16 = 59;
pub const SCENE_HUT: C2RustUnnamed_16 = 58;
pub const SCENE_TENT: C2RustUnnamed_16 = 57;
pub const SCENE_HYLIA_LABO: C2RustUnnamed_16 = 56;
pub const SCENE_LABO: C2RustUnnamed_16 = 55;
pub const SCENE_MALON_STABLE: C2RustUnnamed_16 = 54;
pub const SCENE_IMPA: C2RustUnnamed_16 = 53;
pub const SCENE_LINK_HOME: C2RustUnnamed_16 = 52;
pub const SCENE_FACE_SHOP: C2RustUnnamed_16 = 51;
pub const SCENE_NIGHT_SHOP: C2RustUnnamed_16 = 50;
pub const SCENE_ALLEY_SHOP: C2RustUnnamed_16 = 49;
pub const SCENE_DRAG: C2RustUnnamed_16 = 48;
pub const SCENE_ZOORA: C2RustUnnamed_16 = 47;
pub const SCENE_GOLON: C2RustUnnamed_16 = 46;
pub const SCENE_KOKIRI_SHOP: C2RustUnnamed_16 = 45;
pub const SCENE_SHOP1: C2RustUnnamed_16 = 44;
pub const SCENE_KAKARIKO3: C2RustUnnamed_16 = 43;
pub const SCENE_KAKARIKO: C2RustUnnamed_16 = 42;
pub const SCENE_KOKIRI_HOME5: C2RustUnnamed_16 = 41;
pub const SCENE_KOKIRI_HOME4: C2RustUnnamed_16 = 40;
pub const SCENE_KOKIRI_HOME3: C2RustUnnamed_16 = 39;
pub const SCENE_KOKIRI_HOME: C2RustUnnamed_16 = 38;
pub const SCENE_SHRINE_R: C2RustUnnamed_16 = 37;
pub const SCENE_SHRINE_N: C2RustUnnamed_16 = 36;
pub const SCENE_SHRINE: C2RustUnnamed_16 = 35;
pub const SCENE_MARKET_RUINS: C2RustUnnamed_16 = 34;
pub const SCENE_MARKET_NIGHT: C2RustUnnamed_16 = 33;
pub const SCENE_MARKET_DAY: C2RustUnnamed_16 = 32;
pub const SCENE_MARKET_ALLEY_N: C2RustUnnamed_16 = 31;
pub const SCENE_MARKET_ALLEY: C2RustUnnamed_16 = 30;
pub const SCENE_ENRUI: C2RustUnnamed_16 = 29;
pub const SCENE_ENTRA_N: C2RustUnnamed_16 = 28;
pub const SCENE_ENTRA: C2RustUnnamed_16 = 27;
pub const SCENE_GANON_FINAL: C2RustUnnamed_16 = 26;
pub const SCENE_GANON_BOSS: C2RustUnnamed_16 = 25;
pub const SCENE_HAKADAN_BS: C2RustUnnamed_16 = 24;
pub const SCENE_JYASINBOSS: C2RustUnnamed_16 = 23;
pub const SCENE_MIZUSIN_BS: C2RustUnnamed_16 = 22;
pub const SCENE_FIRE_BS: C2RustUnnamed_16 = 21;
pub const SCENE_MORIBOSSROOM: C2RustUnnamed_16 = 20;
pub const SCENE_BDAN_BOSS: C2RustUnnamed_16 = 19;
pub const SCENE_DDAN_BOSS: C2RustUnnamed_16 = 18;
pub const SCENE_YDAN_BOSS: C2RustUnnamed_16 = 17;
pub const SCENE_TAKARAYA: C2RustUnnamed_16 = 16;
pub const SCENE_GANONTIKA_SONOGO: C2RustUnnamed_16 = 15;
pub const SCENE_GANON_SONOGO: C2RustUnnamed_16 = 14;
pub const SCENE_GANONTIKA: C2RustUnnamed_16 = 13;
pub const SCENE_GERUDOWAY: C2RustUnnamed_16 = 12;
pub const SCENE_MEN: C2RustUnnamed_16 = 11;
pub const SCENE_GANON: C2RustUnnamed_16 = 10;
pub const SCENE_ICE_DOUKUTO: C2RustUnnamed_16 = 9;
pub const SCENE_HAKADANCH: C2RustUnnamed_16 = 8;
pub const SCENE_HAKADAN: C2RustUnnamed_16 = 7;
pub const SCENE_JYASINZOU: C2RustUnnamed_16 = 6;
pub const SCENE_MIZUSIN: C2RustUnnamed_16 = 5;
pub const SCENE_HIDAN: C2RustUnnamed_16 = 4;
pub const SCENE_BMORI1: C2RustUnnamed_16 = 3;
pub const SCENE_BDAN: C2RustUnnamed_16 = 2;
pub const SCENE_DDAN: C2RustUnnamed_16 = 1;
pub const SCENE_YDAN: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameInfo {
    pub regPage: s32,
    pub regGroup: s32,
    pub regCur: s32,
    pub dpadLast: s32,
    pub repeat: s32,
    pub data: [s16; 2784],
}
#[no_mangle]
pub unsafe extern "C" fn Object_Spawn(mut objectCtx: *mut ObjectContext,
                                      mut objectId: s16) -> s32 {
    let mut size: u32_0 =
        0; // Needs to be a new variable to match (possibly a sub struct?)
    (*objectCtx).status[(*objectCtx).num as usize].id = objectId;
    size =
        gObjectTable[objectId as
                         usize].vromEnd.wrapping_sub(gObjectTable[objectId as
                                                                      usize].vromStart);
    osSyncPrintf(b"OBJECT[%d] SIZE %fK SEG=%x\n\x00" as *const u8 as
                     *const libc::c_char, objectId as libc::c_int,
                 (size as libc::c_float / 1024.0f32) as libc::c_double,
                 (*objectCtx).status[(*objectCtx).num as usize].segment);
    osSyncPrintf(b"num=%d adrs=%x end=%x\n\x00" as *const u8 as
                     *const libc::c_char, (*objectCtx).num as libc::c_int,
                 ((*objectCtx).status[(*objectCtx).num as usize].segment as
                      s32 as libc::c_uint).wrapping_add(size),
                 (*objectCtx).spaceEnd);
    if ((*objectCtx).num as libc::c_int) < 19 as libc::c_int &&
           ((*objectCtx).status[(*objectCtx).num as usize].segment as s32 as
                libc::c_uint).wrapping_add(size) <
               (*objectCtx).spaceEnd as u32_0 {
    } else {
        __assert(b"this->num < OBJECT_EXCHANGE_BANK_MAX && (this->status[this->num].Segment + size) < this->endSegment\x00"
                     as *const u8 as *const libc::c_char,
                 b"../z_scene.c\x00" as *const u8 as *const libc::c_char,
                 142 as libc::c_int);
    };
    DmaMgr_SendRequest1((*objectCtx).status[(*objectCtx).num as
                                                usize].segment,
                        gObjectTable[objectId as usize].vromStart, size,
                        b"../z_scene.c\x00" as *const u8 as
                            *const libc::c_char, 145 as libc::c_int);
    if ((*objectCtx).num as libc::c_int) <
           19 as libc::c_int - 1 as libc::c_int {
        (*objectCtx).status[((*objectCtx).num as libc::c_int +
                                 1 as libc::c_int) as usize].segment =
            (((*objectCtx).status[(*objectCtx).num as usize].segment as s32 as
                  libc::c_uint).wrapping_add(size).wrapping_add(0xf as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                 & !(0xf as libc::c_int) as libc::c_uint) as *mut libc::c_void
    }
    (*objectCtx).num = (*objectCtx).num.wrapping_add(1);
    (*objectCtx).unk_09 = (*objectCtx).num;
    return (*objectCtx).num as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Object_InitBank(mut globalCtx: *mut GlobalContext,
                                         mut objectCtx: *mut ObjectContext) {
    let mut globalCtx2: *mut GlobalContext = globalCtx;
    let mut spaceSize: u32_0 = 0;
    let mut i: s32 = 0;
    if (*globalCtx2).sceneNum as libc::c_int == SCENE_SPOT00 as libc::c_int {
        spaceSize = 1024000 as libc::c_int as u32_0
    } else if (*globalCtx2).sceneNum as libc::c_int ==
                  SCENE_GANON_DEMO as libc::c_int {
        if gSaveContext.sceneSetupIndex != 4 as libc::c_int {
            spaceSize = 1177600 as libc::c_int as u32_0
        } else { spaceSize = 1024000 as libc::c_int as u32_0 }
    } else if (*globalCtx2).sceneNum as libc::c_int ==
                  SCENE_JYASINBOSS as libc::c_int {
        spaceSize = 1075200 as libc::c_int as u32_0
    } else if (*globalCtx2).sceneNum as libc::c_int ==
                  SCENE_KENJYANOMA as libc::c_int {
        spaceSize = 1075200 as libc::c_int as u32_0
    } else if (*globalCtx2).sceneNum as libc::c_int ==
                  SCENE_GANON_BOSS as libc::c_int {
        spaceSize = 1075200 as libc::c_int as u32_0
    } else { spaceSize = 1024000 as libc::c_int as u32_0 }
    (*objectCtx).unk_09 = 0 as libc::c_int as u8_0;
    (*objectCtx).num = (*objectCtx).unk_09;
    (*objectCtx).subKeepIndex = 0 as libc::c_int as u8_0;
    (*objectCtx).mainKeepIndex = (*objectCtx).subKeepIndex;
    i = 0 as libc::c_int;
    while i < 19 as libc::c_int {
        (*objectCtx).status[i as usize].id =
            OBJECT_INVALID as libc::c_int as s16;
        i += 1
    }
    osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
    // "Object exchange bank data %8.3fKB"
    osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\x96\xe3\x82\xb8\xe3\x82\xa7\xe3\x82\xaf\xe3\x83\x88\xe5\x85\xa5\xe3\x82\x8c\xe6\x9b\xbf\xe3\x81\x88\xe3\x83\x90\xe3\x83\xb3\xe3\x82\xaf\xe6\x83\x85\xe5\xa0\xb1 %8.3fKB\n\x00"
                     as *const u8 as *const libc::c_char,
                 (spaceSize as libc::c_float / 1024.0f32) as libc::c_double);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    (*objectCtx).status[0 as libc::c_int as usize].segment =
        GameState_Alloc(&mut (*globalCtx).state, spaceSize as size_t,
                        b"../z_scene.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        219 as libc::c_int);
    (*objectCtx).spaceStart =
        (*objectCtx).status[0 as libc::c_int as usize].segment;
    (*objectCtx).spaceEnd =
        ((*objectCtx).spaceStart as s32 as
             libc::c_uint).wrapping_add(spaceSize) as *mut libc::c_void;
    (*objectCtx).mainKeepIndex =
        Object_Spawn(objectCtx, OBJECT_GAMEPLAY_KEEP as libc::c_int as s16) as
            u8_0;
    gSegments[4 as libc::c_int as usize] =
        ((*objectCtx).status[(*objectCtx).mainKeepIndex as usize].segment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn Object_UpdateBank(mut objectCtx:
                                               *mut ObjectContext) {
    let mut i: s32 = 0;
    let mut status: *mut ObjectStatus =
        &mut *(*objectCtx).status.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
            *mut ObjectStatus;
    let mut objectFile: *mut RomFile = 0 as *mut RomFile;
    let mut size: u32_0 = 0;
    i = 0 as libc::c_int;
    while i < (*objectCtx).num as libc::c_int {
        if ((*status).id as libc::c_int) < 0 as libc::c_int {
            if (*status).dmaRequest.vromAddr ==
                   0 as libc::c_int as libc::c_uint {
                osCreateMesgQueue(&mut (*status).loadQueue,
                                  &mut (*status).loadMsg, 1 as libc::c_int);
                objectFile =
                    &mut *gObjectTable.as_mut_ptr().offset(-((*status).id as
                                                                 libc::c_int)
                                                               as isize) as
                        *mut RomFile;
                size =
                    (*objectFile).vromEnd.wrapping_sub((*objectFile).vromStart);
                osSyncPrintf(b"OBJECT EXCHANGE BANK-%2d SIZE %8.3fK SEG=%08x\n\x00"
                                 as *const u8 as *const libc::c_char, i,
                             (size as libc::c_float / 1024.0f32) as
                                 libc::c_double, (*status).segment);
                DmaMgr_SendRequest2(&mut (*status).dmaRequest,
                                    (*status).segment as u32_0,
                                    (*objectFile).vromStart, size,
                                    0 as libc::c_int as u32_0,
                                    &mut (*status).loadQueue,
                                    0 as *mut libc::c_void,
                                    b"../z_scene.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    266 as libc::c_int);
            } else if osRecvMesg(&mut (*status).loadQueue, 0 as *mut OSMesg,
                                 0 as libc::c_int) == 0 {
                (*status).id = -((*status).id as libc::c_int) as s16
            }
        }
        status = status.offset(1);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Object_GetIndex(mut objectCtx: *mut ObjectContext,
                                         mut objectId: s16) -> s32 {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*objectCtx).num as libc::c_int {
        if (if (*objectCtx).status[i as usize].id as libc::c_int >=
                   0 as libc::c_int {
                (*objectCtx).status[i as usize].id as libc::c_int
            } else { -((*objectCtx).status[i as usize].id as libc::c_int) })
               == objectId as libc::c_int {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Object_IsLoaded(mut objectCtx: *mut ObjectContext,
                                         mut bankIndex: s32) -> s32 {
    if (*objectCtx).status[bankIndex as usize].id as libc::c_int >
           0 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn func_800981B8(mut objectCtx: *mut ObjectContext) {
    let mut i: s32 = 0;
    let mut id: s32 = 0;
    let mut size: u32_0 = 0;
    i = 0 as libc::c_int;
    while i < (*objectCtx).num as libc::c_int {
        id = (*objectCtx).status[i as usize].id as s32;
        size =
            gObjectTable[id as
                             usize].vromEnd.wrapping_sub(gObjectTable[id as
                                                                          usize].vromStart);
        osSyncPrintf(b"OBJECT[%d] SIZE %fK SEG=%x\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*objectCtx).status[i as usize].id as libc::c_int,
                     (size as libc::c_float / 1024.0f32) as libc::c_double,
                     (*objectCtx).status[i as usize].segment);
        osSyncPrintf(b"num=%d adrs=%x end=%x\n\x00" as *const u8 as
                         *const libc::c_char, (*objectCtx).num as libc::c_int,
                     ((*objectCtx).status[i as usize].segment as s32 as
                          libc::c_uint).wrapping_add(size),
                     (*objectCtx).spaceEnd);
        DmaMgr_SendRequest1((*objectCtx).status[i as usize].segment,
                            gObjectTable[id as usize].vromStart, size,
                            b"../z_scene.c\x00" as *const u8 as
                                *const libc::c_char, 342 as libc::c_int);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800982FC(mut objectCtx: *mut ObjectContext,
                                       mut bankIndex: s32, mut objectId: s16)
 -> *mut libc::c_void {
    let mut status: *mut ObjectStatus =
        &mut *(*objectCtx).status.as_mut_ptr().offset(bankIndex as isize) as
            *mut ObjectStatus;
    let mut objectFile: *mut RomFile =
        &mut *gObjectTable.as_mut_ptr().offset(objectId as isize) as
            *mut RomFile;
    let mut size: u32_0 = 0;
    let mut nextPtr: *mut libc::c_void = 0 as *mut libc::c_void;
    (*status).id = -(objectId as libc::c_int) as s16;
    (*status).dmaRequest.vromAddr = 0 as libc::c_int as u32_0;
    size = (*objectFile).vromEnd.wrapping_sub((*objectFile).vromStart);
    osSyncPrintf(b"OBJECT EXCHANGE NO=%2d BANK=%3d SIZE=%8.3fK\n\x00" as
                     *const u8 as *const libc::c_char, bankIndex,
                 objectId as libc::c_int,
                 (size as libc::c_float / 1024.0f32) as libc::c_double);
    nextPtr =
        (((*status).segment as s32 as
              libc::c_uint).wrapping_add(size).wrapping_add(0xf as libc::c_int
                                                                as
                                                                libc::c_uint)
             & !(0xf as libc::c_int) as libc::c_uint) as *mut libc::c_void;
    // Necessary to match
    if nextPtr < (*objectCtx).spaceEnd {
    } else {
        __assert(b"nextptr < this->endSegment\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_scene.c\x00" as *const u8 as *const libc::c_char,
                 381 as libc::c_int);
    };
    // "Object exchange free size=%08x"
    osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\x96\xe3\x82\xb8\xe3\x82\xa7\xe3\x82\xaf\xe3\x83\x88\xe5\x85\xa5\xe3\x82\x8c\xe6\x9b\xbf\xe3\x81\x88\xe7\xa9\xba\xe3\x81\x8d\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba=%08x\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*objectCtx).spaceEnd as s32 -
                     nextPtr as s32); // "code variable is abnormal"
    return nextPtr;
}
#[no_mangle]
pub unsafe extern "C" fn Scene_ExecuteCommands(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut sceneCmd: *mut SceneCmd)
 -> s32 {
    let mut cmdCode: u32_0 = 0;
    loop  {
        cmdCode = (*sceneCmd).base.code as u32_0;
        osSyncPrintf(b"*** Scene_Word = { code=%d, data1=%02x, data2=%04x } ***\n\x00"
                         as *const u8 as *const libc::c_char, cmdCode,
                     (*sceneCmd).base.data1 as libc::c_int,
                     (*sceneCmd).base.data2);
        if cmdCode == 0x14 as libc::c_int as libc::c_uint { break ; }
        if cmdCode <= 0x19 as libc::c_int as libc::c_uint {
            gSceneCmdHandlers[cmdCode as
                                  usize].expect("non-null function pointer")(globalCtx,
                                                                             sceneCmd);
        } else {
            osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
            osSyncPrintf(b"code \xe3\x81\xae\xe5\x80\xa4\xe3\x81\x8c\xe7\x95\xb0\xe5\xb8\xb8\xe3\x81\xa7\xe3\x81\x99\n\x00"
                             as *const u8 as *const libc::c_char);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        }
        sceneCmd = sceneCmd.offset(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80098508(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).linkActorEntry =
        (gSegments[(((*cmd).spawnList.segment as u32_0) << 4 as libc::c_int >>
                        28 as libc::c_int) as
                       usize].wrapping_add((*cmd).spawnList.segment as u32_0 &
                                               0xffffff as libc::c_int as
                                                   libc::c_uint).wrapping_add(0x80000000
                                                                                  as
                                                                                  libc::c_uint)
             as *mut libc::c_void as
             *mut ActorEntry).offset((*(*globalCtx).setupEntranceList.offset((*globalCtx).curSpawn
                                                                                 as
                                                                                 isize)).spawn
                                         as libc::c_int as isize);
    let mut linkEntry: *mut ActorEntry = (*globalCtx).linkActorEntry;
    let mut linkObjectId: s16 = 0;
    (*globalCtx).linkAgeOnLoad = gSaveContext.linkAge as u8_0;
    linkObjectId = gLinkObjectIds[gSaveContext.linkAge as usize];
    (*gActorOverlayTable[(*linkEntry).id as usize].initInfo).objectId =
        linkObjectId;
    Object_Spawn(&mut (*globalCtx).objectCtx, linkObjectId);
}
// Scene Command 0x01: Actor List
#[no_mangle]
pub unsafe extern "C" fn func_800985DC(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).numSetupActors = (*cmd).actorList.num;
    (*globalCtx).setupActorList =
        gSegments[(((*cmd).actorList.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).actorList.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut ActorEntry;
}
// Scene Command 0x02: Unused 02
#[no_mangle]
pub unsafe extern "C" fn func_80098630(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).unk_11DFC =
        gSegments[(((*cmd).unused02.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).unused02.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void;
}
// Scene Command 0x03: Collision Header
#[no_mangle]
pub unsafe extern "C" fn func_80098674(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    let mut colHeader: *mut CollisionHeader =
        gSegments[(((*cmd).colHeader.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).colHeader.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut CollisionHeader;
    (*colHeader).vtxList =
        gSegments[(((*colHeader).vtxList as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).vtxList as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut Vec3s;
    (*colHeader).polyList =
        gSegments[(((*colHeader).polyList as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).polyList as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut CollisionPoly;
    (*colHeader).surfaceTypeList =
        gSegments[(((*colHeader).surfaceTypeList as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).surfaceTypeList as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut SurfaceType;
    (*colHeader).cameraDataList =
        gSegments[(((*colHeader).cameraDataList as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).cameraDataList as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut CamData;
    (*colHeader).waterBoxes =
        gSegments[(((*colHeader).waterBoxes as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).waterBoxes as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut WaterBox;
    BgCheck_Allocate(&mut (*globalCtx).colCtx, globalCtx, colHeader);
}
// Scene Command 0x04: Room List
#[no_mangle]
pub unsafe extern "C" fn func_800987A4(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).numRooms = (*cmd).roomList.num;
    (*globalCtx).roomList =
        gSegments[(((*cmd).roomList.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).roomList.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut RomFile;
}
// Scene Command 0x06: Entrance List
#[no_mangle]
pub unsafe extern "C" fn func_800987F8(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).setupEntranceList =
        gSegments[(((*cmd).entranceList.segment as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*cmd).entranceList.segment as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut EntranceEntry;
}
// Scene Command 0x07: Special Files
#[no_mangle]
pub unsafe extern "C" fn func_8009883C(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    if (*cmd).specialFiles.keepObjectId != 0 as libc::c_int as libc::c_uint {
        (*globalCtx).objectCtx.subKeepIndex =
            Object_Spawn(&mut (*globalCtx).objectCtx,
                         (*cmd).specialFiles.keepObjectId as s16) as u8_0;
        gSegments[5 as libc::c_int as usize] =
            ((*globalCtx).objectCtx.status[(*globalCtx).objectCtx.subKeepIndex
                                               as usize].segment as
                 *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
                u32_0
    }
    if (*cmd).specialFiles.cUpElfMsgNum as libc::c_int != 0 as libc::c_int {
        (*globalCtx).cUpElfMsgs =
            Gameplay_LoadFile(globalCtx,
                              &mut *sNaviMsgFiles.as_mut_ptr().offset(((*cmd).specialFiles.cUpElfMsgNum
                                                                           as
                                                                           libc::c_int
                                                                           -
                                                                           1
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          isize))
                as *mut ElfMessage
    };
}
// Scene Command 0x08: Room Behavior
#[no_mangle]
pub unsafe extern "C" fn func_80098904(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).roomCtx.curRoom.unk_03 = (*cmd).roomBehavior.gpFlag1;
    (*globalCtx).roomCtx.curRoom.unk_02 =
        ((*cmd).roomBehavior.gpFlag2 & 0xff as libc::c_int as libc::c_uint) as
            u8_0;
    (*globalCtx).roomCtx.curRoom.showInvisActors =
        ((*cmd).roomBehavior.gpFlag2 >> 8 as libc::c_int &
             1 as libc::c_int as libc::c_uint) as u8_0;
    (*globalCtx).msgCtx.disableWarpSongs =
        ((*cmd).roomBehavior.gpFlag2 >> 0xa as libc::c_int &
             1 as libc::c_int as libc::c_uint) as s16;
}
// Scene Command 0x0A: Mesh Header
#[no_mangle]
pub unsafe extern "C" fn func_80098958(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).roomCtx.curRoom.mesh =
        gSegments[(((*cmd).mesh.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).mesh.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut Mesh;
}
// Scene Command 0x0B: Object List
#[no_mangle]
pub unsafe extern "C" fn func_8009899C(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut k: s32 = 0;
    let mut status: *mut ObjectStatus = 0 as *mut ObjectStatus;
    let mut status2: *mut ObjectStatus = 0 as *mut ObjectStatus;
    let mut firstStatus: *mut ObjectStatus = 0 as *mut ObjectStatus;
    let mut objectEntry: *mut s16 =
        gSegments[(((*cmd).objectList.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).objectList.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut s16;
    let mut nextPtr: *mut libc::c_void = 0 as *mut libc::c_void;
    k = 0 as libc::c_int;
    i = (*globalCtx).objectCtx.unk_09 as s32;
    firstStatus =
        &mut *(*globalCtx).objectCtx.status.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
            as *mut ObjectStatus;
    status =
        &mut *(*globalCtx).objectCtx.status.as_mut_ptr().offset(i as isize) as
            *mut ObjectStatus;
    while i < (*globalCtx).objectCtx.num as libc::c_int {
        if (*status).id as libc::c_int != *objectEntry as libc::c_int {
            status2 =
                &mut *(*globalCtx).objectCtx.status.as_mut_ptr().offset(i as
                                                                            isize)
                    as *mut ObjectStatus;
            j = i;
            while j < (*globalCtx).objectCtx.num as libc::c_int {
                (*status2).id = OBJECT_INVALID as libc::c_int as s16;
                status2 = status2.offset(1);
                j += 1
            }
            (*globalCtx).objectCtx.num = i as u8_0;
            func_80031A28(globalCtx, &mut (*globalCtx).actorCtx);
        } else {
            i += 1;
            k += 1;
            objectEntry = objectEntry.offset(1);
            status = status.offset(1)
        }
    }
    if (*cmd).objectList.num as libc::c_int <= 19 as libc::c_int {
    } else {
        __assert(b"scene_info->object_bank.num <= OBJECT_EXCHANGE_BANK_MAX\x00"
                     as *const u8 as *const libc::c_char,
                 b"../z_scene.c\x00" as *const u8 as *const libc::c_char,
                 705 as libc::c_int);
    };
    while k < (*cmd).objectList.num as libc::c_int {
        nextPtr = func_800982FC(&mut (*globalCtx).objectCtx, i, *objectEntry);
        if i < 19 as libc::c_int - 1 as libc::c_int {
            let ref mut fresh0 =
                (*firstStatus.offset((i + 1 as libc::c_int) as
                                         isize)).segment;
            *fresh0 = nextPtr
        }
        i += 1;
        k += 1;
        objectEntry = objectEntry.offset(1)
    }
    (*globalCtx).objectCtx.num = i as u8_0;
}
// Scene Command 0x0C: Light List
#[no_mangle]
pub unsafe extern "C" fn func_80098B74(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    let mut i: s32 = 0;
    let mut lightInfo: *mut LightInfo =
        gSegments[(((*cmd).lightList.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).lightList.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LightInfo;
    i = 0 as libc::c_int;
    while i < (*cmd).lightList.num as libc::c_int {
        LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 lightInfo);
        lightInfo = lightInfo.offset(1);
        i += 1
    };
}
// Scene Command 0x0D: Path List
#[no_mangle]
pub unsafe extern "C" fn func_80098C24(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).setupPathList =
        gSegments[(((*cmd).pathList.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).pathList.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut Path;
}
// Scene Command 0x0E: Transition Actor List
#[no_mangle]
pub unsafe extern "C" fn func_80098C68(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).transiActorCtx.numActors = (*cmd).transiActorList.num;
    (*globalCtx).transiActorCtx.list =
        gSegments[(((*cmd).transiActorList.segment as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add((*cmd).transiActorList.segment as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut TransitionActorEntry;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionActor_InitContext(mut state:
                                                         *mut GameState,
                                                     mut transiActorCtx:
                                                         *mut TransitionActorContext) {
    (*transiActorCtx).numActors = 0 as libc::c_int as u8_0;
}
// Scene Command 0x0F: Light Setting List
#[no_mangle]
pub unsafe extern "C" fn func_80098CC8(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).envCtx.numLightSettings = (*cmd).lightSettingList.num;
    (*globalCtx).envCtx.lightSettingsList =
        gSegments[(((*cmd).lightSettingList.segment as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add((*cmd).lightSettingList.segment as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut EnvLightSettings;
}
// Scene Command 0x11: Skybox Settings
#[no_mangle]
pub unsafe extern "C" fn func_80098D1C(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).skyboxId = (*cmd).skyboxSettings.skyboxId;
    (*globalCtx).envCtx.unk_18 = (*cmd).skyboxSettings.unk_05;
    (*globalCtx).envCtx.unk_17 = (*globalCtx).envCtx.unk_18;
    (*globalCtx).envCtx.indoors = (*cmd).skyboxSettings.unk_06;
}
// Scene Command 0x12: Skybox Disables
#[no_mangle]
pub unsafe extern "C" fn func_80098D5C(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).envCtx.skyboxDisabled = (*cmd).skyboxDisables.unk_04;
    (*globalCtx).envCtx.sunMoonDisabled = (*cmd).skyboxDisables.unk_05;
}
// Scene Command 0x10: Time Settings
#[no_mangle]
pub unsafe extern "C" fn func_80098D80(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    if (*cmd).timeSettings.hour as libc::c_int != 0xff as libc::c_int &&
           (*cmd).timeSettings.min as libc::c_int != 0xff as libc::c_int {
        gSaveContext.dayTime =
            (((*cmd).timeSettings.hour as libc::c_int as libc::c_float +
                  (*cmd).timeSettings.min as libc::c_int as libc::c_float /
                      60.0f32) * 60.0f32 /
                 ((24 as libc::c_int * 60 as libc::c_int) as f32_0 /
                      0x10000 as libc::c_int as libc::c_float)) as u16_0;
        gSaveContext.skyboxTime = gSaveContext.dayTime
    }
    if (*cmd).timeSettings.unk_06 as libc::c_int != 0xff as libc::c_int {
        (*globalCtx).envCtx.timeIncrement =
            (*cmd).timeSettings.unk_06 as u16_0
    } else { (*globalCtx).envCtx.timeIncrement = 0 as libc::c_int as u16_0 }
    if gSaveContext.sunsSongState as libc::c_int ==
           SUNSSONG_INACTIVE as libc::c_int {
        gTimeIncrement = (*globalCtx).envCtx.timeIncrement
    }
    (*globalCtx).envCtx.sunPos.x =
        -(Math_SinS((gSaveContext.dayTime as libc::c_int -
                         0x8000 as libc::c_int) as s16) * 120.0f32) * 25.0f32;
    (*globalCtx).envCtx.sunPos.y =
        Math_CosS((gSaveContext.dayTime as libc::c_int -
                       0x8000 as libc::c_int) as s16) * 120.0f32 * 25.0f32;
    (*globalCtx).envCtx.sunPos.z =
        Math_CosS((gSaveContext.dayTime as libc::c_int -
                       0x8000 as libc::c_int) as s16) * 20.0f32 * 25.0f32;
    if (*globalCtx).envCtx.timeIncrement as libc::c_int == 0 as libc::c_int &&
           gSaveContext.cutsceneIndex < 0xfff0 as libc::c_int ||
           gSaveContext.entranceIndex == 0x604 as libc::c_int {
        gSaveContext.skyboxTime = gSaveContext.dayTime;
        if gSaveContext.skyboxTime as libc::c_int >= 0x2aac as libc::c_int &&
               (gSaveContext.skyboxTime as libc::c_int) <
                   0x4555 as libc::c_int {
            gSaveContext.skyboxTime = 0x3556 as libc::c_int as u16_0
        } else if gSaveContext.skyboxTime as libc::c_int >=
                      0x4555 as libc::c_int &&
                      (gSaveContext.skyboxTime as libc::c_int) <
                          0x5556 as libc::c_int {
            gSaveContext.skyboxTime = 0x5556 as libc::c_int as u16_0
        } else if gSaveContext.skyboxTime as libc::c_int >=
                      0xaaab as libc::c_int &&
                      (gSaveContext.skyboxTime as libc::c_int) <
                          0xb556 as libc::c_int {
            gSaveContext.skyboxTime = 0xb556 as libc::c_int as u16_0
        } else if gSaveContext.skyboxTime as libc::c_int >=
                      0xc001 as libc::c_int &&
                      (gSaveContext.skyboxTime as libc::c_int) <
                          0xcaac as libc::c_int {
            gSaveContext.skyboxTime = 0xcaac as libc::c_int as u16_0
        }
    };
}
// Scene Command 0x05: Wind Settings
#[no_mangle]
pub unsafe extern "C" fn func_80099090(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    let mut x: s8 = (*cmd).windSettings.x as s8;
    let mut y: s8 = (*cmd).windSettings.y as s8;
    let mut z: s8 = (*cmd).windSettings.z as s8;
    (*globalCtx).envCtx.windDirection.x = x as s16;
    (*globalCtx).envCtx.windDirection.y = y as s16;
    (*globalCtx).envCtx.windDirection.z = z as s16;
    (*globalCtx).envCtx.windSpeed = (*cmd).windSettings.unk_07 as f32_0;
}
// Scene Command 0x13: Exit List
#[no_mangle]
pub unsafe extern "C" fn func_800990F0(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).setupExitList =
        gSegments[(((*cmd).exitList.segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*cmd).exitList.segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut s16;
}
// Scene Command 0x09: Undefined
#[no_mangle]
pub unsafe extern "C" fn func_80099134(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
}
// Scene Command 0x15: Sound Settings
#[no_mangle]
pub unsafe extern "C" fn func_80099140(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).sequenceCtx.seqId = (*cmd).soundSettings.seqId;
    (*globalCtx).sequenceCtx.natureAmbienceId =
        (*cmd).soundSettings.natureAmbienceId;
    if gSaveContext.seqId as libc::c_int ==
           0xffff as libc::c_int as u8_0 as libc::c_int {
        Audio_QueueSeqCmd((*cmd).soundSettings.specId as libc::c_uint |
                              0xf0000000 as libc::c_uint);
    };
}
// Scene Command 0x16: Echo Setting
#[no_mangle]
pub unsafe extern "C" fn func_8009918C(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*globalCtx).roomCtx.curRoom.echo = (*cmd).echoSettings.echo as s8;
}
// Scene Command 0x18: Alternate Headers
#[no_mangle]
pub unsafe extern "C" fn func_800991A0(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    let mut pad: s32 = 0;
    let mut altHeader: *mut SceneCmd = 0 as *mut SceneCmd;
    osSyncPrintf(b"\n[ZU]sceneset age    =[%X]\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.linkAge);
    osSyncPrintf(b"\n[ZU]sceneset time   =[%X]\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.cutsceneIndex);
    osSyncPrintf(b"\n[ZU]sceneset counter=[%X]\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.sceneSetupIndex);
    if gSaveContext.sceneSetupIndex != 0 as libc::c_int {
        altHeader =
            *(gSegments[(((*cmd).altHeaders.segment as u32_0) <<
                             4 as libc::c_int >> 28 as libc::c_int) as
                            usize].wrapping_add((*cmd).altHeaders.segment as
                                                    u32_0 &
                                                    0xffffff as libc::c_int as
                                                        libc::c_uint).wrapping_add(0x80000000
                                                                                       as
                                                                                       libc::c_uint)
                  as *mut libc::c_void as
                  *mut *mut SceneCmd).offset((gSaveContext.sceneSetupIndex -
                                                  1 as libc::c_int) as isize);
        if !altHeader.is_null() {
            Scene_ExecuteCommands(globalCtx,
                                  gSegments[((altHeader as u32_0) <<
                                                 4 as libc::c_int >>
                                                 28 as libc::c_int) as
                                                usize].wrapping_add(altHeader
                                                                        as
                                                                        u32_0
                                                                        &
                                                                        0xffffff
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                                           as
                                                                                                           libc::c_uint)
                                      as *mut libc::c_void as *mut SceneCmd);
            (*cmd.offset(1 as libc::c_int as isize)).base.code =
                0x14 as libc::c_int as u8_0
        } else {
            // "Coughh! There is no specified dataaaaa!"
            osSyncPrintf(b"\n\xe3\x81\x92\xe3\x81\xbc\xe3\x81\xaf\xe3\x81\xa3\xef\xbc\x81 \xe6\x8c\x87\xe5\xae\x9a\xe3\x81\x95\xe3\x82\x8c\xe3\x81\x9f\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe3\x81\x8c\xe3\x81\xaa\xe3\x81\x84\xe3\x81\xa7\xe3\x81\x88\xe3\x81\x88\xe3\x81\xa3\xe3\x81\x99\xef\xbc\x81\x00"
                             as *const u8 as *const libc::c_char);
            if gSaveContext.sceneSetupIndex == 3 as libc::c_int {
                altHeader =
                    *(gSegments[(((*cmd).altHeaders.segment as u32_0) <<
                                     4 as libc::c_int >> 28 as libc::c_int) as
                                    usize].wrapping_add((*cmd).altHeaders.segment
                                                            as u32_0 &
                                                            0xffffff as
                                                                libc::c_int as
                                                                libc::c_uint).wrapping_add(0x80000000
                                                                                               as
                                                                                               libc::c_uint)
                          as *mut libc::c_void as
                          *mut *mut SceneCmd).offset((gSaveContext.sceneSetupIndex
                                                          - 2 as libc::c_int)
                                                         as isize);
                // "Using adult day data there!"
                osSyncPrintf(b"\n\xe3\x81\x9d\xe3\x81\x93\xe3\x81\xa7\xe3\x80\x81\xe5\xa4\xa7\xe4\xba\xba\xe3\x81\xae\xe6\x98\xbc\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe3\x82\x92\xe4\xbd\xbf\xe7\x94\xa8\xe3\x81\x99\xe3\x82\x8b\xe3\x81\xa7\xe3\x81\x88\xe3\x81\x88\xe3\x81\xa3\xe3\x81\x99\xef\xbc\x81\xef\xbc\x81\x00"
                                 as *const u8 as *const libc::c_char);
                if !altHeader.is_null() {
                    Scene_ExecuteCommands(globalCtx,
                                          gSegments[((altHeader as u32_0) <<
                                                         4 as libc::c_int >>
                                                         28 as libc::c_int) as
                                                        usize].wrapping_add(altHeader
                                                                                as
                                                                                u32_0
                                                                                &
                                                                                0xffffff
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_add(0x80000000
                                                                                                                   as
                                                                                                                   libc::c_uint)
                                              as *mut libc::c_void as
                                              *mut SceneCmd);
                    (*cmd.offset(1 as libc::c_int as isize)).base.code =
                        0x14 as libc::c_int as u8_0
                }
            }
        }
    };
}
// Scene Command 0x17: Cutscene Data
#[no_mangle]
pub unsafe extern "C" fn func_8009934C(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    osSyncPrintf(b"\ngame_play->demo_play.data=[%x]\x00" as *const u8 as
                     *const libc::c_char, (*globalCtx).csCtx.segment);
    (*globalCtx).csCtx.segment =
        gSegments[(((*cmd).cutsceneData.segment as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*cmd).cutsceneData.segment as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void;
}
// Scene Command 0x19: Misc. Settings (Camera & World Map Area)
#[no_mangle]
pub unsafe extern "C" fn func_800993C0(mut globalCtx: *mut GlobalContext,
                                       mut cmd: *mut SceneCmd) {
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 15 as libc::c_int) as usize] =
        (*cmd).miscSettings.cameraMovement as s16;
    gSaveContext.worldMapArea = (*cmd).miscSettings.area as s16;
    if (*globalCtx).sceneNum as libc::c_int == SCENE_SHOP1 as libc::c_int ||
           (*globalCtx).sceneNum as libc::c_int ==
               SCENE_SYATEKIJYOU as libc::c_int {
        if (if !(gSaveContext.linkAge == 0 as libc::c_int) {
                5 as libc::c_int
            } else { 17 as libc::c_int }) == 17 as libc::c_int {
            gSaveContext.worldMapArea = 1 as libc::c_int as s16
        }
    }
    if (*globalCtx).sceneNum as libc::c_int >= SCENE_SPOT00 as libc::c_int &&
           (*globalCtx).sceneNum as libc::c_int <=
               SCENE_GANON_TOU as libc::c_int ||
           (*globalCtx).sceneNum as libc::c_int >= SCENE_ENTRA as libc::c_int
               &&
               (*globalCtx).sceneNum as libc::c_int <=
                   SCENE_SHRINE_R as libc::c_int {
        if gSaveContext.cutsceneIndex < 0xfff0 as libc::c_int {
            gSaveContext.worldMapAreaData |=
                gBitFlags[gSaveContext.worldMapArea as usize];
            osSyncPrintf(b"\xef\xbc\x90\xef\xbc\x90\xef\xbc\x90  \xef\xbd\x81\xef\xbd\x92\xef\xbd\x85\xef\xbd\x81\xef\xbc\xbf\xef\xbd\x81\xef\xbd\x92\xef\xbd\x92\xef\xbd\x89\xef\xbd\x96\xef\xbd\x81\xef\xbd\x8c\xef\xbc\x9d%x (%d)\n\x00"
                             as *const u8 as *const libc::c_char,
                         gSaveContext.worldMapAreaData,
                         gSaveContext.worldMapArea as libc::c_int);
        }
    };
}
#[no_mangle]
pub static mut gSceneCmdHandlers:
           [Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut SceneCmd) -> ()>; 26] =
    unsafe {
        [Some(func_80098508 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_800985DC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098630 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098674 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_800987A4 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80099090 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_800987F8 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_8009883C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098904 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80099134 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098958 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_8009899C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098B74 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098C24 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098C68 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098CC8 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098D80 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098D1C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_80098D5C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_800990F0 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()), None,
         Some(func_80099140 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_8009918C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_8009934C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_800991A0 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ()),
         Some(func_800993C0 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut SceneCmd) -> ())]
    };
// Initialized in run_static_initializers
#[no_mangle]
pub static mut sNaviMsgFiles: [RomFile; 3] =
    [RomFile{vromStart: 0, vromEnd: 0,}; 3];
#[no_mangle]
pub static mut gLinkObjectIds: [s16; 2] =
    [OBJECT_LINK_BOY as libc::c_int as s16,
     OBJECT_LINK_CHILD as libc::c_int as s16];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut gObjectTableSize: u32_0 = 0;
// Object linker symbol declarations (used in the table below)
// Object Table definition
// Initialized in run_static_initializers
#[no_mangle]
pub static mut gObjectTable: [RomFile; 402] =
    [RomFile{vromStart: 0, vromEnd: 0,}; 402];
unsafe extern "C" fn run_static_initializers() {
    sNaviMsgFiles =
        [{
             let mut init =
                 RomFile{vromStart:
                             _elf_message_fieldSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _elf_message_fieldSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _elf_message_ydanSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _elf_message_ydanSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         }];
    gObjectTableSize =
        (::std::mem::size_of::<[RomFile; 402]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<RomFile>() as
                                             libc::c_ulong) as s32 as u32_0;
    gObjectTable =
        [{
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _gameplay_keepSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _gameplay_keepSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _gameplay_field_keepSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _gameplay_field_keepSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _gameplay_dangeon_keepSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _gameplay_dangeon_keepSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_humanSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_humanSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_okutaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_okutaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_crowSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_crowSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_pohSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_pohSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dy_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_dy_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_wallmasterSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_wallmasterSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dodongoSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_dodongoSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fireflySegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_fireflySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_boxSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_boxSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fireSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_fireSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bubbleSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_bubbleSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_niwSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_niwSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_link_boySegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_link_boySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_link_childSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_link_childSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_titeSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_titeSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_reebaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_reebaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_peehatSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_peehatSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_kingdodongoSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_kingdodongoSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_horseSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_horseSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zfSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_zfSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gomaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gomaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zl1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_zl1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_golSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_golSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dodojrSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_dodojrSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_torch2SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_torch2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_blSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_blSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_tpSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_tpSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_stSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_stSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bwSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bwSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_eiSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_eiSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_horse_normalSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_horse_normalSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oB1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oB1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_o_animeSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_o_animeSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot04_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot04_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ddan_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_ddan_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_hidan_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_hidan_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_horse_ganonSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_horse_ganonSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot00_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot00_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mbSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_mbSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bombfSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_bombfSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_sk2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_sk2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE_animeSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oE_animeSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ydan_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_ydan_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gndSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_gndSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_amSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_amSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dekubabaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_dekubabaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA3SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA3SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA4SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA4SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA5SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA5SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA6SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA6SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA7SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA7SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_jjSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_jjSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA8SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA8SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA9SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oA9SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oB2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oB2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oB3SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oB3SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oB4SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oB4SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_horse_zeldaSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_horse_zeldaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_opening_demo1SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_opening_demo1SegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_warp1SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_warp1SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_b_heartSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_b_heartSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dekunutsSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_dekunutsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE3SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE3SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE4SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE4SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_menkuri_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_menkuri_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE5SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE5SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE6SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE6SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE7SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE7SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE8SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE8SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE9SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_oE9SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE10SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oE10SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE11SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oE11SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE12SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oE12SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_valiSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_valiSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA10SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oA10SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oA11SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oA11SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mizu_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_mizu_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fhgSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_fhgSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ossanSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_ossanSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mori_hineri1SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_mori_hineri1SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_BbSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_BbSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_toki_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_toki_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_yukabyunSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_yukabyunSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zl2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_zl2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjinSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjinSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjin_flashSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjin_flashSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjin_darkSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjin_darkSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjin_flameSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjin_flameSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjin_iceSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjin_iceSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjin_soulSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjin_soulSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjin_windSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjin_windSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mjin_okaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mjin_okaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_haka_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_haka_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot06_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot06_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ice_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_ice_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_relay_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_relay_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_po_fieldSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_po_fieldSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_po_composerSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_po_composerSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mori_hineri1aSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_mori_hineri1aSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mori_hineri2SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_mori_hineri2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mori_hineri2aSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_mori_hineri2aSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mori_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_mori_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mori_texSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mori_texSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot08_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot08_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_warp2SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_warp2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_hataSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_hataSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_birdSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_birdSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_wood02SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_wood02SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_lightboxSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_lightboxSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_pu_boxSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_pu_boxSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_trapSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_trapSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_vaseSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_vaseSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_imSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_imSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_taSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_taSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_tkSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_tkSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_xcSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_xcSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_vmSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_vmSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bvSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bvSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_hakach_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_hakach_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_crystal_lightSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_efc_crystal_lightSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_fire_ballSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_efc_fire_ballSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_flashSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_efc_flashSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_lgt_showerSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_efc_lgt_showerSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_star_fieldSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_efc_star_fieldSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_god_lgtSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_god_lgtSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_light_ringSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_light_ringSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_triforce_spotSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_triforce_spotSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bdan_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_bdan_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_sdSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_sdSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_rdSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_rdSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_po_sistersSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_po_sistersSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_heavy_objectSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_heavy_objectSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gnddSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gnddSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fdSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_fdSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_duSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_duSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fwSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_fwSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_medalSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_medalSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_horse_link_childSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_horse_link_childSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot02_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot02_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_hakaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_hakaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ru1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ru1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_syokudaiSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_syokudaiSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fd2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_fd2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dhSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_dhSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_rlSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_rlSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_twSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_efc_twSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_demo_tre_lgtSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_demo_tre_lgtSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_keySegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_keySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mir_raySegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mir_raySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_brobSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_brobSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_jewelSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_jewelSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot09_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot09_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot18_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot18_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bdoorSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_bdoorSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot17_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot17_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_shop_dungenSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_shop_dungenSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_nbSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_nbSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_moSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_moSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_sbSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_sbSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_melodySegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_melodySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_heartSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_heartSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_compassSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_compassSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_bosskeySegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_bosskeySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_medalSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_medalSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_nutsSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_nutsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_saSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_saSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_heartsSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_heartsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_arrowcaseSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_arrowcaseSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_bombpouchSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_bombpouchSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_inSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_inSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_trSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_trSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot16_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot16_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE1sSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oE1sSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oE4sSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oE4sSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_os_animeSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_os_animeSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_bottleSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_bottleSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_stickSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_stickSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_mapSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_mapSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oF1d_mapSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oF1d_mapSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ru2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ru2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_shield_1SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_shield_1SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dekujrSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_dekujrSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_magicpotSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_magicpotSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_bomb_1SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_bomb_1SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_oF1sSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_oF1sSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ma2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ma2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_purseSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_purseSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_hniSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_hniSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_twSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_twSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_rrSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_rrSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bxaSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bxaSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_anubiceSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_anubiceSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_gerudoSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_gerudoSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_arrowSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_arrowSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_bomb_2SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_bomb_2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_eggSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_eggSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_scaleSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_scaleSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_shield_2SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_shield_2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_hookshotSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_hookshotSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_ocarinaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_ocarinaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_milkSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_milkSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ma1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ma1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ganonSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_ganonSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_sstSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_sstSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_nySegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_nySegmentRomStart.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_nySegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_nySegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_frSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_frSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_pachinkoSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_pachinkoSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_boomerangSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_boomerangSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_bowSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_bowSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_glassesSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_glassesSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_liquidSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_liquidSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_aniSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_aniSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_demo_6kSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_demo_6kSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_shield_3SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_shield_3SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_letterSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_letterSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot15_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot15_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_jya_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_jya_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_clothesSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_clothesSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_beanSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_beanSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_fishSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_fishSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_sawSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_sawSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_hammerSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_hammerSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_grassSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_grassSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_longswordSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_longswordSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot01_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot01_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mdSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_mdSegmentRomStart.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mdSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_mdSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_km1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_km1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_kw1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_kw1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zoSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_zoSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_kzSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_kzSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_umajumpSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_umajumpSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_masterkokiriSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_masterkokiriSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_masterkokiriheadSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_masterkokiriheadSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mastergolonSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_mastergolonSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_masterzooraSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_masterzooraSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_aobSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_aobSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ikSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ikSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ahgSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ahgSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_cneSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_cneSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_niwatoriSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_niwatoriSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_skjSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_skjSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_bottle_letterSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_bottle_letterSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bjiSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bjiSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bbaSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bbaSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_ocarina_0SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_ocarina_0SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dsSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_dsSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_aneSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_aneSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bojSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bojSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot03_objectSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot03_objectSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot07_objectSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot07_objectSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fzSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_fzSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bobSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bobSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ge1SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ge1SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_yabusame_pointSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_yabusame_pointSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_boots_2SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_boots_2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_seedSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_seedSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gnd_magicSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gnd_magicSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_d_elevatorSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_d_elevatorSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_d_hsblockSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_d_hsblockSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_d_liftSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_d_liftSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mamenokiSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_mamenokiSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_goroiwaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_goroiwaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_toryoSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_toryoSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_daikuSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_daikuSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_nwcSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_nwcSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_blkobjSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_blkobjSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gmSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_gmSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_msSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_msSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_hsSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_hsSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ingateSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_ingateSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_lightswitchSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_lightswitchSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_kusaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_kusaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_tsuboSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_tsuboSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_glovesSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_glovesSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_coinSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_coinSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_kanbanSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_kanbanSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gjyo_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gjyo_objectsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_owlSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_owlSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mkSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_mkSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fuSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_fuSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_ki_tan_maskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_ki_tan_maskSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_redead_maskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_redead_maskSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_skj_maskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_skj_maskSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_rabit_maskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_rabit_maskSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_truth_maskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_truth_maskSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ganon_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_ganon_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_siofukiSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_siofukiSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_streamSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_streamSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_mmSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_mmSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_faSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_faSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_osSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_osSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_eye_lotionSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_eye_lotionSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_powderSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_powderSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_mushroomSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_mushroomSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_ticketstoneSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_ticketstoneSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_brokenswordSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_brokenswordSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_jsSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_jsSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_csSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_csSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_prescriptionSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_prescriptionSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_braceletSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_braceletSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_soldoutSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_soldoutSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_frogSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_frogSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_magSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_magSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_door_gerudoSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_door_gerudoSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gtSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_gtSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_erupcSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_efc_erupcSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zl2_anime1SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_zl2_anime1SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zl2_anime2SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_zl2_anime2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_golonmaskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_golonmaskSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_zoramaskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_zoramaskSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_gerudomaskSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_gerudomaskSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ganon2SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_ganon2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_kaSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_kaSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_tsSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_tsSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zgSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_zgSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_hoverbootsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_hoverbootsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_m_arrowSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_m_arrowSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ds2SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ds2SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ecSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_ecSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_fishSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_fishSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_sutaruSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_sutaruSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_goddessSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_goddessSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_sshSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_sshSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bigokutaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_bigokutaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bgSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_bgSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot05_objectsSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot05_objectsSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot12_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot12_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bombiwaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_bombiwaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_hintnutsSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_hintnutsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_rsSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_rsSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot00_breakSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot00_breakSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_glaSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_glaSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_shopnutsSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_shopnutsSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_geldbSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_geldbSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_grSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_grSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dogSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_dogSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_jya_ironSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_jya_ironSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_jya_doorSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_jya_doorSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart: 0 as libc::c_int as u32_0, vromEnd: 0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot11_objSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_spot11_objSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_kibako2SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_kibako2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dnsSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_dnsSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_dnkSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_dnkSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_fireSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_fireSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_insectSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_insectSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_butterflySegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_butterflySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_ghostSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_ghostSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_soulSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_soulSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bowlSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_bowlSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_demo_kekkaiSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_demo_kekkaiSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_efc_doughnutSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_efc_doughnutSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_dekupouchSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_gi_dekupouchSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ganon_anime1SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_ganon_anime1SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ganon_anime2SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_ganon_anime2SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ganon_anime3SegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_ganon_anime3SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_rupySegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_rupySegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot01_matoyaSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot01_matoyaSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_spot01_matoyabSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_spot01_matoyabSegmentRomEnd.as_mut_ptr()
                                 as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_muSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_muSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_wfSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_wfSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_skbSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_skbSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gjSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_gjSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_geffSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_geffSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_haka_doorSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_haka_doorSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gsSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_gsSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_psSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_psSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_bwallSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_bwallSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_cowSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_cowSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_cobSegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_cobSegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_gi_sword_1SegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_gi_sword_1SegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_door_killerSegmentRomStart.as_mut_ptr()
                                 as u32_0,
                         vromEnd:
                             _object_door_killerSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_ouke_hakaSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_ouke_hakaSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_timeblockSegmentRomStart.as_mut_ptr() as
                                 u32_0,
                         vromEnd:
                             _object_timeblockSegmentRomEnd.as_mut_ptr() as
                                 u32_0,};
             init
         },
         {
             let mut init =
                 RomFile{vromStart:
                             _object_zl4SegmentRomStart.as_mut_ptr() as u32_0,
                         vromEnd:
                             _object_zl4SegmentRomEnd.as_mut_ptr() as u32_0,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
