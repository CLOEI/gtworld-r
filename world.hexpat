import std.io;
import std.mem;

enum WeatherType : u16 {
    Default = 0,
    Sunset = 1,
    Night = 2,
    Desert = 3,
    Sunny = 4,
    RainyCity = 5,
    Harvest = 6,
    Mars = 7,
    Spooky = 8,
    Maw = 9,
    Blank = 10,
    Snowy = 11,
    Growch = 12,
    GrowchHappy = 13,
    Undersea = 14,
    Warp = 15,
    Comet = 16,
    Comet2 = 17,
    Party = 18,
    Pineapple = 19,
    SnowyNight = 20,
    Spring = 21,
    Wolf = 22,
    NotInitialized = 23,
    PurpleHaze = 24,
    FireHaze = 25,
    GreenHaze = 26,
    AquaHaze = 27,
    CustomHaze = 28,
    CustomItems = 29,
    Pagoda = 30,
    Apocalypse = 31,
    Jungle = 32,
    BalloonWarz = 33,
    Background = 34,
    Autumn = 35,
    Hearth = 36,
    StPatricks = 37,
    IceAge = 38,
    Volcano = 39,
    FloatingIslands = 40,
    Mascot = 41,
    DigitalRain = 42,
    MonoChrome = 43,
    Treasure = 44,
    Surgery = 45,
    Bountiful = 46,
    Meteor = 47,
    Stars = 48,
    Ascended = 49,
    Destroyed = 50,
    GrowtopiaSign = 51,
    Dungeon = 52,
    LegendaryCity = 53,
    BloodDragon = 54,
    PopCity = 55,
    Anzu = 56,
    TmntCity = 57,
    RadCity = 58,
    Plaze = 59,
    Nebula = 60,
    ProtoStar = 61,
    DarkMountains = 62,
    Ac15 = 63,
    MountGrowMore = 64,
    CrackInReality = 65,
    LnyNian = 66,
    RaymanLock = 67,
    Steampunk = 68,
    RealmOfSpirits = 69,
    Blackhole = 70,
    Gems = 71,
    HolidayHaven = 72,
    FenyxLock = 73,
    EnchantedLock = 74,
    RoyalEnchantedLock = 75,
    NeptunesAtlantis = 76,
    PinuskiPetalPerfectHaven = 77,
    Candyland = 78
};

bitfield TileFlags {
    HAS_EXTRA_DATA : 1;
    HAS_PARENT : 1;
    WAS_SPLICED : 1;
    WILL_SPAWN_SEEDS_TOO : 1;
    IS_SEEDLING : 1;
    FLIPPED_X : 1;
    IS_ON : 1;
    IS_OPEN_TO_PUBLIC : 1;
    BG_IS_ON : 1;
    FG_ALT_MODE : 1;
    IS_WET : 1;
    GLUED : 1;
    ON_FIRE : 1;
    PAINTED_RED : 1;
    PAINTED_GREEN : 1;
    PAINTED_BLUE : 1;
};

struct FishInfo {
    u32 fish_item_id;
    u32 lbs;
};

struct SilkWormColor {
    u8 a;
    u8 r;
    u8 g;
    u8 b;
};

struct StorageBlockItemInfo {
    padding[3];
    u32 id;
    padding[2];
    u32 amount;
};

struct CookingOvenIngredientInfo {
    u32 item_id;
    u32 time_added;
};

struct CyBotCommandData {
    u32 command_id;
    u32 is_command_used;
    padding[7];
};

struct TileExtraData {
    u8 extra_tile_type;

