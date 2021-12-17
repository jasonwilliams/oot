#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    static mut Player_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TestSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TestSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TestSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TestSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Test_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GirlASegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GirlASegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GirlASegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GirlASegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_GirlA_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_PartSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PartSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PartSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PartSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Part_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_LightSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_LightSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_LightSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_LightSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Light_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DoorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DoorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DoorSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DoorSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Door_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BoxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BoxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BoxSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BoxSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Box_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Dy_YoseizoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Dy_YoseizoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Dy_YoseizoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Dy_YoseizoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Dy_Yoseizo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FirewallSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FirewallSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FirewallSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FirewallSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Firewall_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_PohSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PohSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PohSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PohSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Poh_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_OkutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OkutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OkutaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OkutaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Okuta_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ydan_SpSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_SpSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_SpSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_SpSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ydan_Sp_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BomSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BomSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BomSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BomSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Bom_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_WallmasSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WallmasSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WallmasSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WallmasSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Wallmas_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DodongoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DodongoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DodongoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DodongoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Dodongo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FireflySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FireflySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FireflySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FireflySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Firefly_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_HorseSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HorseSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HorseSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HorseSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Horse_InitVars: ActorInit;
    #[no_mangle]
    static mut En_Item00_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ArrowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ArrowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ArrowSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ArrowSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Arrow_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ElfSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ElfSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ElfSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ElfSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Elf_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_NiwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NiwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NiwSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NiwSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Niw_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TiteSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TiteSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TiteSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TiteSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Tite_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ReebaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ReebaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ReebaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ReebaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Reeba_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_PeehatSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PeehatSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PeehatSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_PeehatSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Peehat_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ButteSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ButteSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ButteSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ButteSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Butte_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_InsectSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_InsectSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_InsectSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_InsectSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Insect_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FishSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FishSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FishSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FishSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Fish_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_HollSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HollSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HollSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HollSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Holl_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Scene_ChangeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Scene_ChangeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Scene_ChangeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Scene_ChangeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Scene_Change_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ZfSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ZfSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ZfSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ZfSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Zf_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_HataSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HataSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HataSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HataSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Hata_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_DodongoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_DodongoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_DodongoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_DodongoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Dodongo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_GomaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GomaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GomaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GomaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Goma_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Zl1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Zl1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ViewerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ViewerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ViewerSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ViewerSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Viewer_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GomaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GomaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GomaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GomaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Goma_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_PushboxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_PushboxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_PushboxSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_PushboxSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Pushbox_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BubbleSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BubbleSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BubbleSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BubbleSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Bubble_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Door_ShutterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_ShutterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_ShutterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_ShutterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Door_Shutter_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DodojrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DodojrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DodojrSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DodojrSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Dodojr_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BdfireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BdfireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BdfireSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BdfireSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Bdfire_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BoomSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BoomSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BoomSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BoomSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Boom_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Torch2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Torch2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Torch2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Torch2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Torch2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BiliSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BiliSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BiliSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BiliSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Bili_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TpSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TpSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TpSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TpSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Tp_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_StSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_StSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_StSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_StSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_St_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BwSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BwSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Bw_InitVars: ActorInit;
    #[no_mangle]
    static mut En_A_Obj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_EiyerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_EiyerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_EiyerSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_EiyerSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Eiyer_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_River_SoundSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_River_SoundSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_River_SoundSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_River_SoundSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_River_Sound_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Horse_NormalSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_NormalSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_NormalSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_NormalSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Horse_Normal_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_OssanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OssanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OssanSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OssanSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Ossan_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_TreemouthSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_TreemouthSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_TreemouthSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_TreemouthSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Treemouth_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_DodoagoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_DodoagoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_DodoagoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_DodoagoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Dodoago_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_DalmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_DalmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_DalmSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_DalmSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Dalm_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HrockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HrockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HrockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HrockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Hrock_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Horse_GanonSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_GanonSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_GanonSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_GanonSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Horse_Ganon_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Rock_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RsekizouSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RsekizouSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RsekizouSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_RsekizouSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Rsekizou_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SekizouSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SekizouSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SekizouSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SekizouSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Sekizou_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SimaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SimaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SimaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SimaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Sima_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SyokuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SyokuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SyokuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_SyokuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Syoku_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_XcSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_XcSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_XcSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_XcSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Xc_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_CurtainSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_CurtainSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_CurtainSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_CurtainSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Curtain_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot00_HanebasiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot00_HanebasiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot00_HanebasiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot00_HanebasiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot00_Hanebasi_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_MbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MbSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MbSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Mb_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BombfSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BombfSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BombfSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BombfSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Bombf_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Zl2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Zl2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FsliftSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FsliftSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FsliftSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FsliftSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Fslift_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_OE2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OE2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OE2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OE2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_OE2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ydan_HasiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_HasiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_HasiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_HasiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ydan_Hasi_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ydan_MarutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_MarutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_MarutaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ydan_MarutaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ydan_Maruta_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_GanondrofSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GanondrofSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GanondrofSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GanondrofSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Ganondrof_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_AmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AmSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AmSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Am_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DekubabaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DekubabaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DekubabaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DekubabaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_Dekubaba_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_M_Fire1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_M_Fire1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_M_Fire1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_M_Fire1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_M_Fire1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_M_ThunderSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_M_ThunderSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_M_ThunderSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_M_ThunderSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut En_M_Thunder_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ddan_JdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ddan_JdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ddan_JdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ddan_JdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ddan_Jd_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_BreakwallSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_BreakwallSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_BreakwallSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_BreakwallSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Breakwall_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_JjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut Obj_Warp2block_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_Warp2blockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Warp2blockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Warp2blockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Warp2blockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Block_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_BlockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BlockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BlockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BlockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Mm2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Mm2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Mm2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Mm2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Mm2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Zl4_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Zl4SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl4SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Hamishi_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_HamishiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HamishiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HamishiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HamishiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Timeblock_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_TimeblockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_TimeblockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_TimeblockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_TimeblockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ge3_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ge3SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge3SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Makekinsuta_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_MakekinsutaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MakekinsutaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MakekinsutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MakekinsutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Zo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ZoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ZoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ZoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ZoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Menkuri_Nisekabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_NisekabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_NisekabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_NisekabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_NisekabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Eg_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_EgSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_EgSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_EgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_EgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Oceff_Wipe4_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Oceff_Wipe4SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe4SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Kakasi3_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Kakasi3SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Kakasi3SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Kakasi3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Kakasi3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Kakasi2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Kakasi2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Kakasi2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Kakasi2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Kakasi2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ice_Shutter_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShutterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShutterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShutterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShutterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ice_Turara_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ice_TuraraSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_TuraraSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_TuraraSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_TuraraSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Cow_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_CowSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CowSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ma3_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ma3SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma3SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot18_Shutter_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ShutterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ShutterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ShutterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ShutterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot18_Futa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot18_FutaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_FutaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_FutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_FutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot11_Oasis_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot11_OasisSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot11_OasisSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot11_OasisSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot11_OasisSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Door_Killer_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Door_KillerSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_KillerSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_KillerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_KillerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Crow_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_CrowSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CrowSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CrowSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CrowSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Po_Desert_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Po_DesertSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_DesertSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_DesertSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_DesertSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Wall_Tubo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Wall_TuboSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wall_TuboSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wall_TuboSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wall_TuboSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Bowl_Wall_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Bowl_WallSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bowl_WallSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bowl_WallSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bowl_WallSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Daiku_Kakariko_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Daiku_KakarikoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Daiku_KakarikoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Daiku_KakarikoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Daiku_KakarikoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mizu_Shutter_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mizu_ShutterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_ShutterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_ShutterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_ShutterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mizu_Bwall_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mizu_BwallSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_BwallSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_BwallSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_BwallSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Gs_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Gb_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GbSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GbSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Gnd_Iceblock_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Gnd_IceblockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_IceblockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_IceblockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_IceblockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Gnd_Nisekabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Gnd_NisekabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_NisekabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_NisekabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_NisekabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Gnd_Soulmeiro_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Gnd_SoulmeiroSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_SoulmeiroSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_SoulmeiroSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_SoulmeiroSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Gnd_Darkmeiro_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Gnd_DarkmeiroSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_DarkmeiroSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_DarkmeiroSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_DarkmeiroSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Gnd_Firemeiro_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Gnd_FiremeiroSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_FiremeiroSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_FiremeiroSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gnd_FiremeiroSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Geff_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_GeffSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GeffSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GeffSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GeffSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Gj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_GjSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GjSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Skb_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SkbSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkbSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Wf_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_WfSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WfSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WfSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WfSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Go2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Go2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Go2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Go2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Go2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Mu_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_MuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Tg_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TgSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TgSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Mure3_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_Mure3SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Mure3SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Mure3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Mure3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot17_Bakudankabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot17_BakudankabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot17_BakudankabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot17_BakudankabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot17_BakudankabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot08_Bakudankabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot08_BakudankabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot08_BakudankabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot08_BakudankabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot08_BakudankabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Kekkai_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_KekkaiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_KekkaiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_KekkaiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_KekkaiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Hs2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Hs2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Hs2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Hs2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Hs2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Bom_Guard_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Bom_GuardSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bom_GuardSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bom_GuardSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bom_GuardSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Guest_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GuestSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GuestSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GuestSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GuestSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dnt_Nomal_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Dnt_NomalSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_NomalSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_NomalSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_NomalSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dnt_Jiji_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Dnt_JijiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_JijiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_JijiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_JijiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dnt_Demo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Dnt_DemoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_DemoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_DemoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dnt_DemoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Kibako2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_Kibako2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Kibako2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Kibako2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Kibako2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot11_Bakudankabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot11_BakudankabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot11_BakudankabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot11_BakudankabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot11_BakudankabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Comb_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_CombSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_CombSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_CombSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_CombSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot01_Objects2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot01_Objects2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_Objects2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_Objects2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_Objects2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Si_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dog_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DogSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DogSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DogSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DogSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Niw_Girl_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Niw_GirlSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Niw_GirlSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Niw_GirlSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Niw_GirlSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Oceff_Wipe3_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Oceff_Wipe3SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe3SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Oceff_Wipe2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Oceff_Wipe2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_Wipe2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_GeldB_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GeldBSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GeldBSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GeldBSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GeldBSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_It_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ItSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ItSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ItSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ItSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Shopnuts_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ShopnutsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ShopnutsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ShopnutsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ShopnutsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot00_Break_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot00_BreakSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot00_BreakSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot00_BreakSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot00_BreakSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Nutsball_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_NutsballSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NutsballSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NutsballSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NutsballSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Hintnuts_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_HintnutsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HintnutsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HintnutsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HintnutsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot12_Saku_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot12_SakuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot12_SakuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot12_SakuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot12_SakuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot12_Gate_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot12_GateSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot12_GateSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot12_GateSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot12_GateSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Haheniron_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_HahenironSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_HahenironSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_HahenironSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_HahenironSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_1flift_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_1fliftSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_1fliftSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_1fliftSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_1fliftSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot05_Soko_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot05_SokoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot05_SokoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot05_SokoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot05_SokoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Weiyer_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_WeiyerSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WeiyerSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WeiyerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_WeiyerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Oceff_Storm_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Oceff_StormSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_StormSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_StormSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_StormSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Oceff_Wipe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Oceff_WipeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_WipeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_WipeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_WipeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Sth_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SthSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SthSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SthSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SthSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ssh_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SshSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SshSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SshSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SshSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Roomtimer_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_RoomtimerSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_RoomtimerSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_RoomtimerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_RoomtimerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ge2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ge2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Wonder_Talk2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Wonder_Talk2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_Talk2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_Talk2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_Talk2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dy_Extra_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Dy_ExtraSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dy_ExtraSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dy_ExtraSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Dy_ExtraSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Shot_Sun_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Shot_SunSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Shot_SunSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Shot_SunSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Shot_SunSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Ec_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_EcSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_EcSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_EcSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_EcSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Torch_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TorchSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TorchSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TorchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TorchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut End_Title_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_End_TitleSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_End_TitleSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_End_TitleSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_End_TitleSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Oceff_Spot_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Oceff_SpotSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_SpotSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_SpotSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Oceff_SpotSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Makeoshihiki_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_MakeoshihikiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MakeoshihikiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MakeoshihikiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MakeoshihikiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Takara_Man_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Takara_ManSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Takara_ManSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Takara_ManSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Takara_ManSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Kakasi_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_KakasiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KakasiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KakasiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KakasiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Ganon2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_Ganon2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_Ganon2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_Ganon2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_Ganon2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Zl3_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Zl3SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl3SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Zl3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Heishi4_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Heishi4SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi4SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Zg_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_ZgSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_ZgSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_ZgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_ZgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Efc_Erupc_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Efc_ErupcSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Efc_ErupcSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Efc_ErupcSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Efc_ErupcSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Po_Field_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Po_FieldSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_FieldSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_FieldSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_FieldSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Gt_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_GtSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GtSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GtSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GtSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Elf_Msg2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Elf_Msg2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Elf_Msg2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Elf_Msg2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Elf_Msg2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Door_Gerudo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Door_GerudoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_GerudoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_GerudoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_GerudoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Mag_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_MagSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MagSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MagSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MagSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Okarina_Effect_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Okarina_EffectSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Okarina_EffectSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Okarina_EffectSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Okarina_EffectSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ganon_Mant_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ganon_MantSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ganon_MantSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ganon_MantSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ganon_MantSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Hy_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_HySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Md_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_MdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Cs_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_CsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_CsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Jsjutan_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_JsjutanSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JsjutanSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JsjutanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JsjutanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Js_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_JsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Ironobj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_IronobjSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_IronobjSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_IronobjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_IronobjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ex_Item_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ex_ItemSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ex_ItemSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ex_ItemSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ex_ItemSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ani_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_AniSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AniSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AniSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AniSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Sst_Floor_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Sst_FloorSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Sst_FloorSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Sst_FloorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Sst_FloorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Weather_Tag_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Weather_TagSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Weather_TagSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Weather_TagSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Weather_TagSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Kz_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_KzSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KzSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KzSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KzSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ko_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_KoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Mm_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_MmSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MmSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Stream_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_StreamSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_StreamSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_StreamSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_StreamSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Siofuki_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SiofukiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SiofukiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SiofukiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SiofukiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ganon_Organ_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ganon_OrganSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ganon_OrganSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ganon_OrganSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ganon_OrganSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot18_Basket_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot18_BasketSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_BasketSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_BasketSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_BasketSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Bombiwa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombiwaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombiwaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombiwaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombiwaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Amishutter_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_AmishutterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_AmishutterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_AmishutterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_AmishutterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Bombchuiwa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombchuiwaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombchuiwaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombchuiwaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BombchuiwaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Bigmirror_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_BigmirrorSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BigmirrorSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BigmirrorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_BigmirrorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Lift_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_LiftSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_LiftSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_LiftSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_LiftSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Megami_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_MegamiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_MegamiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_MegamiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_MegamiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Changer_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ChangerSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ChangerSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ChangerSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ChangerSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fu_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Go_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Mure2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_Mure2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Mure2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Mure2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Mure2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Lightswitch_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_LightswitchSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_LightswitchSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_LightswitchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_LightswitchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Hana_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_HanaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HanaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HanaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HanaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ishi_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_IshiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_IshiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_IshiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_IshiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Owl_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_OwlSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OwlSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OwlSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_OwlSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Bom_Bowl_Pit_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_PitSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_PitSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_PitSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_PitSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Bom_Bowl_Man_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_ManSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_ManSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_ManSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_Bowl_ManSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Mk_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_MkSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MkSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ds_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Gjyo_Bridge_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Gjyo_BridgeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gjyo_BridgeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gjyo_BridgeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gjyo_BridgeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Wonder_Talk_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Wonder_TalkSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_TalkSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_TalkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_TalkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Sa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot01_Idosoko_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdosokoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdosokoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdosokoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdosokoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Attack_Niw_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Attack_NiwSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Attack_NiwSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Attack_NiwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Attack_NiwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Syateki_Niw_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Syateki_NiwSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_NiwSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_NiwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_NiwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Heishi3_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Heishi3SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi3SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi3SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi3SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Kanban_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_KanbanSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KanbanSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KanbanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KanbanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ingate_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_IngateSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_IngateSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_IngateSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_IngateSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Hs_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_HsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ms_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_MsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_MsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Gm_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GmSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GmSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Niw_Lady_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Niw_LadySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Niw_LadySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Niw_LadySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Niw_LadySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Clear_Tag_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Clear_TagSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Clear_TagSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Clear_TagSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Clear_TagSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Sda_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SdaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SdaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SdaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SdaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Blockstop_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_BlockstopSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BlockstopSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BlockstopSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BlockstopSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ge1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ge1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ge1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Item_Inbox_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Item_InboxSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_InboxSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_InboxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_InboxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Blkobj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BlkobjSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BlkobjSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BlkobjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BlkobjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Nwc_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_NwcSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NwcSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NwcSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NwcSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Daiku_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DaikuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DaikuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DaikuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DaikuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Toryo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ToryoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ToryoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ToryoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ToryoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ex_Ruppy_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ex_RuppySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ex_RuppySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ex_RuppySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ex_RuppySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Goroiwa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_GoroiwaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GoroiwaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GoroiwaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_GoroiwaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Yabusame_Mark_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Yabusame_MarkSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Yabusame_MarkSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Yabusame_MarkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Yabusame_MarkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Okarina_Tag_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Okarina_TagSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Okarina_TagSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Okarina_TagSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Okarina_TagSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Hsblock_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_HsblockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HsblockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HsblockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_HsblockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Lift_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_LiftSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_LiftSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_LiftSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_LiftSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Elevator_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_ElevatorSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_ElevatorSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_ElevatorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_ElevatorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Switch_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_SwitchSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_SwitchSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_SwitchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_SwitchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Bombiwa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_BombiwaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BombiwaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BombiwaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BombiwaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Bean_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_BeanSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BeanSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BeanSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_BeanSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Kusa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_KusaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KusaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KusaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KusaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Diving_Game_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Diving_GameSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Diving_GameSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Diving_GameSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Diving_GameSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Relay_Objects_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Relay_ObjectsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Relay_ObjectsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Relay_ObjectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Relay_ObjectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Po_Relay_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Po_RelaySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_RelaySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_RelaySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_RelaySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fz_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FzSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FzSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FzSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FzSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot07_Taki_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot07_TakiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot07_TakiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot07_TakiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot07_TakiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot03_Taki_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot03_TakiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot03_TakiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot03_TakiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot03_TakiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Ice_Poly_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_Ice_PolySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Ice_PolySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Ice_PolySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_Ice_PolySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Tubo_Trap_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Tubo_TrapSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Tubo_TrapSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Tubo_TrapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Tubo_TrapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Honotrap_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_HonotrapSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HonotrapSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HonotrapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_HonotrapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Elf_Msg_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Elf_MsgSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Elf_MsgSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Elf_MsgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Elf_MsgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dns_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DnsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DnsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DnsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DnsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Shd_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_ShdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ShdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ShdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ShdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Ext_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_ExtSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ExtSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ExtSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ExtSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_G_Switch_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_G_SwitchSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_G_SwitchSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_G_SwitchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_G_SwitchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Skjneedle_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SkjneedleSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkjneedleSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkjneedleSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkjneedleSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Skj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SkjSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkjSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SkjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Ik_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_IkSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_IkSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_IkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_IkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ik_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_IkSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_IkSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_IkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_IkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Wonder_Item_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Wonder_ItemSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_ItemSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_ItemSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wonder_ItemSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Tsubo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_TsuboSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_TsuboSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_TsuboSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_TsuboSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Kibako_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_KibakoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_KibakoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_KibakoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_KibakoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Item_Etcetera_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Item_EtceteraSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_EtceteraSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_EtceteraSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_EtceteraSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Arrow_Light_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Arrow_LightSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_LightSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_LightSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_LightSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Arrow_Ice_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Arrow_IceSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_IceSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_IceSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_IceSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Arrow_Fire_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Arrow_FireSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_FireSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_FireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arrow_FireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Umajump_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_UmajumpSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_UmajumpSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_UmajumpSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_UmajumpSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot15_Rrbox_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot15_RrboxSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot15_RrboxSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot15_RrboxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot15_RrboxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ganon_Otyuka_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ganon_OtyukaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ganon_OtyukaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ganon_OtyukaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ganon_OtyukaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Po_Syokudai_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Po_SyokudaiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Po_SyokudaiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Po_SyokudaiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Po_SyokudaiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot01_Idomizu_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdomizuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdomizuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdomizuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdomizuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot01_Idohashira_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdohashiraSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdohashiraSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdohashiraSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_IdohashiraSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot01_Fusya_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot01_FusyaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_FusyaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_FusyaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot01_FusyaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Eff_Dust_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Eff_DustSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Eff_DustSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Eff_DustSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Eff_DustSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Gate_Shutter_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Gate_ShutterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gate_ShutterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gate_ShutterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Gate_ShutterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Oshihiki_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_OshihikiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_OshihikiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_OshihikiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_OshihikiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Fishing_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_FishingSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_FishingSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_FishingSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_FishingSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Kanaami_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_KanaamiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_KanaamiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_KanaamiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_KanaamiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Cobra_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_CobraSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_CobraSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_CobraSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_CobraSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Zurerukabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_ZurerukabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_ZurerukabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_ZurerukabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_ZurerukabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Jya_Goroiwa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Jya_GoroiwaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_GoroiwaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_GoroiwaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Jya_GoroiwaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot15_Saku_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot15_SakuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot15_SakuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot15_SakuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot15_SakuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Gate_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_GateSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_GateSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_GateSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_GateSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Anubice_Tag_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Anubice_TagSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Anubice_TagSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Anubice_TagSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Anubice_TagSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_6K_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_6KSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_6KSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_6KSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_6KSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Magic_Dark_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Magic_DarkSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_DarkSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_DarkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_DarkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Item_Ocarina_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Item_OcarinaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_OcarinaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_OcarinaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_OcarinaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ice_Hono_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ice_HonoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ice_HonoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ice_HonoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ice_HonoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ice_Shelter_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShelterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShelterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShelterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ShelterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Item_Shield_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Item_ShieldSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_ShieldSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_ShieldSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_ShieldSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fr_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FrSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FrSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ny_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_NySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Sst_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_SstSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_SstSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_SstSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_SstSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Ganon_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_GanonSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GanonSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GanonSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_GanonSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ma1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ma1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Bdan_Switch_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Bdan_SwitchSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bdan_SwitchSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bdan_SwitchSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bdan_SwitchSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot16_Doughnut_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot16_DoughnutSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot16_DoughnutSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot16_DoughnutSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot16_DoughnutSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Idomizu_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_IdomizuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_IdomizuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_IdomizuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_IdomizuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Hashira4_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_Hashira4SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_Hashira4SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_Hashira4SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_Hashira4SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Hashigo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_HashigoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_HashigoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_HashigoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_HashigoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Anubice_Fire_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Anubice_FireSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Anubice_FireSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Anubice_FireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Anubice_FireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Anubice_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_AnubiceSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AnubiceSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AnubiceSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_AnubiceSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Bx_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BxSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BxSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ba_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Rr_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_RrSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RrSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Tw_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_TwSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_TwSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_TwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_TwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Horse_Game_Check_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Horse_Game_CheckSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_Game_CheckSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_Game_CheckSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_Game_CheckSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Bom_Chu_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Bom_ChuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_ChuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_ChuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Bom_ChuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ma2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ma2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ma2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Water_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_WaterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_WaterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_WaterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_WaterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ice_Objects_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ice_ObjectsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ObjectsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ObjectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ice_ObjectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot06_Objects_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot06_ObjectsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot06_ObjectsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot06_ObjectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot06_ObjectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mizu_Uzu_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mizu_UzuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_UzuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_UzuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_UzuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Dekujr_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_DekujrSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_DekujrSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_DekujrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_DekujrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ru2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ru2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ru2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ru2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ru2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot08_Iceblock_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot08_IceblockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot08_IceblockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot08_IceblockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot08_IceblockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Bombwall_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_BombwallSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_BombwallSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_BombwallSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_BombwallSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Kowarerukabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KowarerukabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KowarerukabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KowarerukabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KowarerukabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot16_Bombstone_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot16_BombstoneSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot16_BombstoneSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot16_BombstoneSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot16_BombstoneSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Tr_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TrSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TrSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TrSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TrSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_In_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_InSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_InSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_InSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_InSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Go_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_GoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_GoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Sa_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_SaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_SaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_SaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_SaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Bdan_Objects_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Bdan_ObjectsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bdan_ObjectsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bdan_ObjectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Bdan_ObjectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Karebaba_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_KarebabaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KarebabaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KarebabaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_KarebabaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Bigokuta_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BigokutaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BigokutaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BigokutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BigokutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Sb_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SbSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SbSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Mo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_MoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_MoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_MoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_MoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Nb_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_NbSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NbSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_NbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Tana_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TanaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TanaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TanaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TanaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Syateki_Man_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Syateki_ManSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_ManSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_ManSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_ManSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Syateki_Itm_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Syateki_ItmSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_ItmSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_ItmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Syateki_ItmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot17_Funen_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot17_FunenSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot17_FunenSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot17_FunenSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot17_FunenSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Zou_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_ZouSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_ZouSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_ZouSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_ZouSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Huta_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_HutaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_HutaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_HutaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_HutaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Trap_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_TrapSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_TrapSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_TrapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_TrapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Tubo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_TuboSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_TuboSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_TuboSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_TuboSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Va_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_VaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_VaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_VaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_VaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot18_Obj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ObjSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ObjSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ObjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot18_ObjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot09_Obj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot09_ObjSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot09_ObjSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot09_ObjSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot09_ObjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Mir_Ray_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Mir_RaySegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Mir_RaySegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Mir_RaySegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Mir_RaySegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Brob_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BrobSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BrobSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BrobSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BrobSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fire_Rock_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Fire_RockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fire_RockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fire_RockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fire_RockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Encount2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Encount2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Encount2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Encount2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Encount2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Heishi2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Heishi2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Sgami_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_SgamiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_SgamiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_SgamiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_SgamiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Ship_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_ShipSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_ShipSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_ShipSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_ShipSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_MeganeBG_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeBGSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeBGSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeBGSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeBGSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_Megane_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Haka_MeganeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Vb_Ball_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Vb_BallSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Vb_BallSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Vb_BallSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Vb_BallSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Vb_Sima_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Vb_SimaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Vb_SimaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Vb_SimaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Vb_SimaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fw_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FwSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FwSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Tre_Lgt_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_Tre_LgtSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_Tre_LgtSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_Tre_LgtSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_Tre_LgtSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Im_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_ImSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ImSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ImSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_ImSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Du_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_DuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_DuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_DuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_DuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Encount1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Encount1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Encount1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Encount1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Encount1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Rl_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_RlSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RlSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RlSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RlSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dha_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DhaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DhaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DhaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DhaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dh_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DhSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DhSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DhSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DhSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fd_Fire_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Fd_FireSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fd_FireSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fd_FireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fd_FireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Fd2_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_Fd2SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_Fd2SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_Fd2SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_Fd2SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ru1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Ru1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ru1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ru1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Ru1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Magic_Fire_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Magic_FireSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_FireSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_FireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_FireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Magic_Wind_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Magic_WindSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_WindSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_WindSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Magic_WindSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_HakaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_HakaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_HakaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_HakaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Spot02_Objects_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Spot02_ObjectsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot02_ObjectsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot02_ObjectsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Spot02_ObjectsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Door_Ana_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Door_AnaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_AnaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_AnaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_AnaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Horse_Link_Child_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Horse_Link_ChildSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_Link_ChildSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_Link_ChildSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_Link_ChildSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fd_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Du_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DuSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DuSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DuSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DuSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Object_Kankyo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Object_KankyoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Object_KankyoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Object_KankyoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Object_KankyoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Boss_Fd_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Boss_FdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_FdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_FdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Boss_FdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Sw_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_SwSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SwSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SwSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_SwSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Mure_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_MureSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MureSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MureSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_MureSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Po_Event_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Po_EventSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Po_EventSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Po_EventSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Po_EventSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Heavy_Block_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Heavy_BlockSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Heavy_BlockSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Heavy_BlockSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Heavy_BlockSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Po_Sisters_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Po_SistersSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_SistersSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_SistersSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Po_SistersSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Rd_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_RdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_RdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Heishi1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Heishi1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Heishi1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Floormas_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_FloormasSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FloormasSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FloormasSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_FloormasSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Fwbig_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FwbigSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FwbigSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FwbigSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_FwbigSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Kankyo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_KankyoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_KankyoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_KankyoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_KankyoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Demo_Effect_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Demo_EffectSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_EffectSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_EffectSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Demo_EffectSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Vm_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_VmSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_VmSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_VmSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_VmSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Rakkatenjo_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_RakkatenjoSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_RakkatenjoSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_RakkatenjoSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_RakkatenjoSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Kaitenkabe_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_KaitenkabeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_KaitenkabeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_KaitenkabeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_KaitenkabeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Elevator_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_ElevatorSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_ElevatorSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_ElevatorSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_ElevatorSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Bigst_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_BigstSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_BigstSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_BigstSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_BigstSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Tk_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TkSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TkSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TkSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TkSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Ta_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Vase_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_VaseSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_VaseSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_VaseSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_VaseSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Arow_Trap_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Arow_TrapSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Arow_TrapSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Arow_TrapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Arow_TrapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Trap_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_TrapSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TrapSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TrapSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_TrapSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Pu_box_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Pu_boxSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Pu_boxSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Pu_boxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Pu_boxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Lightbox_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_LightboxSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_LightboxSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_LightboxSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_LightboxSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Wood02_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Wood02SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wood02SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wood02SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Wood02SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Bird_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BirdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BirdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BirdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BirdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Hamstep_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HamstepSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HamstepSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HamstepSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_HamstepSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Door_Toki_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Door_TokiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_TokiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_TokiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_TokiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Hidan_Kousi_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KousiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KousiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KousiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Hidan_KousiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mjin_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_MjinSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_MjinSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_MjinSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_MjinSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Fhg_Fire_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Fhg_FireSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fhg_FireSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fhg_FireSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Fhg_FireSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Toki_Swd_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Toki_SwdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Toki_SwdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Toki_SwdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Toki_SwdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Yukabyun_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_YukabyunSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_YukabyunSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_YukabyunSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_YukabyunSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Toki_Hikari_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Toki_HikariSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Toki_HikariSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Toki_HikariSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Toki_HikariSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Bb_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_BbSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BbSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BbSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_BbSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mori_Hineri_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mori_HineriSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_HineriSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_HineriSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mori_HineriSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_fHG_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_fHGSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_fHGSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_fHGSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_fHGSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Arms_Hook_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Arms_HookSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arms_HookSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arms_HookSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Arms_HookSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mizu_Water_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mizu_WaterSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_WaterSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_WaterSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_WaterSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Mizu_Movebg_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Mizu_MovebgSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_MovebgSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_MovebgSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Mizu_MovebgSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Vali_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_ValiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ValiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ValiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_ValiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Menkuri_Eye_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_EyeSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_EyeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_EyeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_EyeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Menkuri_Kaiten_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_KaitenSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_KaitenSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_KaitenSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Menkuri_KaitenSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Dekunuts_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_DekunutsSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DekunutsSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DekunutsSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_DekunutsSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Item_B_Heart_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Item_B_HeartSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_B_HeartSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_B_HeartSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Item_B_HeartSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Obj_Syokudai_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Obj_SyokudaiSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_SyokudaiSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_SyokudaiSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Obj_SyokudaiSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Door_Warp1_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Door_Warp1SegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_Warp1SegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_Warp1SegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Door_Warp1SegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Ddan_Kd_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_Bg_Ddan_KdSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ddan_KdSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ddan_KdSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_Bg_Ddan_KdSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Horse_Zelda_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_Horse_ZeldaSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_ZeldaSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_ZeldaSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_Horse_ZeldaSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut En_Jj_InitVars: ActorInit;
    #[no_mangle]
    static mut _ovl_En_JjSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JjSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _ovl_En_JjSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut Bg_Haka_InitVars: ActorInit;
    #[no_mangle]
    fn FaultDrawer_Printf(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FaultDrawer_SetCharPad(_: s8, _: s8);
    #[no_mangle]
    fn Fault_AddClient(_: *mut FaultClient, _: *mut libc::c_void,
                       _: *mut libc::c_void, _: *mut libc::c_void);
    #[no_mangle]
    fn Fault_RemoveClient(_: *mut FaultClient);
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
pub struct Color_RGB8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Color_RGBA8_u32 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub c2rust_unnamed: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
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
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub sides: [C2RustUnnamed_6; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub single: C2RustUnnamed_9,
    pub multi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
    pub restrictions: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub c2rust_unnamed: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
    pub flags: C2RustUnnamed_13,
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
pub struct C2RustUnnamed_13 {
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
pub type C2RustUnnamed_14 = libc::c_uint;
pub const ALLOCTYPE_PERMANENT: C2RustUnnamed_14 = 2;
pub const ALLOCTYPE_ABSOLUTE: C2RustUnnamed_14 = 1;
pub const ALLOCTYPE_NORMAL: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const ACTOR_ID_MAX: C2RustUnnamed_15 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_15 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_15 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_15 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_15 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_15 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_15 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_15 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_15 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_15 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_15 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_15 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_15 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_15 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_15 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_15 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_15 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_15 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_15 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_15 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_15 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_15 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_15 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_15 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_15 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_15 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_15 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_15 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_15 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_15 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_15 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_15 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_15 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_15 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_15 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_15 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_15 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_15 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_15 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_15 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_15 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_15 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_15 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_15 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_15 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_15 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_15 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_15 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_15 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_15 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_15 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_15 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_15 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_15 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_15 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_15 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_15 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_15 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_15 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_15 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_15 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_15 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_15 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_15 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_15 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_15 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_15 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_15 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_15 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_15 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_15 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_15 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_15 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_15 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_15 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_15 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_15 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_15 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_15 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_15 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_15 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_15 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_15 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_15 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_15 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_15 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_15 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_15 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_15 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_15 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_15 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_15 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_15 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_15 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_15 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_15 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_15 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_15 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_15 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_15 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_15 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_15 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_15 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_15 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_15 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_15 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_15 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_15 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_15 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_15 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_15 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_15 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_15 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_15 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_15 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_15 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_15 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_15 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_15 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_15 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_15 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_15 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_15 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_15 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_15 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_15 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_15 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_15 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_15 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_15 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_15 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_15 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_15 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_15 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_15 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_15 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_15 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_15 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_15 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_15 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_15 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_15 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_15 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_15 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_15 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_15 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_15 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_15 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_15 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_15 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_15 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_15 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_15 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_15 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_15 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_15 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_15 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_15 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_15 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_15 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_15 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_15 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_15 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_15 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_15 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_15 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_15 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_15 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_15 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_15 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_15 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_15 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_15 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_15 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_15 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_15 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_15 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_15 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_15 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_15 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_15 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_15 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_15 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_15 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_15 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_15 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_15 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_15 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_15 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_15 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_15 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_15 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_15 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_15 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_15 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_15 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_15 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_15 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_15 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_15 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_15 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_15 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_15 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_15 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_15 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_15 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_15 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_15 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_15 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_15 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_15 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_15 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_15 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_15 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_15 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_15 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_15 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_15 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_15 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_15 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_15 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_15 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_15 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_15 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_15 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_15 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_15 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_15 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_15 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_15 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_15 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_15 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_15 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_15 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_15 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_15 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_15 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_15 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_15 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_15 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_15 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_15 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_15 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_15 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_15 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_15 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_15 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_15 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_15 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_15 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_15 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_15 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_15 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_15 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_15 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_15 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_15 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_15 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_15 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_15 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_15 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_15 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_15 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_15 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_15 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_15 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_15 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_15 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_15 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_15 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_15 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_15 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_15 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_15 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_15 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_15 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_15 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_15 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_15 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_15 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_15 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_15 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_15 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_15 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_15 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_15 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_15 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_15 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_15 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_15 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_15 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_15 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_15 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_15 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_15 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_15 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_15 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_15 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_15 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_15 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_15 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_15 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_15 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_15 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_15 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_15 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_15 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_15 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_15 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_15 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_15 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_15 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_15 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_15 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_15 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_15 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_15 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_15 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_15 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_15 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_15 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_15 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_15 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_15 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_15 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_15 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_15 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_15 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_15 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_15 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_15 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_15 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_15 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_15 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_15 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_15 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_15 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_15 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_15 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_15 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_15 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_15 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_15 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_15 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_15 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_15 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_15 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_15 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_15 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_15 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_15 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_15 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_15 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_15 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_15 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_15 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_15 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_15 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_15 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_15 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_15 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_15 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_15 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_15 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_15 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_15 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_15 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_15 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_15 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_15 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_15 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_15 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_15 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_15 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_15 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_15 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_15 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_15 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_15 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_15 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_15 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_15 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_15 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_15 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_15 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_15 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_15 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_15 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_15 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_15 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_15 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_15 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_15 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_15 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_15 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_15 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_15 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_15 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_15 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_15 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_15 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_15 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_15 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_15 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_15 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_15 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_15 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_15 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_15 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_15 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_15 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_15 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_15 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_15 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_15 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_15 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_15 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_15 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_15 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_15 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_15 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_15 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_15 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_15 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_15 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_15 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_15 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_15 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_15 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_15 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_15 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_15 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_15 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_15 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_15 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_15 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_15 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_15 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_15 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_15 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_15 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_15 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_15 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_15 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_15 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_15 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_15 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_15 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_15 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_15 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_15 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_15 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_15 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_15 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_15 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_15 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_15 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_15 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_15 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_15 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_15 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_15 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_15 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_15 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_15 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_15 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_15 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_15 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_15 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_15 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_15 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultClient {
    pub next: *mut FaultClient,
    pub callback: u32_0,
    pub param1: u32_0,
    pub param2: u32_0,
}
// Linker symbol declarations (used in the table below)
// Init Vars declarations (also used in the table below)
// Actor Overlay Table definition
// Initialized in run_static_initializers
#[no_mangle]
pub static mut gActorOverlayTable: [ActorOverlay; 471] =
    [ActorOverlay{vromStart: 0,
                  vromEnd: 0,
                  vramStart: 0 as *mut libc::c_void,
                  vramEnd: 0 as *mut libc::c_void,
                  loadedRamAddr: 0 as *mut libc::c_void,
                  initInfo: 0 as *mut ActorInit,
                  name: 0 as *mut libc::c_char,
                  allocType: 0,
                  numLoaded: 0,}; 471];
#[no_mangle]
pub static mut gMaxActorId: s32 = 0 as libc::c_int;
static mut sFaultClient: FaultClient =
    FaultClient{next: 0 as *const FaultClient as *mut FaultClient,
                callback: 0,
                param1: 0,
                param2: 0,};
#[no_mangle]
pub unsafe extern "C" fn ActorOverlayTable_LogPrint() {
    let mut overlayEntry: *mut ActorOverlay = 0 as *mut ActorOverlay;
    let mut i: u32_0 = 0;
    osSyncPrintf(b"actor_dlftbls %u\n\x00" as *const u8 as
                     *const libc::c_char, gMaxActorId);
    osSyncPrintf(b"RomStart RomEnd   SegStart SegEnd   allocp   profile  segname\n\x00"
                     as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as u32_0;
    overlayEntry =
        &mut *gActorOverlayTable.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
            *mut ActorOverlay;
    while i < gMaxActorId as u32_0 {
        osSyncPrintf(b"%08x %08x %08x %08x %08x %08x %s\n\x00" as *const u8 as
                         *const libc::c_char, (*overlayEntry).vromStart,
                     (*overlayEntry).vromEnd, (*overlayEntry).vramStart,
                     (*overlayEntry).vramEnd, (*overlayEntry).loadedRamAddr,
                     &mut (*(*overlayEntry).initInfo).id as *mut s16,
                     if !(*overlayEntry).name.is_null() {
                         (*overlayEntry).name as *const libc::c_char
                     } else { b"?\x00" as *const u8 as *const libc::c_char });
        i = i.wrapping_add(1);
        overlayEntry = overlayEntry.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ActorOverlayTable_FaultPrint(mut arg0:
                                                          *mut libc::c_void,
                                                      mut arg1:
                                                          *mut libc::c_void) {
    let mut overlayEntry: *mut ActorOverlay = 0 as *mut ActorOverlay;
    let mut overlaySize: u32_0 = 0;
    let mut i: s32 = 0;
    FaultDrawer_SetCharPad(-(2 as libc::c_int) as s8, 0 as libc::c_int as s8);
    FaultDrawer_Printf(b"actor_dlftbls %u\n\x00" as *const u8 as
                           *const libc::c_char, gMaxActorId);
    FaultDrawer_Printf(b"No. RamStart- RamEnd cn  Name\n\x00" as *const u8 as
                           *const libc::c_char);
    i = 0 as libc::c_int;
    overlayEntry =
        &mut *gActorOverlayTable.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
            *mut ActorOverlay;
    while i < gMaxActorId {
        overlaySize =
            ((*overlayEntry).vramEnd as
                 u32_0).wrapping_sub((*overlayEntry).vramStart as u32_0);
        if !(*overlayEntry).loadedRamAddr.is_null() {
            FaultDrawer_Printf(b"%3d %08x-%08x %3d %s\n\x00" as *const u8 as
                                   *const libc::c_char, i,
                               (*overlayEntry).loadedRamAddr,
                               ((*overlayEntry).loadedRamAddr as
                                    u32_0).wrapping_add(overlaySize),
                               (*overlayEntry).numLoaded as libc::c_int,
                               if !(*overlayEntry).name.is_null() {
                                   (*overlayEntry).name as *const libc::c_char
                               } else {
                                   b"\x00" as *const u8 as *const libc::c_char
                               });
        }
        i += 1;
        overlayEntry = overlayEntry.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ActorOverlayTable_Init() {
    gMaxActorId = ACTOR_ID_MAX as libc::c_int;
    Fault_AddClient(&mut sFaultClient,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> ()>,
                                            *mut libc::c_void>(Some(ActorOverlayTable_FaultPrint
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut libc::c_void,
                                                                                             _:
                                                                                                 *mut libc::c_void)
                                                                            ->
                                                                                ())),
                    0 as *mut libc::c_void, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ActorOverlayTable_Cleanup() {
    Fault_RemoveClient(&mut sFaultClient);
    gMaxActorId = 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    gActorOverlayTable =
        [{
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0 as libc::c_int as u32_0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Player_InitVars,
                              name:
                                  b"Player\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TestSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TestSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TestSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TestSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Test_InitVars,
                              name:
                                  b"En_Test\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GirlASegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GirlASegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GirlASegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GirlASegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_GirlA_InitVars,
                              name:
                                  b"En_GirlA\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_PartSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_PartSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_PartSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_PartSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Part_InitVars,
                              name:
                                  b"En_Part\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_LightSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_LightSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_LightSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_LightSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Light_InitVars,
                              name:
                                  b"En_Light\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DoorSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DoorSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DoorSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DoorSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Door_InitVars,
                              name:
                                  b"En_Door\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BoxSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BoxSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BoxSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BoxSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Box_InitVars,
                              name:
                                  b"En_Box\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Dy_YoseizoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Dy_YoseizoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Dy_YoseizoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Dy_YoseizoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Dy_Yoseizo_InitVars,
                              name:
                                  b"Bg_Dy_Yoseizo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_FirewallSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_FirewallSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_FirewallSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_FirewallSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Firewall_InitVars,
                              name:
                                  b"Bg_Hidan_Firewall\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_PohSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_PohSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_PohSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_PohSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Poh_InitVars,
                              name:
                                  b"En_Poh\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_OkutaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_OkutaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_OkutaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_OkutaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Okuta_InitVars,
                              name:
                                  b"En_Okuta\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ydan_SpSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ydan_SpSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_Ydan_SpSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ydan_SpSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ydan_Sp_InitVars,
                              name:
                                  b"Bg_Ydan_Sp\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BomSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BomSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BomSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BomSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bom_InitVars,
                              name:
                                  b"En_Bom\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_WallmasSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_WallmasSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_WallmasSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_WallmasSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Wallmas_InitVars,
                              name:
                                  b"En_Wallmas\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DodongoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_DodongoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DodongoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DodongoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dodongo_InitVars,
                              name:
                                  b"En_Dodongo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FireflySegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_FireflySegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_FireflySegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FireflySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Firefly_InitVars,
                              name:
                                  b"En_Firefly\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_HorseSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_HorseSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_HorseSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_HorseSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Horse_InitVars,
                              name:
                                  b"En_Horse\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0 as libc::c_int as u32_0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Item00_InitVars,
                              name:
                                  b"En_Item00\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ArrowSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ArrowSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ArrowSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ArrowSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Arrow_InitVars,
                              name:
                                  b"En_Arrow\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ElfSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ElfSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ElfSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ElfSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Elf_InitVars,
                              name:
                                  b"En_Elf\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_NiwSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_NiwSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_NiwSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_NiwSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Niw_InitVars,
                              name:
                                  b"En_Niw\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TiteSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TiteSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TiteSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TiteSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Tite_InitVars,
                              name:
                                  b"En_Tite\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ReebaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ReebaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ReebaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ReebaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Reeba_InitVars,
                              name:
                                  b"En_Reeba\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_PeehatSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_PeehatSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_PeehatSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_PeehatSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Peehat_InitVars,
                              name:
                                  b"En_Peehat\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ButteSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ButteSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ButteSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ButteSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Butte_InitVars,
                              name:
                                  b"En_Butte\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_InsectSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_InsectSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_InsectSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_InsectSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Insect_InitVars,
                              name:
                                  b"En_Insect\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FishSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_FishSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_FishSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FishSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fish_InitVars,
                              name:
                                  b"En_Fish\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_HollSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_HollSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_HollSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_HollSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Holl_InitVars,
                              name:
                                  b"En_Holl\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Scene_ChangeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Scene_ChangeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Scene_ChangeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Scene_ChangeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Scene_Change_InitVars,
                              name:
                                  b"En_Scene_Change\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ZfSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ZfSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ZfSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ZfSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Zf_InitVars,
                              name:
                                  b"En_Zf\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_HataSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_HataSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_HataSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_HataSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Hata_InitVars,
                              name:
                                  b"En_Hata\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_DodongoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Boss_DodongoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Boss_DodongoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_DodongoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Dodongo_InitVars,
                              name:
                                  b"Boss_Dodongo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_GomaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Boss_GomaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_GomaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_GomaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Goma_InitVars,
                              name:
                                  b"Boss_Goma\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Zl1SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Zl1SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Zl1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Zl1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Zl1_InitVars,
                              name:
                                  b"En_Zl1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ViewerSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_ViewerSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ViewerSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ViewerSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Viewer_InitVars,
                              name:
                                  b"En_Viewer\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GomaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GomaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GomaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GomaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Goma_InitVars,
                              name:
                                  b"En_Goma\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_PushboxSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_PushboxSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_PushboxSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_PushboxSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Pushbox_InitVars,
                              name:
                                  b"Bg_Pushbox\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BubbleSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_BubbleSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BubbleSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BubbleSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bubble_InitVars,
                              name:
                                  b"En_Bubble\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Door_ShutterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Door_ShutterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Door_ShutterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Door_ShutterSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Door_Shutter_InitVars,
                              name:
                                  b"Door_Shutter\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DodojrSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_DodojrSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DodojrSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DodojrSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dodojr_InitVars,
                              name:
                                  b"En_Dodojr\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BdfireSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_BdfireSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BdfireSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BdfireSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bdfire_InitVars,
                              name:
                                  b"En_Bdfire\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BoomSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BoomSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BoomSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BoomSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Boom_InitVars,
                              name:
                                  b"En_Boom\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Torch2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Torch2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Torch2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Torch2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Torch2_InitVars,
                              name:
                                  b"En_Torch2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BiliSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BiliSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BiliSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BiliSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bili_InitVars,
                              name:
                                  b"En_Bili\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TpSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TpSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TpSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TpSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Tp_InitVars,
                              name:
                                  b"En_Tp\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_StSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_StSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_StSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_StSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_St_InitVars,
                              name:
                                  b"En_St\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BwSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BwSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BwSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BwSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bw_InitVars,
                              name:
                                  b"En_Bw\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0 as libc::c_int as u32_0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_A_Obj_InitVars,
                              name:
                                  b"En_A_Obj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_EiyerSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_EiyerSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_EiyerSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_EiyerSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Eiyer_InitVars,
                              name:
                                  b"En_Eiyer\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_River_SoundSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_River_SoundSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_River_SoundSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_River_SoundSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_River_Sound_InitVars,
                              name:
                                  b"En_River_Sound\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Horse_NormalSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Horse_NormalSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Horse_NormalSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Horse_NormalSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Horse_Normal_InitVars,
                              name:
                                  b"En_Horse_Normal\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_OssanSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_OssanSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_OssanSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_OssanSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ossan_InitVars,
                              name:
                                  b"En_Ossan\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_TreemouthSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_TreemouthSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_TreemouthSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_TreemouthSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Treemouth_InitVars,
                              name:
                                  b"Bg_Treemouth\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_DodoagoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_DodoagoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_DodoagoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_DodoagoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Dodoago_InitVars,
                              name:
                                  b"Bg_Dodoago\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_DalmSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_DalmSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_DalmSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_DalmSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Dalm_InitVars,
                              name:
                                  b"Bg_Hidan_Dalm\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_HrockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_HrockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_HrockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_HrockSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Hrock_InitVars,
                              name:
                                  b"Bg_Hidan_Hrock\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Horse_GanonSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Horse_GanonSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Horse_GanonSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Horse_GanonSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Horse_Ganon_InitVars,
                              name:
                                  b"En_Horse_Ganon\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_RockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_RockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_RockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_RockSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Rock_InitVars,
                              name:
                                  b"Bg_Hidan_Rock\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_RsekizouSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_RsekizouSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_RsekizouSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_RsekizouSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Rsekizou_InitVars,
                              name:
                                  b"Bg_Hidan_Rsekizou\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_SekizouSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_SekizouSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_SekizouSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_SekizouSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Sekizou_InitVars,
                              name:
                                  b"Bg_Hidan_Sekizou\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_SimaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_SimaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_SimaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_SimaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Sima_InitVars,
                              name:
                                  b"Bg_Hidan_Sima\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_SyokuSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_SyokuSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_SyokuSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_SyokuSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Syoku_InitVars,
                              name:
                                  b"Bg_Hidan_Syoku\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_XcSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_XcSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_XcSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_XcSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Xc_InitVars,
                              name:
                                  b"En_Xc\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_CurtainSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_CurtainSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_CurtainSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_CurtainSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Curtain_InitVars,
                              name:
                                  b"Bg_Hidan_Curtain\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot00_HanebasiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot00_HanebasiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot00_HanebasiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot00_HanebasiSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot00_Hanebasi_InitVars,
                              name:
                                  b"Bg_Spot00_Hanebasi\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_MbSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_MbSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_MbSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_MbSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Mb_InitVars,
                              name:
                                  b"En_Mb\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BombfSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BombfSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BombfSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BombfSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bombf_InitVars,
                              name:
                                  b"En_Bombf\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Zl2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Zl2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Zl2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Zl2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Zl2_InitVars,
                              name:
                                  b"En_Zl2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_FsliftSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_FsliftSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_FsliftSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_FsliftSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Fslift_InitVars,
                              name:
                                  b"Bg_Hidan_Fslift\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_OE2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_OE2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_OE2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_OE2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_OE2_InitVars,
                              name:
                                  b"En_OE2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ydan_HasiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ydan_HasiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Ydan_HasiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ydan_HasiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ydan_Hasi_InitVars,
                              name:
                                  b"Bg_Ydan_Hasi\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ydan_MarutaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ydan_MarutaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Ydan_MarutaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ydan_MarutaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ydan_Maruta_InitVars,
                              name:
                                  b"Bg_Ydan_Maruta\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_GanondrofSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Boss_GanondrofSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Boss_GanondrofSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_GanondrofSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Ganondrof_InitVars,
                              name:
                                  b"Boss_Ganondrof\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_AmSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_AmSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_AmSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_AmSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Am_InitVars,
                              name:
                                  b"En_Am\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DekubabaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_DekubabaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_DekubabaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DekubabaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dekubaba_InitVars,
                              name:
                                  b"En_Dekubaba\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_M_Fire1SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_M_Fire1SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_M_Fire1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_M_Fire1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_M_Fire1_InitVars,
                              name:
                                  b"En_M_Fire1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_M_ThunderSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_M_ThunderSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_M_ThunderSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_M_ThunderSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_M_Thunder_InitVars,
                              name:
                                  b"En_M_Thunder\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ddan_JdSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ddan_JdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_Ddan_JdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ddan_JdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ddan_Jd_InitVars,
                              name:
                                  b"Bg_Ddan_Jd\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_BreakwallSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_BreakwallSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_BreakwallSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_BreakwallSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Breakwall_InitVars,
                              name:
                                  b"Bg_Breakwall\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_JjSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_JjSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_JjSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_JjSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Jj_InitVars,
                              name:
                                  b"En_Jj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Horse_ZeldaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Horse_ZeldaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Horse_ZeldaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Horse_ZeldaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Horse_Zelda_InitVars,
                              name:
                                  b"En_Horse_Zelda\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ddan_KdSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ddan_KdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_Ddan_KdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ddan_KdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ddan_Kd_InitVars,
                              name:
                                  b"Bg_Ddan_Kd\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Door_Warp1SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Door_Warp1SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Door_Warp1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Door_Warp1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Door_Warp1_InitVars,
                              name:
                                  b"Door_Warp1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_SyokudaiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_SyokudaiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_SyokudaiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_SyokudaiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Syokudai_InitVars,
                              name:
                                  b"Obj_Syokudai\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Item_B_HeartSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Item_B_HeartSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Item_B_HeartSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Item_B_HeartSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Item_B_Heart_InitVars,
                              name:
                                  b"Item_B_Heart\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DekunutsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_DekunutsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_DekunutsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DekunutsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dekunuts_InitVars,
                              name:
                                  b"En_Dekunuts\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Menkuri_KaitenSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Menkuri_KaitenSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Menkuri_KaitenSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Menkuri_KaitenSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Menkuri_Kaiten_InitVars,
                              name:
                                  b"Bg_Menkuri_Kaiten\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Menkuri_EyeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Menkuri_EyeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Menkuri_EyeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Menkuri_EyeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Menkuri_Eye_InitVars,
                              name:
                                  b"Bg_Menkuri_Eye\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ValiSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ValiSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ValiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ValiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Vali_InitVars,
                              name:
                                  b"En_Vali\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mizu_MovebgSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mizu_MovebgSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mizu_MovebgSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mizu_MovebgSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mizu_Movebg_InitVars,
                              name:
                                  b"Bg_Mizu_Movebg\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mizu_WaterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mizu_WaterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mizu_WaterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mizu_WaterSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mizu_Water_InitVars,
                              name:
                                  b"Bg_Mizu_Water\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Arms_HookSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Arms_HookSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Arms_HookSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Arms_HookSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Arms_Hook_InitVars,
                              name:
                                  b"Arms_Hook\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_PERMANENT as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_fHGSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_fHGSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_fHGSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_fHGSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_fHG_InitVars,
                              name:
                                  b"En_fHG\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_HineriSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_HineriSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_HineriSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_HineriSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Hineri_InitVars,
                              name:
                                  b"Bg_Mori_Hineri\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BbSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BbSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BbSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BbSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bb_InitVars,
                              name:
                                  b"En_Bb\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Toki_HikariSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Toki_HikariSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Toki_HikariSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Toki_HikariSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Toki_Hikari_InitVars,
                              name:
                                  b"Bg_Toki_Hikari\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_YukabyunSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_YukabyunSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_YukabyunSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_YukabyunSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Yukabyun_InitVars,
                              name:
                                  b"En_Yukabyun\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Toki_SwdSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Toki_SwdSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Toki_SwdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Toki_SwdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Toki_Swd_InitVars,
                              name:
                                  b"Bg_Toki_Swd\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Fhg_FireSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Fhg_FireSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Fhg_FireSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Fhg_FireSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fhg_Fire_InitVars,
                              name:
                                  b"En_Fhg_Fire\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_MjinSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Bg_MjinSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_MjinSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_MjinSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mjin_InitVars,
                              name:
                                  b"Bg_Mjin\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_KousiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_KousiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_KousiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_KousiSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Kousi_InitVars,
                              name:
                                  b"Bg_Hidan_Kousi\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Door_TokiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Door_TokiSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Door_TokiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Door_TokiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Door_Toki_InitVars,
                              name:
                                  b"Door_Toki\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_HamstepSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_HamstepSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_HamstepSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_HamstepSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Hamstep_InitVars,
                              name:
                                  b"Bg_Hidan_Hamstep\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BirdSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BirdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BirdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BirdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bird_InitVars,
                              name:
                                  b"En_Bird\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Wood02SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Wood02SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Wood02SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Wood02SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Wood02_InitVars,
                              name:
                                  b"En_Wood02\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_LightboxSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_LightboxSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_LightboxSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_LightboxSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Lightbox_InitVars,
                              name:
                                  b"En_Lightbox\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Pu_boxSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Pu_boxSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Pu_boxSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Pu_boxSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Pu_box_InitVars,
                              name:
                                  b"En_Pu_box\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TrapSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TrapSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TrapSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TrapSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Trap_InitVars,
                              name:
                                  b"En_Trap\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Arow_TrapSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Arow_TrapSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Arow_TrapSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Arow_TrapSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Arow_Trap_InitVars,
                              name:
                                  b"En_Arow_Trap\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_VaseSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_VaseSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_VaseSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_VaseSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Vase_InitVars,
                              name:
                                  b"En_Vase\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ta_InitVars,
                              name:
                                  b"En_Ta\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TkSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TkSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TkSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TkSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Tk_InitVars,
                              name:
                                  b"En_Tk\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_BigstSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_BigstSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_BigstSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_BigstSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Bigst_InitVars,
                              name:
                                  b"Bg_Mori_Bigst\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_ElevatorSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_ElevatorSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_ElevatorSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_ElevatorSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Elevator_InitVars,
                              name:
                                  b"Bg_Mori_Elevator\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_KaitenkabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_KaitenkabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_KaitenkabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_KaitenkabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Kaitenkabe_InitVars,
                              name:
                                  b"Bg_Mori_Kaitenkabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_RakkatenjoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_RakkatenjoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_RakkatenjoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_RakkatenjoSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Rakkatenjo_InitVars,
                              name:
                                  b"Bg_Mori_Rakkatenjo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_VmSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_VmSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_VmSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_VmSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Vm_InitVars,
                              name:
                                  b"En_Vm\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_EffectSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Demo_EffectSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Demo_EffectSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_EffectSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Effect_InitVars,
                              name:
                                  b"Demo_Effect\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_KankyoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Demo_KankyoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Demo_KankyoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_KankyoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Kankyo_InitVars,
                              name:
                                  b"Demo_Kankyo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_FwbigSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_FwbigSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_FwbigSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_FwbigSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Fwbig_InitVars,
                              name:
                                  b"Bg_Hidan_Fwbig\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FloormasSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_FloormasSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_FloormasSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FloormasSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Floormas_InitVars,
                              name:
                                  b"En_Floormas\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Heishi1SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Heishi1SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Heishi1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Heishi1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Heishi1_InitVars,
                              name:
                                  b"En_Heishi1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_RdSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_RdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_RdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_RdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Rd_InitVars,
                              name:
                                  b"En_Rd\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Po_SistersSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Po_SistersSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Po_SistersSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Po_SistersSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Po_Sisters_InitVars,
                              name:
                                  b"En_Po_Sisters\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Heavy_BlockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Heavy_BlockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Heavy_BlockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Heavy_BlockSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Heavy_Block_InitVars,
                              name:
                                  b"Bg_Heavy_Block\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Po_EventSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Po_EventSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Po_EventSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Po_EventSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Po_Event_InitVars,
                              name:
                                  b"Bg_Po_Event\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_MureSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Obj_MureSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_MureSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_MureSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Mure_InitVars,
                              name:
                                  b"Obj_Mure\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SwSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SwSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SwSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SwSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Sw_InitVars,
                              name:
                                  b"En_Sw\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_FdSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Boss_FdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_FdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_FdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Fd_InitVars,
                              name:
                                  b"Boss_Fd\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Object_KankyoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Object_KankyoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Object_KankyoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Object_KankyoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Object_Kankyo_InitVars,
                              name:
                                  b"Object_Kankyo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DuSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DuSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DuSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DuSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Du_InitVars,
                              name:
                                  b"En_Du\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FdSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_FdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_FdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fd_InitVars,
                              name:
                                  b"En_Fd\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Horse_Link_ChildSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Horse_Link_ChildSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Horse_Link_ChildSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Horse_Link_ChildSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Horse_Link_Child_InitVars,
                              name:
                                  b"En_Horse_Link_Child\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Door_AnaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Door_AnaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Door_AnaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Door_AnaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Door_Ana_InitVars,
                              name:
                                  b"Door_Ana\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot02_ObjectsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot02_ObjectsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot02_ObjectsSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot02_ObjectsSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot02_Objects_InitVars,
                              name:
                                  b"Bg_Spot02_Objects\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_HakaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Bg_HakaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_HakaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_HakaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_InitVars,
                              name:
                                  b"Bg_Haka\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Magic_WindSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Magic_WindSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Magic_WindSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Magic_WindSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Magic_Wind_InitVars,
                              name:
                                  b"Magic_Wind\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Magic_FireSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Magic_FireSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Magic_FireSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Magic_FireSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Magic_Fire_InitVars,
                              name:
                                  b"Magic_Fire\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ru1SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ru1SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ru1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ru1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ru1_InitVars,
                              name:
                                  b"En_Ru1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_Fd2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Boss_Fd2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_Fd2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_Fd2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Fd2_InitVars,
                              name:
                                  b"Boss_Fd2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Fd_FireSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Fd_FireSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Fd_FireSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Fd_FireSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fd_Fire_InitVars,
                              name:
                                  b"En_Fd_Fire\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DhSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DhSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DhSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DhSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dh_InitVars,
                              name:
                                  b"En_Dh\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DhaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DhaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DhaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DhaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dha_InitVars,
                              name:
                                  b"En_Dha\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_RlSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_RlSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_RlSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_RlSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Rl_InitVars,
                              name:
                                  b"En_Rl\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Encount1SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Encount1SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Encount1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Encount1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Encount1_InitVars,
                              name:
                                  b"En_Encount1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_DuSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_DuSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_DuSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_DuSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Du_InitVars,
                              name:
                                  b"Demo_Du\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_ImSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_ImSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_ImSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_ImSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Im_InitVars,
                              name:
                                  b"Demo_Im\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_Tre_LgtSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Demo_Tre_LgtSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Demo_Tre_LgtSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_Tre_LgtSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Tre_Lgt_InitVars,
                              name:
                                  b"Demo_Tre_Lgt\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FwSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_FwSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_FwSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FwSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fw_InitVars,
                              name:
                                  b"En_Fw\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Vb_SimaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Vb_SimaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_Vb_SimaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Vb_SimaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Vb_Sima_InitVars,
                              name:
                                  b"Bg_Vb_Sima\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Vb_BallSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Vb_BallSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Vb_BallSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Vb_BallSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Vb_Ball_InitVars,
                              name:
                                  b"En_Vb_Ball\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_MeganeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_MeganeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_MeganeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_MeganeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Megane_InitVars,
                              name:
                                  b"Bg_Haka_Megane\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_MeganeBGSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_MeganeBGSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_MeganeBGSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_MeganeBGSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_MeganeBG_InitVars,
                              name:
                                  b"Bg_Haka_MeganeBG\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_ShipSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_ShipSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_ShipSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_ShipSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Ship_InitVars,
                              name:
                                  b"Bg_Haka_Ship\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_SgamiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_SgamiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_SgamiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_SgamiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Sgami_InitVars,
                              name:
                                  b"Bg_Haka_Sgami\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Heishi2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Heishi2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Heishi2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Heishi2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Heishi2_InitVars,
                              name:
                                  b"En_Heishi2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Encount2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Encount2SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Encount2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Encount2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Encount2_InitVars,
                              name:
                                  b"En_Encount2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Fire_RockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Fire_RockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Fire_RockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Fire_RockSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fire_Rock_InitVars,
                              name:
                                  b"En_Fire_Rock\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BrobSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BrobSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BrobSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BrobSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Brob_InitVars,
                              name:
                                  b"En_Brob\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Mir_RaySegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Mir_RaySegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Mir_RaySegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Mir_RaySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Mir_Ray_InitVars,
                              name:
                                  b"Mir_Ray\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot09_ObjSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot09_ObjSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot09_ObjSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot09_ObjSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot09_Obj_InitVars,
                              name:
                                  b"Bg_Spot09_Obj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot18_ObjSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot18_ObjSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot18_ObjSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot18_ObjSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot18_Obj_InitVars,
                              name:
                                  b"Bg_Spot18_Obj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_VaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Boss_VaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_VaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_VaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Va_InitVars,
                              name:
                                  b"Boss_Va\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_TuboSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_TuboSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_TuboSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_TuboSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Tubo_InitVars,
                              name:
                                  b"Bg_Haka_Tubo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_TrapSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_TrapSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_TrapSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_TrapSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Trap_InitVars,
                              name:
                                  b"Bg_Haka_Trap\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_HutaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_HutaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_HutaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_HutaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Huta_InitVars,
                              name:
                                  b"Bg_Haka_Huta\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_ZouSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_ZouSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_ZouSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_ZouSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Zou_InitVars,
                              name:
                                  b"Bg_Haka_Zou\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot17_FunenSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot17_FunenSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot17_FunenSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot17_FunenSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot17_Funen_InitVars,
                              name:
                                  b"Bg_Spot17_Funen\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Syateki_ItmSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Syateki_ItmSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Syateki_ItmSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Syateki_ItmSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Syateki_Itm_InitVars,
                              name:
                                  b"En_Syateki_Itm\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Syateki_ManSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Syateki_ManSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Syateki_ManSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Syateki_ManSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Syateki_Man_InitVars,
                              name:
                                  b"En_Syateki_Man\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TanaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TanaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TanaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TanaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Tana_InitVars,
                              name:
                                  b"En_Tana\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_NbSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_NbSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_NbSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_NbSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Nb_InitVars,
                              name:
                                  b"En_Nb\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_MoSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Boss_MoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_MoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_MoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Mo_InitVars,
                              name:
                                  b"Boss_Mo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SbSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SbSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SbSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SbSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Sb_InitVars,
                              name:
                                  b"En_Sb\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BigokutaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_BigokutaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_BigokutaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BigokutaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bigokuta_InitVars,
                              name:
                                  b"En_Bigokuta\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_KarebabaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_KarebabaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_KarebabaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_KarebabaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Karebaba_InitVars,
                              name:
                                  b"En_Karebaba\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Bdan_ObjectsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Bdan_ObjectsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Bdan_ObjectsSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Bdan_ObjectsSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Bdan_Objects_InitVars,
                              name:
                                  b"Bg_Bdan_Objects\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_SaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_SaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_SaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_SaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Sa_InitVars,
                              name:
                                  b"Demo_Sa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_GoSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_GoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_GoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_GoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Go_InitVars,
                              name:
                                  b"Demo_Go\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_InSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_InSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_InSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_InSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_In_InitVars,
                              name:
                                  b"En_In\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TrSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TrSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TrSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TrSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Tr_InitVars,
                              name:
                                  b"En_Tr\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot16_BombstoneSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot16_BombstoneSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot16_BombstoneSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot16_BombstoneSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot16_Bombstone_InitVars,
                              name:
                                  b"Bg_Spot16_Bombstone\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Hidan_KowarerukabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Hidan_KowarerukabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Hidan_KowarerukabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Hidan_KowarerukabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Hidan_Kowarerukabe_InitVars,
                              name:
                                  b"Bg_Hidan_Kowarerukabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_BombwallSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_BombwallSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_BombwallSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_BombwallSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Bombwall_InitVars,
                              name:
                                  b"Bg_Bombwall\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot08_IceblockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot08_IceblockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot08_IceblockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot08_IceblockSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot08_Iceblock_InitVars,
                              name:
                                  b"Bg_Spot08_Iceblock\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ru2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ru2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ru2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ru2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ru2_InitVars,
                              name:
                                  b"En_Ru2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_DekujrSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_DekujrSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_DekujrSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_DekujrSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Dekujr_InitVars,
                              name:
                                  b"Obj_Dekujr\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mizu_UzuSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mizu_UzuSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mizu_UzuSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mizu_UzuSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mizu_Uzu_InitVars,
                              name:
                                  b"Bg_Mizu_Uzu\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot06_ObjectsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot06_ObjectsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot06_ObjectsSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot06_ObjectsSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot06_Objects_InitVars,
                              name:
                                  b"Bg_Spot06_Objects\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ice_ObjectsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ice_ObjectsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Ice_ObjectsSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ice_ObjectsSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ice_Objects_InitVars,
                              name:
                                  b"Bg_Ice_Objects\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_WaterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_WaterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_WaterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_WaterSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Water_InitVars,
                              name:
                                  b"Bg_Haka_Water\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ma2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ma2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ma2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ma2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ma2_InitVars,
                              name:
                                  b"En_Ma2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Bom_ChuSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Bom_ChuSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Bom_ChuSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Bom_ChuSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bom_Chu_InitVars,
                              name:
                                  b"En_Bom_Chu\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Horse_Game_CheckSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Horse_Game_CheckSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Horse_Game_CheckSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Horse_Game_CheckSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Horse_Game_Check_InitVars,
                              name:
                                  b"En_Horse_Game_Check\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_TwSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Boss_TwSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_TwSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_TwSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Tw_InitVars,
                              name:
                                  b"Boss_Tw\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_RrSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_RrSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_RrSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_RrSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Rr_InitVars,
                              name:
                                  b"En_Rr\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ba_InitVars,
                              name:
                                  b"En_Ba\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BxSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_BxSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BxSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BxSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bx_InitVars,
                              name:
                                  b"En_Bx\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_AnubiceSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_AnubiceSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_AnubiceSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_AnubiceSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Anubice_InitVars,
                              name:
                                  b"En_Anubice\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Anubice_FireSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Anubice_FireSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Anubice_FireSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Anubice_FireSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Anubice_Fire_InitVars,
                              name:
                                  b"En_Anubice_Fire\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_HashigoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_HashigoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_HashigoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_HashigoSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Hashigo_InitVars,
                              name:
                                  b"Bg_Mori_Hashigo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_Hashira4SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_Hashira4SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_Hashira4SegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_Hashira4SegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Hashira4_InitVars,
                              name:
                                  b"Bg_Mori_Hashira4\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mori_IdomizuSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mori_IdomizuSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mori_IdomizuSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mori_IdomizuSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mori_Idomizu_InitVars,
                              name:
                                  b"Bg_Mori_Idomizu\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot16_DoughnutSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot16_DoughnutSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot16_DoughnutSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot16_DoughnutSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot16_Doughnut_InitVars,
                              name:
                                  b"Bg_Spot16_Doughnut\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Bdan_SwitchSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Bdan_SwitchSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Bdan_SwitchSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Bdan_SwitchSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Bdan_Switch_InitVars,
                              name:
                                  b"Bg_Bdan_Switch\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ma1SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ma1SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ma1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ma1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ma1_InitVars,
                              name:
                                  b"En_Ma1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_GanonSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Boss_GanonSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_GanonSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_GanonSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Ganon_InitVars,
                              name:
                                  b"Boss_Ganon\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_SstSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Boss_SstSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Boss_SstSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_SstSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Sst_InitVars,
                              name:
                                  b"Boss_Sst\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_NySegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_NySegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_NySegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_NySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ny_InitVars,
                              name:
                                  b"En_Ny\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FrSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_FrSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_FrSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FrSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fr_InitVars,
                              name:
                                  b"En_Fr\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Item_ShieldSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Item_ShieldSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Item_ShieldSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Item_ShieldSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Item_Shield_InitVars,
                              name:
                                  b"Item_Shield\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ice_ShelterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ice_ShelterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Ice_ShelterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ice_ShelterSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ice_Shelter_InitVars,
                              name:
                                  b"Bg_Ice_Shelter\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ice_HonoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Ice_HonoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Ice_HonoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ice_HonoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ice_Hono_InitVars,
                              name:
                                  b"En_Ice_Hono\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Item_OcarinaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Item_OcarinaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Item_OcarinaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Item_OcarinaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Item_Ocarina_InitVars,
                              name:
                                  b"Item_Ocarina\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Magic_DarkSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Magic_DarkSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Magic_DarkSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Magic_DarkSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Magic_Dark_InitVars,
                              name:
                                  b"Magic_Dark\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_6KSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_6KSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_6KSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_6KSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_6K_InitVars,
                              name:
                                  b"Demo_6K\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Anubice_TagSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Anubice_TagSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Anubice_TagSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Anubice_TagSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Anubice_Tag_InitVars,
                              name:
                                  b"En_Anubice_Tag\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Haka_GateSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Haka_GateSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Haka_GateSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Haka_GateSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Haka_Gate_InitVars,
                              name:
                                  b"Bg_Haka_Gate\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot15_SakuSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot15_SakuSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot15_SakuSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot15_SakuSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot15_Saku_InitVars,
                              name:
                                  b"Bg_Spot15_Saku\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_GoroiwaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_GoroiwaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_GoroiwaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_GoroiwaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Goroiwa_InitVars,
                              name:
                                  b"Bg_Jya_Goroiwa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_ZurerukabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_ZurerukabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_ZurerukabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_ZurerukabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Zurerukabe_InitVars,
                              name:
                                  b"Bg_Jya_Zurerukabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_CobraSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_CobraSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_CobraSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_CobraSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Cobra_InitVars,
                              name:
                                  b"Bg_Jya_Cobra\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_KanaamiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_KanaamiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_KanaamiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_KanaamiSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Kanaami_InitVars,
                              name:
                                  b"Bg_Jya_Kanaami\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_FishingSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_FishingSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_FishingSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_FishingSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Fishing_InitVars,
                              name:
                                  b"Fishing\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_OshihikiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_OshihikiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_OshihikiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_OshihikiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Oshihiki_InitVars,
                              name:
                                  b"Obj_Oshihiki\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Gate_ShutterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Gate_ShutterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Gate_ShutterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Gate_ShutterSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Gate_Shutter_InitVars,
                              name:
                                  b"Bg_Gate_Shutter\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Eff_DustSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Eff_DustSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Eff_DustSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Eff_DustSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Eff_Dust_InitVars,
                              name:
                                  b"Eff_Dust\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot01_FusyaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot01_FusyaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot01_FusyaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot01_FusyaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot01_Fusya_InitVars,
                              name:
                                  b"Bg_Spot01_Fusya\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot01_IdohashiraSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot01_IdohashiraSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot01_IdohashiraSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot01_IdohashiraSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot01_Idohashira_InitVars,
                              name:
                                  b"Bg_Spot01_Idohashira\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot01_IdomizuSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot01_IdomizuSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot01_IdomizuSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot01_IdomizuSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot01_Idomizu_InitVars,
                              name:
                                  b"Bg_Spot01_Idomizu\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Po_SyokudaiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Po_SyokudaiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Po_SyokudaiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Po_SyokudaiSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Po_Syokudai_InitVars,
                              name:
                                  b"Bg_Po_Syokudai\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ganon_OtyukaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ganon_OtyukaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Ganon_OtyukaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ganon_OtyukaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ganon_Otyuka_InitVars,
                              name:
                                  b"Bg_Ganon_Otyuka\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot15_RrboxSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot15_RrboxSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot15_RrboxSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot15_RrboxSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot15_Rrbox_InitVars,
                              name:
                                  b"Bg_Spot15_Rrbox\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_UmajumpSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_UmajumpSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_UmajumpSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_UmajumpSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Umajump_InitVars,
                              name:
                                  b"Bg_Umajump\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Arrow_FireSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Arrow_FireSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Arrow_FireSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Arrow_FireSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Arrow_Fire_InitVars,
                              name:
                                  b"Arrow_Fire\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Arrow_IceSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Arrow_IceSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Arrow_IceSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Arrow_IceSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Arrow_Ice_InitVars,
                              name:
                                  b"Arrow_Ice\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Arrow_LightSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Arrow_LightSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Arrow_LightSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Arrow_LightSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Arrow_Light_InitVars,
                              name:
                                  b"Arrow_Light\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Item_EtceteraSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Item_EtceteraSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Item_EtceteraSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Item_EtceteraSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Item_Etcetera_InitVars,
                              name:
                                  b"Item_Etcetera\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_KibakoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_KibakoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_KibakoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_KibakoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Kibako_InitVars,
                              name:
                                  b"Obj_Kibako\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_TsuboSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_TsuboSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_TsuboSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_TsuboSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Tsubo_InitVars,
                              name:
                                  b"Obj_Tsubo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Wonder_ItemSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Wonder_ItemSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Wonder_ItemSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Wonder_ItemSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Wonder_Item_InitVars,
                              name:
                                  b"En_Wonder_Item\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_IkSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_IkSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_IkSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_IkSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ik_InitVars,
                              name:
                                  b"En_Ik\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_IkSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_IkSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_IkSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_IkSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Ik_InitVars,
                              name:
                                  b"Demo_Ik\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SkjSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SkjSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SkjSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SkjSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Skj_InitVars,
                              name:
                                  b"En_Skj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SkjneedleSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_SkjneedleSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_SkjneedleSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SkjneedleSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Skjneedle_InitVars,
                              name:
                                  b"En_Skjneedle\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_G_SwitchSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_G_SwitchSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_G_SwitchSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_G_SwitchSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_G_Switch_InitVars,
                              name:
                                  b"En_G_Switch\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_ExtSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_ExtSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_ExtSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_ExtSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Ext_InitVars,
                              name:
                                  b"Demo_Ext\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_ShdSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_ShdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_ShdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_ShdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Shd_InitVars,
                              name:
                                  b"Demo_Shd\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DnsSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DnsSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DnsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DnsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dns_InitVars,
                              name:
                                  b"En_Dns\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Elf_MsgSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Elf_MsgSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Elf_MsgSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Elf_MsgSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Elf_Msg_InitVars,
                              name:
                                  b"Elf_Msg\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_HonotrapSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_HonotrapSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_HonotrapSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_HonotrapSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Honotrap_InitVars,
                              name:
                                  b"En_Honotrap\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Tubo_TrapSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Tubo_TrapSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Tubo_TrapSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Tubo_TrapSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Tubo_Trap_InitVars,
                              name:
                                  b"En_Tubo_Trap\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_Ice_PolySegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_Ice_PolySegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_Ice_PolySegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_Ice_PolySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Ice_Poly_InitVars,
                              name:
                                  b"Obj_Ice_Poly\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot03_TakiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot03_TakiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot03_TakiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot03_TakiSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot03_Taki_InitVars,
                              name:
                                  b"Bg_Spot03_Taki\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot07_TakiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot07_TakiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot07_TakiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot07_TakiSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot07_Taki_InitVars,
                              name:
                                  b"Bg_Spot07_Taki\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FzSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_FzSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_FzSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FzSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fz_InitVars,
                              name:
                                  b"En_Fz\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Po_RelaySegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Po_RelaySegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Po_RelaySegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Po_RelaySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Po_Relay_InitVars,
                              name:
                                  b"En_Po_Relay\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Relay_ObjectsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Relay_ObjectsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Relay_ObjectsSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Relay_ObjectsSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Relay_Objects_InitVars,
                              name:
                                  b"Bg_Relay_Objects\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Diving_GameSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Diving_GameSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Diving_GameSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Diving_GameSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Diving_Game_InitVars,
                              name:
                                  b"En_Diving_Game\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_KusaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_KusaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_KusaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_KusaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Kusa_InitVars,
                              name:
                                  b"En_Kusa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_BeanSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Obj_BeanSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_BeanSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_BeanSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Bean_InitVars,
                              name:
                                  b"Obj_Bean\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_BombiwaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_BombiwaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_BombiwaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_BombiwaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Bombiwa_InitVars,
                              name:
                                  b"Obj_Bombiwa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_SwitchSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_SwitchSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_SwitchSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_SwitchSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Switch_InitVars,
                              name:
                                  b"Obj_Switch\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_ElevatorSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_ElevatorSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_ElevatorSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_ElevatorSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Elevator_InitVars,
                              name:
                                  b"Obj_Elevator\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_LiftSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Obj_LiftSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_LiftSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_LiftSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Lift_InitVars,
                              name:
                                  b"Obj_Lift\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_HsblockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_HsblockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_HsblockSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_HsblockSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Hsblock_InitVars,
                              name:
                                  b"Obj_Hsblock\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Okarina_TagSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Okarina_TagSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Okarina_TagSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Okarina_TagSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Okarina_Tag_InitVars,
                              name:
                                  b"En_Okarina_Tag\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Yabusame_MarkSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Yabusame_MarkSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Yabusame_MarkSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Yabusame_MarkSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Yabusame_Mark_InitVars,
                              name:
                                  b"En_Yabusame_Mark\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GoroiwaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_GoroiwaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GoroiwaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GoroiwaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Goroiwa_InitVars,
                              name:
                                  b"En_Goroiwa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ex_RuppySegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Ex_RuppySegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Ex_RuppySegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ex_RuppySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ex_Ruppy_InitVars,
                              name:
                                  b"En_Ex_Ruppy\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ToryoSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ToryoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ToryoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ToryoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Toryo_InitVars,
                              name:
                                  b"En_Toryo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DaikuSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DaikuSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DaikuSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DaikuSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Daiku_InitVars,
                              name:
                                  b"En_Daiku\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_NwcSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_NwcSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_NwcSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_NwcSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Nwc_InitVars,
                              name:
                                  b"En_Nwc\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_BlkobjSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_BlkobjSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_BlkobjSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_BlkobjSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Blkobj_InitVars,
                              name:
                                  b"En_Blkobj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Item_InboxSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Item_InboxSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Item_InboxSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Item_InboxSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Item_Inbox_InitVars,
                              name:
                                  b"Item_Inbox\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ge1SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ge1SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ge1SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ge1SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ge1_InitVars,
                              name:
                                  b"En_Ge1\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_BlockstopSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_BlockstopSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_BlockstopSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_BlockstopSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Blockstop_InitVars,
                              name:
                                  b"Obj_Blockstop\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SdaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SdaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SdaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SdaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Sda_InitVars,
                              name:
                                  b"En_Sda\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Clear_TagSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Clear_TagSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Clear_TagSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Clear_TagSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Clear_Tag_InitVars,
                              name:
                                  b"En_Clear_Tag\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Niw_LadySegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Niw_LadySegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Niw_LadySegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Niw_LadySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Niw_Lady_InitVars,
                              name:
                                  b"En_Niw_Lady\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GmSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GmSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GmSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GmSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Gm_InitVars,
                              name:
                                  b"En_Gm\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_MsSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_MsSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_MsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_MsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ms_InitVars,
                              name:
                                  b"En_Ms\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_HsSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_HsSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_HsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_HsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Hs_InitVars,
                              name:
                                  b"En_Hs\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_IngateSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_IngateSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_IngateSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_IngateSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ingate_InitVars,
                              name:
                                  b"Bg_Ingate\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_KanbanSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_KanbanSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_KanbanSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_KanbanSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Kanban_InitVars,
                              name:
                                  b"En_Kanban\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Heishi3SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Heishi3SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Heishi3SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Heishi3SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Heishi3_InitVars,
                              name:
                                  b"En_Heishi3\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Syateki_NiwSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Syateki_NiwSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Syateki_NiwSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Syateki_NiwSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Syateki_Niw_InitVars,
                              name:
                                  b"En_Syateki_Niw\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Attack_NiwSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Attack_NiwSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Attack_NiwSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Attack_NiwSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Attack_Niw_InitVars,
                              name:
                                  b"En_Attack_Niw\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot01_IdosokoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot01_IdosokoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot01_IdosokoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot01_IdosokoSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot01_Idosoko_InitVars,
                              name:
                                  b"Bg_Spot01_Idosoko\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Sa_InitVars,
                              name:
                                  b"En_Sa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Wonder_TalkSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Wonder_TalkSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Wonder_TalkSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Wonder_TalkSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Wonder_Talk_InitVars,
                              name:
                                  b"En_Wonder_Talk\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Gjyo_BridgeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Gjyo_BridgeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Gjyo_BridgeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Gjyo_BridgeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Gjyo_Bridge_InitVars,
                              name:
                                  b"Bg_Gjyo_Bridge\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DsSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DsSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ds_InitVars,
                              name:
                                  b"En_Ds\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_MkSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_MkSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_MkSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_MkSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Mk_InitVars,
                              name:
                                  b"En_Mk\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Bom_Bowl_ManSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Bom_Bowl_ManSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Bom_Bowl_ManSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Bom_Bowl_ManSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bom_Bowl_Man_InitVars,
                              name:
                                  b"En_Bom_Bowl_Man\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Bom_Bowl_PitSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Bom_Bowl_PitSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Bom_Bowl_PitSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Bom_Bowl_PitSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Bom_Bowl_Pit_InitVars,
                              name:
                                  b"En_Bom_Bowl_Pit\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_OwlSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_OwlSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_OwlSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_OwlSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Owl_InitVars,
                              name:
                                  b"En_Owl\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_IshiSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_IshiSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_IshiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_IshiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ishi_InitVars,
                              name:
                                  b"En_Ishi\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_HanaSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Obj_HanaSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_HanaSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_HanaSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Hana_InitVars,
                              name:
                                  b"Obj_Hana\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_LightswitchSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_LightswitchSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_LightswitchSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_LightswitchSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Lightswitch_InitVars,
                              name:
                                  b"Obj_Lightswitch\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_Mure2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_Mure2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_Mure2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_Mure2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Mure2_InitVars,
                              name:
                                  b"Obj_Mure2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GoSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Go_InitVars,
                              name:
                                  b"En_Go\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_FuSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_FuSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_FuSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_FuSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Fu_InitVars,
                              name:
                                  b"En_Fu\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ChangerSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_ChangerSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ChangerSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ChangerSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Changer_InitVars,
                              name:
                                  b"En_Changer\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_MegamiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_MegamiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_MegamiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_MegamiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Megami_InitVars,
                              name:
                                  b"Bg_Jya_Megami\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_LiftSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_LiftSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_LiftSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_LiftSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Lift_InitVars,
                              name:
                                  b"Bg_Jya_Lift\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_BigmirrorSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_BigmirrorSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_BigmirrorSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_BigmirrorSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Bigmirror_InitVars,
                              name:
                                  b"Bg_Jya_Bigmirror\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_BombchuiwaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_BombchuiwaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_BombchuiwaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_BombchuiwaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Bombchuiwa_InitVars,
                              name:
                                  b"Bg_Jya_Bombchuiwa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_AmishutterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_AmishutterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_AmishutterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_AmishutterSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Amishutter_InitVars,
                              name:
                                  b"Bg_Jya_Amishutter\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_BombiwaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_BombiwaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_BombiwaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_BombiwaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Bombiwa_InitVars,
                              name:
                                  b"Bg_Jya_Bombiwa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot18_BasketSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot18_BasketSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot18_BasketSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot18_BasketSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot18_Basket_InitVars,
                              name:
                                  b"Bg_Spot18_Basket\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ganon_OrganSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Ganon_OrganSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Ganon_OrganSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ganon_OrganSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ganon_Organ_InitVars,
                              name:
                                  b"En_Ganon_Organ\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SiofukiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_SiofukiSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SiofukiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SiofukiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Siofuki_InitVars,
                              name:
                                  b"En_Siofuki\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_StreamSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_StreamSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_StreamSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_StreamSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Stream_InitVars,
                              name:
                                  b"En_Stream\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_MmSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_MmSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_MmSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_MmSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Mm_InitVars,
                              name:
                                  b"En_Mm\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_KoSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_KoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_KoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_KoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ko_InitVars,
                              name:
                                  b"En_Ko\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_KzSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_KzSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_KzSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_KzSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Kz_InitVars,
                              name:
                                  b"En_Kz\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Weather_TagSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Weather_TagSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Weather_TagSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Weather_TagSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Weather_Tag_InitVars,
                              name:
                                  b"En_Weather_Tag\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Sst_FloorSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Sst_FloorSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Sst_FloorSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Sst_FloorSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Sst_Floor_InitVars,
                              name:
                                  b"Bg_Sst_Floor\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_AniSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_AniSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_AniSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_AniSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ani_InitVars,
                              name:
                                  b"En_Ani\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ex_ItemSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Ex_ItemSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ex_ItemSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ex_ItemSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ex_Item_InitVars,
                              name:
                                  b"En_Ex_Item\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_IronobjSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_IronobjSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_IronobjSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_IronobjSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Ironobj_InitVars,
                              name:
                                  b"Bg_Jya_Ironobj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_JsSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_JsSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_JsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_JsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Js_InitVars,
                              name:
                                  b"En_Js\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_JsjutanSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_JsjutanSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_JsjutanSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_JsjutanSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Jsjutan_InitVars,
                              name:
                                  b"En_Jsjutan\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_CsSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_CsSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_CsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_CsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Cs_InitVars,
                              name:
                                  b"En_Cs\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_MdSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_MdSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_MdSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_MdSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Md_InitVars,
                              name:
                                  b"En_Md\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_HySegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_HySegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_HySegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_HySegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Hy_InitVars,
                              name:
                                  b"En_Hy\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ganon_MantSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Ganon_MantSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Ganon_MantSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ganon_MantSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ganon_Mant_InitVars,
                              name:
                                  b"En_Ganon_Mant\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Okarina_EffectSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Okarina_EffectSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Okarina_EffectSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Okarina_EffectSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Okarina_Effect_InitVars,
                              name:
                                  b"En_Okarina_Effect\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_MagSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_MagSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_MagSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_MagSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Mag_InitVars,
                              name:
                                  b"En_Mag\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Door_GerudoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Door_GerudoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Door_GerudoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Door_GerudoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Door_Gerudo_InitVars,
                              name:
                                  b"Door_Gerudo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Elf_Msg2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Elf_Msg2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Elf_Msg2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Elf_Msg2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Elf_Msg2_InitVars,
                              name:
                                  b"Elf_Msg2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_GtSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_GtSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_GtSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_GtSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Gt_InitVars,
                              name:
                                  b"Demo_Gt\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Po_FieldSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Po_FieldSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Po_FieldSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Po_FieldSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Po_Field_InitVars,
                              name:
                                  b"En_Po_Field\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Efc_ErupcSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Efc_ErupcSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Efc_ErupcSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Efc_ErupcSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Efc_Erupc_InitVars,
                              name:
                                  b"Efc_Erupc\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_ZgSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Bg_ZgSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Bg_ZgSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_ZgSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Zg_InitVars,
                              name:
                                  b"Bg_Zg\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Heishi4SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Heishi4SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Heishi4SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Heishi4SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Heishi4_InitVars,
                              name:
                                  b"En_Heishi4\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Zl3SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Zl3SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Zl3SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Zl3SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Zl3_InitVars,
                              name:
                                  b"En_Zl3\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Boss_Ganon2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Boss_Ganon2SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Boss_Ganon2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Boss_Ganon2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Boss_Ganon2_InitVars,
                              name:
                                  b"Boss_Ganon2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_KakasiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_KakasiSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_KakasiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_KakasiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Kakasi_InitVars,
                              name:
                                  b"En_Kakasi\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Takara_ManSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Takara_ManSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Takara_ManSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Takara_ManSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Takara_Man_InitVars,
                              name:
                                  b"En_Takara_Man\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_MakeoshihikiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_MakeoshihikiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_MakeoshihikiSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_MakeoshihikiSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Makeoshihiki_InitVars,
                              name:
                                  b"Obj_Makeoshihiki\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Oceff_SpotSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Oceff_SpotSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Oceff_SpotSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Oceff_SpotSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Oceff_Spot_InitVars,
                              name:
                                  b"Oceff_Spot\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_End_TitleSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_End_TitleSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_End_TitleSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_End_TitleSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut End_Title_InitVars,
                              name:
                                  b"End_Title\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TorchSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TorchSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TorchSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TorchSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Torch_InitVars,
                              name:
                                  b"En_Torch\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_EcSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_EcSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_EcSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_EcSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Ec_InitVars,
                              name:
                                  b"Demo_Ec\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Shot_SunSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Shot_SunSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Shot_SunSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Shot_SunSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Shot_Sun_InitVars,
                              name:
                                  b"Shot_Sun\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Dy_ExtraSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Dy_ExtraSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Dy_ExtraSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Dy_ExtraSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dy_Extra_InitVars,
                              name:
                                  b"En_Dy_Extra\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Wonder_Talk2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Wonder_Talk2SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Wonder_Talk2SegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Wonder_Talk2SegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Wonder_Talk2_InitVars,
                              name:
                                  b"En_Wonder_Talk2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ge2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ge2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ge2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ge2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ge2_InitVars,
                              name:
                                  b"En_Ge2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_RoomtimerSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_RoomtimerSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_RoomtimerSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_RoomtimerSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Roomtimer_InitVars,
                              name:
                                  b"Obj_Roomtimer\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SshSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SshSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SshSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SshSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ssh_InitVars,
                              name:
                                  b"En_Ssh\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SthSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SthSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SthSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SthSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Sth_InitVars,
                              name:
                                  b"En_Sth\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Oceff_WipeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Oceff_WipeSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Oceff_WipeSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Oceff_WipeSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Oceff_Wipe_InitVars,
                              name:
                                  b"Oceff_Wipe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Oceff_StormSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Oceff_StormSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Oceff_StormSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Oceff_StormSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Oceff_Storm_InitVars,
                              name:
                                  b"Oceff_Storm\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_WeiyerSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_WeiyerSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_WeiyerSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_WeiyerSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Weiyer_InitVars,
                              name:
                                  b"En_Weiyer\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot05_SokoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot05_SokoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot05_SokoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot05_SokoSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot05_Soko_InitVars,
                              name:
                                  b"Bg_Spot05_Soko\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_1fliftSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_1fliftSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_1fliftSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_1fliftSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_1flift_InitVars,
                              name:
                                  b"Bg_Jya_1flift\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_HahenironSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_HahenironSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_HahenironSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_HahenironSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Haheniron_InitVars,
                              name:
                                  b"Bg_Jya_Haheniron\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot12_GateSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot12_GateSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot12_GateSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot12_GateSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot12_Gate_InitVars,
                              name:
                                  b"Bg_Spot12_Gate\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot12_SakuSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot12_SakuSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot12_SakuSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot12_SakuSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot12_Saku_InitVars,
                              name:
                                  b"Bg_Spot12_Saku\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_HintnutsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_HintnutsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_HintnutsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_HintnutsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Hintnuts_InitVars,
                              name:
                                  b"En_Hintnuts\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_NutsballSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_NutsballSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_NutsballSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_NutsballSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Nutsball_InitVars,
                              name:
                                  b"En_Nutsball\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot00_BreakSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot00_BreakSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot00_BreakSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot00_BreakSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot00_Break_InitVars,
                              name:
                                  b"Bg_Spot00_Break\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ShopnutsSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_ShopnutsSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_ShopnutsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ShopnutsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Shopnuts_InitVars,
                              name:
                                  b"En_Shopnuts\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ItSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ItSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ItSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ItSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_It_InitVars,
                              name:
                                  b"En_It\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GeldBSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GeldBSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GeldBSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GeldBSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_GeldB_InitVars,
                              name:
                                  b"En_GeldB\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Oceff_Wipe2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Oceff_Wipe2SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Oceff_Wipe2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Oceff_Wipe2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Oceff_Wipe2_InitVars,
                              name:
                                  b"Oceff_Wipe2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Oceff_Wipe3SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Oceff_Wipe3SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Oceff_Wipe3SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Oceff_Wipe3SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Oceff_Wipe3_InitVars,
                              name:
                                  b"Oceff_Wipe3\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Niw_GirlSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Niw_GirlSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Niw_GirlSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Niw_GirlSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Niw_Girl_InitVars,
                              name:
                                  b"En_Niw_Girl\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_DogSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_DogSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_DogSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_DogSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dog_InitVars,
                              name:
                                  b"En_Dog\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SiSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SiSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Si_InitVars,
                              name:
                                  b"En_Si\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot01_Objects2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot01_Objects2SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot01_Objects2SegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot01_Objects2SegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot01_Objects2_InitVars,
                              name:
                                  b"Bg_Spot01_Objects2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_CombSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Obj_CombSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_CombSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_CombSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Comb_InitVars,
                              name:
                                  b"Obj_Comb\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot11_BakudankabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot11_BakudankabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot11_BakudankabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot11_BakudankabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot11_Bakudankabe_InitVars,
                              name:
                                  b"Bg_Spot11_Bakudankabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_Kibako2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_Kibako2SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_Kibako2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_Kibako2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Kibako2_InitVars,
                              name:
                                  b"Obj_Kibako2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Dnt_DemoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Dnt_DemoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Dnt_DemoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Dnt_DemoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dnt_Demo_InitVars,
                              name:
                                  b"En_Dnt_Demo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Dnt_JijiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Dnt_JijiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Dnt_JijiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Dnt_JijiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dnt_Jiji_InitVars,
                              name:
                                  b"En_Dnt_Jiji\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Dnt_NomalSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Dnt_NomalSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Dnt_NomalSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Dnt_NomalSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Dnt_Nomal_InitVars,
                              name:
                                  b"En_Dnt_Nomal\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GuestSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GuestSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GuestSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GuestSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Guest_InitVars,
                              name:
                                  b"En_Guest\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Bom_GuardSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Bom_GuardSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Bom_GuardSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Bom_GuardSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Bom_Guard_InitVars,
                              name:
                                  b"Bg_Bom_Guard\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Hs2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Hs2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Hs2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Hs2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Hs2_InitVars,
                              name:
                                  b"En_Hs2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_KekkaiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Demo_KekkaiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Demo_KekkaiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_KekkaiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Kekkai_InitVars,
                              name:
                                  b"Demo_Kekkai\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot08_BakudankabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot08_BakudankabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot08_BakudankabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot08_BakudankabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot08_Bakudankabe_InitVars,
                              name:
                                  b"Bg_Spot08_Bakudankabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot17_BakudankabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot17_BakudankabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot17_BakudankabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot17_BakudankabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot17_Bakudankabe_InitVars,
                              name:
                                  b"Bg_Spot17_Bakudankabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart: 0 as libc::c_int as u32_0,
                              vromEnd: 0,
                              vramStart: 0 as *mut libc::c_void,
                              vramEnd: 0 as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: 0 as *mut ActorInit,
                              name: 0 as *mut libc::c_char,
                              allocType: 0,
                              numLoaded: 0,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_Mure3SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_Mure3SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Obj_Mure3SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_Mure3SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Mure3_InitVars,
                              name:
                                  b"Obj_Mure3\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_TgSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_TgSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_TgSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_TgSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Tg_InitVars,
                              name:
                                  b"En_Tg\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_MuSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_MuSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_MuSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_MuSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Mu_InitVars,
                              name:
                                  b"En_Mu\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Go2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Go2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Go2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Go2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Go2_InitVars,
                              name:
                                  b"En_Go2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_WfSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_WfSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_WfSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_WfSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Wf_InitVars,
                              name:
                                  b"En_Wf\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_SkbSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_SkbSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_SkbSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_SkbSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Skb_InitVars,
                              name:
                                  b"En_Skb\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_GjSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_Demo_GjSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_GjSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_GjSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Gj_InitVars,
                              name:
                                  b"Demo_Gj\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Demo_GeffSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Demo_GeffSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_Demo_GeffSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Demo_GeffSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Demo_Geff_InitVars,
                              name:
                                  b"Demo_Geff\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Gnd_FiremeiroSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Gnd_FiremeiroSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Gnd_FiremeiroSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Gnd_FiremeiroSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Gnd_Firemeiro_InitVars,
                              name:
                                  b"Bg_Gnd_Firemeiro\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Gnd_DarkmeiroSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Gnd_DarkmeiroSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Gnd_DarkmeiroSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Gnd_DarkmeiroSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Gnd_Darkmeiro_InitVars,
                              name:
                                  b"Bg_Gnd_Darkmeiro\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Gnd_SoulmeiroSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Gnd_SoulmeiroSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Gnd_SoulmeiroSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Gnd_SoulmeiroSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Gnd_Soulmeiro_InitVars,
                              name:
                                  b"Bg_Gnd_Soulmeiro\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Gnd_NisekabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Gnd_NisekabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Gnd_NisekabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Gnd_NisekabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Gnd_Nisekabe_InitVars,
                              name:
                                  b"Bg_Gnd_Nisekabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Gnd_IceblockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Gnd_IceblockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Gnd_IceblockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Gnd_IceblockSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Gnd_Iceblock_InitVars,
                              name:
                                  b"Bg_Gnd_Iceblock\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GbSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GbSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GbSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GbSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Gb_InitVars,
                              name:
                                  b"En_Gb\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_GsSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_GsSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_GsSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_GsSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Gs_InitVars,
                              name:
                                  b"En_Gs\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mizu_BwallSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mizu_BwallSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mizu_BwallSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mizu_BwallSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mizu_Bwall_InitVars,
                              name:
                                  b"Bg_Mizu_Bwall\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Mizu_ShutterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Mizu_ShutterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Mizu_ShutterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Mizu_ShutterSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Mizu_Shutter_InitVars,
                              name:
                                  b"Bg_Mizu_Shutter\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Daiku_KakarikoSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Daiku_KakarikoSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Daiku_KakarikoSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Daiku_KakarikoSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Daiku_Kakariko_InitVars,
                              name:
                                  b"En_Daiku_Kakariko\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Bowl_WallSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Bowl_WallSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Bowl_WallSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Bowl_WallSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Bowl_Wall_InitVars,
                              name:
                                  b"Bg_Bowl_Wall\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Wall_TuboSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Wall_TuboSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Wall_TuboSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Wall_TuboSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Wall_Tubo_InitVars,
                              name:
                                  b"En_Wall_Tubo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Po_DesertSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Po_DesertSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_En_Po_DesertSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Po_DesertSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Po_Desert_InitVars,
                              name:
                                  b"En_Po_Desert\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_CrowSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_CrowSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_CrowSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_CrowSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Crow_InitVars,
                              name:
                                  b"En_Crow\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Door_KillerSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Door_KillerSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Door_KillerSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Door_KillerSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Door_Killer_InitVars,
                              name:
                                  b"Door_Killer\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot11_OasisSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot11_OasisSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot11_OasisSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot11_OasisSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot11_Oasis_InitVars,
                              name:
                                  b"Bg_Spot11_Oasis\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot18_FutaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot18_FutaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot18_FutaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot18_FutaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot18_Futa_InitVars,
                              name:
                                  b"Bg_Spot18_Futa\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Spot18_ShutterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Spot18_ShutterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Spot18_ShutterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Spot18_ShutterSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Spot18_Shutter_InitVars,
                              name:
                                  b"Bg_Spot18_Shutter\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ma3SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ma3SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ma3SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ma3SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ma3_InitVars,
                              name:
                                  b"En_Ma3\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_CowSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_CowSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_CowSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_CowSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Cow_InitVars,
                              name:
                                  b"En_Cow\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ice_TuraraSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ice_TuraraSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Ice_TuraraSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ice_TuraraSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ice_Turara_InitVars,
                              name:
                                  b"Bg_Ice_Turara\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Ice_ShutterSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Ice_ShutterSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Ice_ShutterSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Ice_ShutterSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Ice_Shutter_InitVars,
                              name:
                                  b"Bg_Ice_Shutter\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Kakasi2SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Kakasi2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Kakasi2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Kakasi2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Kakasi2_InitVars,
                              name:
                                  b"En_Kakasi2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Kakasi3SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_En_Kakasi3SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Kakasi3SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Kakasi3SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Kakasi3_InitVars,
                              name:
                                  b"En_Kakasi3\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Oceff_Wipe4SegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Oceff_Wipe4SegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Oceff_Wipe4SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Oceff_Wipe4SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Oceff_Wipe4_InitVars,
                              name:
                                  b"Oceff_Wipe4\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_ABSOLUTE as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_EgSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_EgSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_EgSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_EgSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Eg_InitVars,
                              name:
                                  b"En_Eg\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Menkuri_NisekabeSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Menkuri_NisekabeSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Menkuri_NisekabeSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Menkuri_NisekabeSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Menkuri_Nisekabe_InitVars,
                              name:
                                  b"Bg_Menkuri_Nisekabe\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_ZoSegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_ZoSegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_ZoSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_ZoSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Zo_InitVars,
                              name:
                                  b"En_Zo\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_MakekinsutaSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_MakekinsutaSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_MakekinsutaSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_MakekinsutaSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Makekinsuta_InitVars,
                              name:
                                  b"Obj_Makekinsuta\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Ge3SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Ge3SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Ge3SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Ge3SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Ge3_InitVars,
                              name:
                                  b"En_Ge3\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_TimeblockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_TimeblockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_TimeblockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_TimeblockSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Timeblock_InitVars,
                              name:
                                  b"Obj_Timeblock\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_HamishiSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_HamishiSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_HamishiSegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_HamishiSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Hamishi_InitVars,
                              name:
                                  b"Obj_Hamishi\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Zl4SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Zl4SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Zl4SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Zl4SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Zl4_InitVars,
                              name:
                                  b"En_Zl4\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_En_Mm2SegmentRomStart.as_mut_ptr() as
                                      u32_0,
                              vromEnd:
                                  _ovl_En_Mm2SegmentRomEnd.as_mut_ptr() as
                                      u32_0,
                              vramStart:
                                  _ovl_En_Mm2SegmentStart.as_mut_ptr() as
                                      *mut libc::c_void,
                              vramEnd:
                                  _ovl_En_Mm2SegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut En_Mm2_InitVars,
                              name:
                                  b"En_Mm2\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Bg_Jya_BlockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Bg_Jya_BlockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Bg_Jya_BlockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Bg_Jya_BlockSegmentEnd.as_mut_ptr() as
                                      *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Bg_Jya_Block_InitVars,
                              name:
                                  b"Bg_Jya_Block\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         },
         {
             let mut init =
                 ActorOverlay{vromStart:
                                  _ovl_Obj_Warp2blockSegmentRomStart.as_mut_ptr()
                                      as u32_0,
                              vromEnd:
                                  _ovl_Obj_Warp2blockSegmentRomEnd.as_mut_ptr()
                                      as u32_0,
                              vramStart:
                                  _ovl_Obj_Warp2blockSegmentStart.as_mut_ptr()
                                      as *mut libc::c_void,
                              vramEnd:
                                  _ovl_Obj_Warp2blockSegmentEnd.as_mut_ptr()
                                      as *mut libc::c_void,
                              loadedRamAddr: 0 as *mut libc::c_void,
                              initInfo: &mut Obj_Warp2block_InitVars,
                              name:
                                  b"Obj_Warp2block\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              allocType:
                                  ALLOCTYPE_NORMAL as libc::c_int as u16_0,
                              numLoaded: 0 as libc::c_int as s8,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