    if (extra_tile_type == 1) { // Sign
        u16 text_len;
        char text[text_len];
        u8 flags;
    }
    else if (extra_tile_type == 2) { // Door
        u16 text_len;
        char text[text_len];
        u32 owner_uid;
    }
    else if (extra_tile_type == 3) { // Lock
        u8 settings;
        u32 owner_uid;
        u32 access_count;
        u32 access_uids[access_count];
        u8 minimum_level;
        u8 unknown_1[7];
    }
    else if (extra_tile_type == 4) { // Seed
        u32 time_passed;
        u8 item_on_tree;
    }
    else if (extra_tile_type == 6) { // Mailbox
        u16 str_len_1;
        char unknown_1[str_len_1];
        u16 str_len_2;
        char unknown_2[str_len_2];
        u16 str_len_3;
        char unknown_3[str_len_3];
        u8 unknown_4;
    }
    else if (extra_tile_type == 7) { // Bulletin
        u16 str_len_1;
        char unknown_1[str_len_1];
        u16 str_len_2;
        char unknown_2[str_len_2];
        u16 str_len_3;
        char unknown_3[str_len_3];
        u8 unknown_4;
    }
    else if (extra_tile_type == 8) { // Dice
        u8 symbol;
    }
    else if (extra_tile_type == 9) { // ChemicalSource
        u32 time_passed;
    }
    else if (extra_tile_type == 10) { // AchievementBlock
        u32 unknown_1;
        u8 tile_type;
    }
    else if (extra_tile_type == 11) { // HearthMonitor
        u32 unknown_1;
        u16 player_name_len;
        char player_name[player_name_len];
    }
    else if (extra_tile_type == 12) { // DonationBox
        u16 str_len_1;
        char unknown_1[str_len_1];
        u16 str_len_2;
        char unknown_2[str_len_2];
        u16 str_len_3;
        char unknown_3[str_len_3];
        u8 unknown_4;
    }
    else if (extra_tile_type == 14) { // Mannequin
        u16 text_len;
        char text[text_len];
        u8 unknown_1;
        u32 clothing_1;
        u16 clothing_2;
        u16 clothing_3;
        u16 clothing_4;
        u16 clothing_5;
        u16 clothing_6;
        u16 clothing_7;
        u16 clothing_8;
        u16 clothing_9;
        u16 clothing_10;
    }
    else if (extra_tile_type == 15) { // BunnyEgg
        u32 egg_placed;
    }
    else if (extra_tile_type == 16) { // GamePack
        u8 team;
    }
    else if (extra_tile_type == 17) { // GameGenerator
        // Empty
    }
    else if (extra_tile_type == 18) { // XenoniteCrystal
        u8 unknown_1;
        u32 unknown_2;
    }
    else if (extra_tile_type == 19) { // PhoneBooth
        u16 clothing_1;
        u16 clothing_2;
        u16 clothing_3;
        u16 clothing_4;
        u16 clothing_5;
        u16 clothing_6;
        u16 clothing_7;
        u16 clothing_8;
        u16 clothing_9;
    }
    else if (extra_tile_type == 20) { // Crystal
        u16 str_len;
        char unknown_1[str_len];
    }
    else if (extra_tile_type == 21) { // CrimeInProgress
        u16 str_len;
        char unknown_1[str_len];
        u32 unknown_2;
        u8 unknown_3;
    }
    else if (extra_tile_type == 23) { // DisplayBlock
        u32 item_id;
    }
    else if (extra_tile_type == 24) { // VendingMachine
        u32 item_id;
        s32 price;
    }
    else if (extra_tile_type == 25) { // FishTankPort
        u8 flags;
        u32 fish_count;
        FishInfo fishes[fish_count / 2];
    }
    else if (extra_tile_type == 26) { // SolarCollector
        u8 unknown_1[5];
    }
    else if (extra_tile_type == 27) { // Forge
        u32 temperature;
    }
    else if (extra_tile_type == 28) { // GivingTree
        u16 unknown_1;
        u32 unknown_2;
    }
    else if (extra_tile_type == 30) { // SteamOrgan
        u8 instrument_type;
        u32 note;
    }
    else if (extra_tile_type == 31) { // SilkWorm
        u8 type_;
        u16 name_len;
        char name[name_len];
        u32 age;
        u32 unknown_1;
        u32 unknown_2;
        u8 can_be_fed;
        u32 color;
        u32 sick_duration;
    }
    else if (extra_tile_type == 32) { // SewingMachine
        u16 bolt_len;
        u32 bolt_id_list[bolt_len];
    }
    else if (extra_tile_type == 33) { // CountryFlag
        u16 country_len;
        char country[country_len];
    }
    else if (extra_tile_type == 34) { // LobsterTrap
        // Empty
    }
    else if (extra_tile_type == 35) { // PaintingEasel
        u32 item_id;
        u16 label_len;
        char label[label_len];
    }
    else if (extra_tile_type == 36) { // PetBattleCage
        u16 label_len;
        char label[label_len];
        u32 base_pet;
        u32 combined_pet_1;
        u32 combined_pet_2;
    }
    else if (extra_tile_type == 37) { // PetTrainer
        u16 name_len;
        char name[name_len];
        u32 pet_total_count;
        u32 unknown_1;
        u32 pets_id[pet_total_count];
    }
    else if (extra_tile_type == 38) { // SteamEngine
        u32 temperature;
    }
    else if (extra_tile_type == 39) { // LockBot
        u32 time_passed;
    }
    else if (extra_tile_type == 40) { // WeatherMachine
        u32 settings;
    }
    else if (extra_tile_type == 41) { // SpiritStorageUnit
        u32 ghost_jar_count;
    }
    else if (extra_tile_type == 42) { // DataBedrock
        padding[21];
    }
    else if (extra_tile_type == 43) { // Shelf
        u32 top_left_item_id;
        u32 top_right_item_id;
        u32 bottom_left_item_id;
        u32 bottom_right_item_id;
    }
    else if (extra_tile_type == 44) { // VipEntrance
        u8 unknown_1;
        u32 owner_uid;
        u32 access_count;
        u32 access_uids[access_count];
    }
    else if (extra_tile_type == 45) { // ChallangeTimer
        // Empty
    }
    else if (extra_tile_type == 47) { // FishWallMount
        u16 label_len;
        char label[label_len];
        u32 item_id;
        u8 lb;
    }
    else if (extra_tile_type == 48) { // Portrait
        u16 label_len;
        char label[label_len];
        u32 unknown_1;
        u32 unknown_2;
        u32 unknown_3;
        u32 unknown_4;
        u32 face;
        u32 hat;
        u32 hair;
        u16 unknown_5;
        u16 unknown_6;
    }
    else if (extra_tile_type == 49) { // GuildWeatherMachine
        u32 unknown_1;
        u32 gravity;
        u8 flags;
    }
    else if (extra_tile_type == 50) { // FossilPrepStation
        u32 unknown_1;
    }
    else if (extra_tile_type == 51) { // DnaExtractor
        // Empty
    }
    else if (extra_tile_type == 52) { // Howler
        // Empty
    }
    else if (extra_tile_type == 53) { // ChemsynthTank
        u32 current_chem;
        u32 target_chem;
    }
    else if (extra_tile_type == 54) { // StorageBlock
        u16 data_len;
        StorageBlockItemInfo items[data_len / 13];
    }
    else if (extra_tile_type == 55) { // CookingOven
        u32 temperature_level;
        u32 ingredient_count;
        CookingOvenIngredientInfo ingredients[ingredient_count];
        u32 unknown_1;
        u32 unknown_2;
        u32 unknown_3;
    }
    else if (extra_tile_type == 56) { // AudioRack
        u16 note_len;
        char note[note_len];
        u32 volume;
    }
    else if (extra_tile_type == 57) { // GeigerCharger
        u32 unknown_1;
    }
    else if (extra_tile_type == 58) { // AdventureBegins
        // Empty
    }
    else if (extra_tile_type == 59) { // TombRobber
        // Empty
    }
    else if (extra_tile_type == 60) { // BalloonOMatic
        u32 total_rarity;
        u8 team_type;
    }
    else if (extra_tile_type == 61) { // TrainingPort
        u32 fish_lb;
        u16 fish_status;
        u32 fish_id;
        u32 fish_total_exp;
        u32 fish_level;
        u32 unknown_2;
    }
    else if (extra_tile_type == 62) { // ItemSucker
        u32 item_id_to_suck;
        u32 item_amount;
        u16 flags;
        u32 limit;
    }
    else if (extra_tile_type == 63) { // CyBot
        u32 sync_timer;
        u32 activated;
        u32 command_data_count;
        CyBotCommandData command_datas[command_data_count];
    }
    else if (extra_tile_type == 65) { // GuildItem
        padding[17];
    }
    else if (extra_tile_type == 66) { // Growscan
        u8 unknown_1;
    }
    else if (extra_tile_type == 67) { // ContainmentFieldPowerNode
        u32 ghost_jar_count;
        u32 unknown_1_size;
        u32 unknown_1[unknown_1_size];
    }
    else if (extra_tile_type == 68) { // SpiritBoard
        u32 unknown_1;
        u32 unknown_2;
        u32 unknown_3;
    }
    else if (extra_tile_type == 69) { // TesseractManipulator (item 6952)
        u32 gems;
        u32 unknown_2;
        u32 item_id;
        u32 unknown_4;
    }
    else if (extra_tile_type == 72) { // StormyCloud
        u32 sting_duration;
        u32 is_solid;
        u32 non_solid_duration;
    }
    else if (extra_tile_type == 73) { // TemporaryPlatform
        u32 unknown_1;
    }
    else if (extra_tile_type == 74) { // SafeVault
        // Empty
    }
    else if (extra_tile_type == 75) { // AngelicCountingCloud
        u32 is_raffling;
        u16 unknown_1;
        u8 ascii_code;
    }
    else if (extra_tile_type == 77) { // InfinityWeatherMachine
        u32 interval_minutes;
        u32 weather_machine_list_size;
        u32 weather_machine_list[weather_machine_list_size];
    }
    else if (extra_tile_type == 79) { // PineappleGuzzler
        // Empty
    }
    else if (extra_tile_type == 80) { // KrakenGalaticBlock
        u8 pattern_index;
        u32 unknown_1;
        u8 r;
        u8 g;
        u8 b;
    }
    else if (extra_tile_type == 81) { // FriendsEntrance
        u32 owner_user_id;
        u16 unknown_1;
        u16 unknown_2;
    }
};

struct Tile {
    u16 foreground_item_id [[color("FF0000")]];
    u16 background_item_id [[color("00FF00")]];
    u16 parent_block_index;
    TileFlags flags;

    if (flags.HAS_PARENT) {
        u16 parent_data;
    }

    if (flags.HAS_EXTRA_DATA) {
        TileExtraData extra_data;
    }

    // Check for special tile 14666
    if (foreground_item_id == 14666) {
        u32 special_str_len;
        char special_text[special_str_len];
    }
};

struct DroppedItem {
    u16 id;
    float x;
    float y;
    u8 count;
    u8 flags;
    u32 uid;
};

struct WorldData {
    u16 version [[color("FFFF00")]];
    u32 flags;
    u16 name_len;
    char name[name_len] [[color("00FFFF")]];
    u32 width [[color("FF00FF")]];
    u32 height [[color("FF00FF")]];
    u32 tile_count [[color("FFAA00")]];

    padding[5]; // Debug flag

    Tile tiles[tile_count];

    padding[12];

    u32 dropped_items_count;
    u32 last_dropped_item_uid;
    DroppedItem dropped_items[dropped_items_count];

    WeatherType base_weather;
    u16 unknown_weather;
    WeatherType current_weather;
};

WorldData world @ 0x0;
