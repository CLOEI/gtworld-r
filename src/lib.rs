#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use byteorder::{LittleEndian, ReadBytesExt};
use gtitem_r::structs::ItemDatabase;
use std::io::{Cursor, Read};
use std::ops::Add;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct World {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tile_count: u32,
    pub tiles: Vec<Tile>,
    pub dropped: Dropped,
    pub base_weather: WeatherType,
    pub current_weather: WeatherType,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub item_database: Arc<RwLock<ItemDatabase>>,
    pub is_error: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tile {
    pub foreground_item_id: u16,
    pub background_item_id: u16,
    pub parent_block_index: u16,
    pub flags: TileFlags,
    pub flags_number: u16,
    pub tile_type: TileType,
    pub x: u32,
    pub y: u32,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub item_database: Arc<RwLock<ItemDatabase>>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TileFlags {
    pub has_extra_data: bool,
    pub has_parent: bool,
    pub was_spliced: bool,
    pub will_spawn_seeds_too: bool,
    pub is_seedling: bool,
    pub flipped_x: bool,
    pub is_on: bool,
    pub is_open_to_public: bool,
    pub bg_is_on: bool,
    pub fg_alt_mode: bool,
    pub is_wet: bool,
    pub glued: bool,
    pub on_fire: bool,
    pub painted_red: bool,
    pub painted_green: bool,
    pub painted_blue: bool,
}

impl TileFlags {
    pub fn from_u16(value: u16) -> Self {
        Self {
            has_extra_data: value & 0x01 != 0,
            has_parent: value & 0x02 != 0,
            was_spliced: value & 0x04 != 0,
            will_spawn_seeds_too: value & 0x08 != 0,
            is_seedling: value & 0x10 != 0,
            flipped_x: value & 0x20 != 0,
            is_on: value & 0x40 != 0,
            is_open_to_public: value & 0x80 != 0,
            bg_is_on: value & 0x100 != 0,
            fg_alt_mode: value & 0x200 != 0,
            is_wet: value & 0x400 != 0,
            glued: value & 0x800 != 0,
            on_fire: value & 0x1000 != 0,
            painted_red: value & 0x2000 != 0,
            painted_green: value & 0x4000 != 0,
            painted_blue: value & 0x8000 != 0,
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        if self.has_extra_data {
            value |= 0x01;
        }
        if self.has_parent {
            value |= 0x02;
        }
        if self.was_spliced {
            value |= 0x04;
        }
        if self.will_spawn_seeds_too {
            value |= 0x08;
        }
        if self.is_seedling {
            value |= 0x10;
        }
        if self.flipped_x {
            value |= 0x20;
        }
        if self.is_on {
            value |= 0x40;
        }
        if self.is_open_to_public {
            value |= 0x80;
        }
        if self.bg_is_on {
            value |= 0x100;
        }
        if self.fg_alt_mode {
            value |= 0x200;
        }
        if self.is_wet {
            value |= 0x400;
        }
        if self.glued {
            value |= 0x800;
        }
        if self.on_fire {
            value |= 0x1000;
        }
        if self.painted_red {
            value |= 0x2000;
        }
        if self.painted_green {
            value |= 0x4000;
        }
        if self.painted_blue {
            value |= 0x8000;
        }
        value
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WeatherType {
    Default,
    Sunset,
    Night,
    Desert,
    Sunny,
    RainyCity,
    Harvest,
    Mars,
    Spooky,
    Maw,
    Blank,
    Snowy,
    Growch,
    GrowchHappy,
    Undersea,
    Warp,
    Comet,
    Comet2,
    Party,
    Pineapple,
    SnowyNight,
    Spring,
    Wolf,
    NotInitialized,
    PurpleHaze,
    FireHaze,
    GreenHaze,
    AquaHaze,
    CustomHaze,
    CustomItems,
    Pagoda,
    Apocalypse,
    Jungle,
    BalloonWarz,
    Background,
    Autumn,
    Hearth,
    StPatricks,
    IceAge,
    Volcano,
    FloatingIslands,
    Mascot,
    DigitalRain,
    MonoChrome,
    Treasure,
    Surgery,
    Bountiful,
    Meteor,
    Stars,
    Ascended,
    Destroyed,
    GrowtopiaSign,
    Dungeon,
    LegendaryCity,
    BloodDragon,
    PopCity,
    Anzu,
    TmntCity,
    RadCity,
    Plaze,
    Nebula,
    ProtoStar,
    DarkMountains,
    Ac15,
    MountGrowMore,
    CrackInReality,
    LnyNian,
    RaymanLock,
    Steampunk,
    RealmOfSpirits,
    Blackhole,
    Gems,
    HolidayHaven,
    FenyxLock,
    EnchantedLock,
    RoyalEnchantedLock,
    NeptunesAtlantis,
    PinuskiPetalPerfectHaven,
    Candyland,
}

impl From<u16> for WeatherType {
    fn from(value: u16) -> Self {
        match value {
            0 => WeatherType::Default,
            1 => WeatherType::Sunset,
            2 => WeatherType::Night,
            3 => WeatherType::Desert,
            4 => WeatherType::Sunny,
            5 => WeatherType::RainyCity,
            6 => WeatherType::Harvest,
            7 => WeatherType::Mars,
            8 => WeatherType::Spooky,
            9 => WeatherType::Maw,
            10 => WeatherType::Blank,
            11 => WeatherType::Snowy,
            12 => WeatherType::Growch,
            13 => WeatherType::GrowchHappy,
            14 => WeatherType::Undersea,
            15 => WeatherType::Warp,
            16 => WeatherType::Comet,
            17 => WeatherType::Comet2,
            18 => WeatherType::Party,
            19 => WeatherType::Pineapple,
            20 => WeatherType::SnowyNight,
            21 => WeatherType::Spring,
            22 => WeatherType::Wolf,
            23 => WeatherType::NotInitialized,
            24 => WeatherType::PurpleHaze,
            25 => WeatherType::FireHaze,
            26 => WeatherType::GreenHaze,
            27 => WeatherType::AquaHaze,
            28 => WeatherType::CustomHaze,
            29 => WeatherType::CustomItems,
            30 => WeatherType::Pagoda,
            31 => WeatherType::Apocalypse,
            32 => WeatherType::Jungle,
            33 => WeatherType::BalloonWarz,
            34 => WeatherType::Background,
            35 => WeatherType::Autumn,
            36 => WeatherType::Hearth,
            37 => WeatherType::StPatricks,
            38 => WeatherType::IceAge,
            39 => WeatherType::Volcano,
            40 => WeatherType::FloatingIslands,
            41 => WeatherType::Mascot,
            42 => WeatherType::DigitalRain,
            43 => WeatherType::MonoChrome,
            44 => WeatherType::Treasure,
            45 => WeatherType::Surgery,
            46 => WeatherType::Bountiful,
            47 => WeatherType::Meteor,
            48 => WeatherType::Stars,
            49 => WeatherType::Ascended,
            50 => WeatherType::Destroyed,
            51 => WeatherType::GrowtopiaSign,
            52 => WeatherType::Dungeon,
            53 => WeatherType::LegendaryCity,
            54 => WeatherType::BloodDragon,
            55 => WeatherType::PopCity,
            56 => WeatherType::Anzu,
            57 => WeatherType::TmntCity,
            58 => WeatherType::RadCity,
            59 => WeatherType::Plaze,
            60 => WeatherType::Nebula,
            61 => WeatherType::ProtoStar,
            62 => WeatherType::DarkMountains,
            63 => WeatherType::Ac15,
            64 => WeatherType::MountGrowMore,
            65 => WeatherType::CrackInReality,
            66 => WeatherType::LnyNian,
            67 => WeatherType::RaymanLock,
            68 => WeatherType::Steampunk,
            69 => WeatherType::RealmOfSpirits,
            70 => WeatherType::Blackhole,
            71 => WeatherType::Gems,
            72 => WeatherType::HolidayHaven,
            73 => WeatherType::FenyxLock,
            74 => WeatherType::EnchantedLock,
            75 => WeatherType::RoyalEnchantedLock,
            76 => WeatherType::NeptunesAtlantis,
            77 => WeatherType::PinuskiPetalPerfectHaven,
            78 => WeatherType::Candyland,
            _ => WeatherType::Default,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TileType {
    Basic,
    Door {
        text: String,
        unknown_1: u8,
    },
    Sign {
        text: String,
    },
    Lock {
        settings: u8,
        owner_uid: u32,
        access_count: u32,
        access_uids: Vec<u32>,
        minimum_level: u8,
    },
    Seed {
        time_passed: u32,
        item_on_tree: u8,
        ready_to_harvest: bool,
        elapsed: Duration,
    },
    Mailbox {
        unknown_1: String,
        unknown_2: String,
        unknown_3: String,
        unknown_4: u8,
    },
    Bulletin {
        unknown_1: String,
        unknown_2: String,
        unknown_3: String,
        unknown_4: u8,
    },
    Dice {
        symbol: u8,
    },
    ChemicalSource {
        time_passed: u32,
        ready_to_harvest: bool,
        elapsed: Duration,
    },
    AchievementBlock {
        unknown_1: u32,
        tile_type: u8,
    },
    HearthMonitor {
        unknown_1: u32,
        player_name: String,
    },
    DonationBox {
        unknown_1: String,
        unknown_2: String,
        unknown_3: String,
        unknown_4: u8,
    },
    Mannequin {
        text: String,
        unknown_1: u8,
        clothing_1: u32,
        clothing_2: u16,
        clothing_3: u16,
        clothing_4: u16,
        clothing_5: u16,
        clothing_6: u16,
        clothing_7: u16,
        clothing_8: u16,
        clothing_9: u16,
        clothing_10: u16,
    },
    BunnyEgg {
        egg_placed: u32,
    },
    GamePack {
        team: u8,
    },
    GameGenerator {},
    XenoniteCrystal {
        unknown_1: u8,
        unknown_2: u32,
    },
    PhoneBooth {
        clothing_1: u16,
        clothing_2: u16,
        clothing_3: u16,
        clothing_4: u16,
        clothing_5: u16,
        clothing_6: u16,
        clothing_7: u16,
        clothing_8: u16,
        clothing_9: u16,
    },
    Crystal {
        unknown_1: String,
    },
    CrimeInProgress {
        unknown_1: String,
        unknown_2: u32,
        unknown_3: u8,
    },
    DisplayBlock {
        item_id: u32,
    },
    VendingMachine {
        item_id: u32,
        price: i32,
    },
    GivingTree {
        unknown_1: u16,
        unknown_2: u32,
    },
    CountryFlag {
        country: String,
    },
    WeatherMachine {
        settings: u32,
    },
    DataBedrock,
    Spotlight,
    FishTankPort {
        flags: u8,
        fishes: Vec<FishInfo>,
    },
    SolarCollector {
        unknown_1: [u8; 5],
    },
    Forge {
        temperature: u32,
    },
    SteamOrgan {
        instrument_type: u8,
        note: u32,
    },
    SilkWorm {
        type_: u8,
        name: String,
        age: u32,
        unknown_1: u32,
        unknown_2: u32,
        can_be_fed: u8,
        color: SilkWormColor,
        sick_duration: u32,
    },
    SewingMachine {
        bolt_id_list: Vec<u32>,
    },
    LobsterTrap,
    PaintingEasel {
        item_id: u32,
        label: String,
    },
    PetBattleCage {
        label: String,
        base_pet: u32,
        combined_pet_1: u32,
        combined_pet_2: u32,
    },
    PetTrainer {
        name: String,
        pet_total_count: u32,
        unknown_1: u32,
        pets_id: Vec<u32>,
    },
    SteamEngine {
        temperature: u32,
    },
    LockBot {
        time_passed: u32,
    },
    SpiritStorageUnit {
        ghost_jar_count: u32,
    },
    Shelf {
        top_left_item_id: u32,
        top_right_item_id: u32,
        bottom_left_item_id: u32,
        bottom_right_item_id: u32,
    },
    VipEntrance {
        unknown_1: u8,
        owner_uid: u32,
        access_uids: Vec<u32>,
    },
    ChallangeTimer,
    FishWallMount {
        label: String,
        item_id: u32,
        lb: u8,
    },
    Portrait {
        label: String,
        unknown_1: u32,
        unknown_2: u32,
        unknown_3: u32,
        unknown_4: u32,
        face: u32,
        hat: u32,
        hair: u32,
        unknown_5: u16,
        unknown_6: u16,
    },
    GuildWeatherMachine {
        unknown_1: u32,
        gravity: u32,
        flags: u8,
    },
    FossilPrepStation {
        unknown_1: u32,
    },
    DnaExtractor,
    Howler,
    ChemsynthTank {
        current_chem: u32,
        target_chem: u32,
    },
    StorageBlock {
        items: Vec<StorageBlockItemInfo>,
    },
    CookingOven {
        temperature_level: u32,
        ingredients: Vec<CookingOvenIngredientInfo>,
        unknown_1: u32,
        unknown_2: u32,
        unknown_3: u32,
    },
    AudioRack {
        note: String,
        volume: u32,
    },
    GeigerCharger {
        unknown_1: u32,
    },
    AdventureBegins,
    TombRobber,
    BalloonOMatic {
        total_rarity: u32,
        team_type: u8,
    },
    TrainingPort {
        fish_lb: u32,
        fish_status: u16,
        fish_id: u32,
        fish_total_exp: u32,
        fish_level: u32,
        unknown_2: u32,
    },
    ItemSucker {
        item_id_to_suck: u32,
        item_amount: u32,
        flags: u16,
        limit: u32,
    },
    CyBot {
        sync_timer: u32,
        activated: u32,
        command_datas: Vec<CyBotCommandData>,
    },
    GuildItem,
    Growscan {
        unknown_1: u8,
    },
    ContainmentFieldPowerNode {
        ghost_jar_count: u32,
        unknown_1: Vec<u32>,
    },
    SpiritBoard {
        unknown_1: u32,
        unknown_2: u32,
        unknown_3: u32,
    },
    StormyCloud {
        sting_duration: u32,
        is_solid: u32,
        non_solid_duration: u32,
    },
    TemporaryPlatform {
        unknown_1: u32,
    },
    SafeVault,
    AngelicCountingCloud {
        is_raffling: u32,
        unknown_1: u16,
        ascii_code: u8,
    },
    InfinityWeatherMachine {
        interval_minutes: u32,
        weather_machine_list: Vec<u32>,
    },
    PineappleGuzzler,
    KrakenGalaticBlock {
        pattern_index: u8,
        unknown_1: u32,
        r: u8,
        g: u8,
        b: u8,
    },
    FriendsEntrance {
        owner_user_id: u32,
        unknown_1: u16,
        unknown_2: u16,
    },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FishInfo {
    pub fish_item_id: u32,
    pub lbs: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SilkWormColor {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StorageBlockItemInfo {
    pub id: u32,
    pub amount: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CookingOvenIngredientInfo {
    pub item_id: u32,
    pub time_added: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CyBotCommandData {
    pub command_id: u32,
    pub is_command_used: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Dropped {
    pub items_count: u32,
    pub last_dropped_item_uid: u32,
    pub items: Vec<DroppedItem>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DroppedItem {
    pub id: u16,
    pub x: f32,
    pub y: f32,
    pub count: u8,
    pub flags: u8,
    pub uid: u32,
}

impl Tile {
    pub fn new(
        foreground_item_id: u16,
        background_item_id: u16,
        parent_block_index: u16,
        flags: TileFlags,
        flags_number: u16,
        x: u32,
        y: u32,
        item_database: Arc<RwLock<ItemDatabase>>
    ) -> Tile {
        Tile {
            foreground_item_id,
            background_item_id,
            parent_block_index,
            flags,
            flags_number,
            tile_type: TileType::Basic,
            x,
            y,
            item_database,
        }
    }

    pub fn harvestable(&self) -> bool {
        match self.tile_type {
            TileType::Seed {
                ready_to_harvest,
                elapsed,
                ..
            } => {
                if ready_to_harvest {
                    true
                } else {
                    let item_database = self.item_database.read().unwrap();
                    let item = item_database
                        .get_item(&(self.foreground_item_id as u32))
                        .unwrap();
                    if (elapsed.as_secs()) >= item.grow_time as u64 {
                        true
                    } else {
                        false
                    }
                }
            }
            TileType::ChemicalSource {
                ready_to_harvest,
                elapsed,
                ..
            } => {
                if ready_to_harvest {
                    true
                } else {
                    let item_database = self.item_database.read().unwrap();
                    let item = item_database
                        .get_item(&(self.foreground_item_id as u32))
                        .unwrap();
                    if (elapsed.as_secs()) >= item.grow_time as u64 {
                        true
                    } else {
                        false
                    }
                }
            }
            _ => false,
        }
    }
}

impl World {
    pub fn new(item_database: Arc<RwLock<ItemDatabase>>) -> World {
        World {
            name: "EXIT".to_string(),
            width: 0,
            height: 0,
            tile_count: 0,
            tiles: Vec::new(),
            dropped: Dropped {
                items_count: 0,
                last_dropped_item_uid: 0,
                items: Vec::new(),
            },
            base_weather: WeatherType::Default,
            current_weather: WeatherType::Default,
            is_error: false,
            item_database,
        }
    }

    pub fn reset(&mut self) {
        self.name = "EXIT".to_string();
        self.width = 0;
        self.height = 0;
        self.tile_count = 0;
        self.tiles.clear();
        self.dropped.items_count = 0;
        self.dropped.last_dropped_item_uid = 0;
        self.dropped.items.clear();
        self.base_weather = WeatherType::Default;
        self.current_weather = WeatherType::Default;
    }

    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = (y * self.width + x) as usize;
        self.tiles.get_mut(index)
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = (y * self.width + x) as usize;
        self.tiles.get(index)
    }

    pub fn is_tile_harvestable(&self, tile: &Tile) -> bool {
        match tile.tile_type {
            TileType::Seed {
                ready_to_harvest,
                elapsed,
                ..
            } => {
                if ready_to_harvest {
                    true
                } else {
                    let item_database = self.item_database.read().unwrap();
                    let item = item_database
                        .get_item(&(tile.foreground_item_id as u32))
                        .unwrap();
                    if (elapsed.as_secs()) >= item.grow_time as u64 {
                        true
                    } else {
                        false
                    }
                }
            }
            TileType::ChemicalSource {
                ready_to_harvest,
                elapsed,
                ..
            } => {
                if ready_to_harvest {
                    true
                } else {
                    let item_database = self.item_database.read().unwrap();
                    let item = item_database
                        .get_item(&(tile.foreground_item_id as u32))
                        .unwrap();
                    if (elapsed.as_secs()) >= item.grow_time as u64 {
                        true
                    } else {
                        false
                    }
                }
            }
            _ => false,
        }
    }

    pub fn is_harvestable(&self, x: u32, y: u32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            return self.is_tile_harvestable(tile);
        }
        false
    }

    pub fn update_tile(&mut self, mut tile: Tile, mut data: &mut Cursor<&[u8]>, replace: bool) -> Option<()> {
        tile.foreground_item_id = data.read_u16::<LittleEndian>().unwrap();
        tile.background_item_id = data.read_u16::<LittleEndian>().unwrap();
        tile.parent_block_index = data.read_u16::<LittleEndian>().unwrap();
        let flags = data.read_u16::<LittleEndian>().unwrap();
        tile.flags = TileFlags::from_u16(flags);
        tile.flags_number = flags;

        let item_count = {
            let item_database = self.item_database.read().unwrap();
            item_database.item_count
        };
        if tile.foreground_item_id > item_count as u16
            || tile.background_item_id > item_count as u16
        {
            self.is_error = true;
            let new_tile = Tile::new(0, 0, 0, tile.flags, tile.flags_number, tile.x, tile.y, Arc::clone(&self.item_database));
            self.tiles.push(new_tile);
            return None;
        }

        if tile.flags.has_parent {
            data.read_u16::<LittleEndian>().unwrap();
        }

        if tile.flags.has_extra_data {
            let extra_tile_type = data.read_u8().unwrap();
            self.get_extra_tile_data(&mut tile, &mut data, extra_tile_type, &self.item_database);
        }

        if tile.foreground_item_id == 14666 {
            let str_len = data.read_u32::<LittleEndian>().unwrap();
            let mut text = vec![0; str_len as usize];
            data.read_exact(&mut text).unwrap();
        }

        if replace {
            let index = (tile.y * self.width + tile.x) as usize;
            self.tiles[index] = tile;
        } else {
            self.tiles.push(tile);
        }

        Some(())
    }

    pub fn parse(&mut self, data: &[u8]) {
        self.reset();
        let mut data = Cursor::new(data);
        // first 6 byte is unknown
        data.set_position(data.position() + 6);
        let str_len = data.read_u16::<LittleEndian>().unwrap();
        let mut name = vec![0; str_len as usize];
        data.read_exact(&mut name).unwrap();
        let width = data.read_u32::<LittleEndian>().unwrap();
        let height = data.read_u32::<LittleEndian>().unwrap();
        let tile_count = data.read_u32::<LittleEndian>().unwrap();
        data.set_position(data.position() + 5);
        self.name = String::from_utf8_lossy(&name).to_string();
        self.width = width;
        self.height = height;
        self.tile_count = tile_count;

        // tiles
        for count in 0..tile_count {
            let x = (count) % self.width;
            let y = (count) / self.width;
            let tile = Tile::new(0, 0, 0, TileFlags::default(), 0, x, y, Arc::clone(&self.item_database));
            match self.update_tile(tile, &mut data, false) {
                Some(_) => {}
                None => {
                    break;
                }
            }
        }

        if self.is_error {
            return;
        }

        data.set_position(data.position() + 12); // it exist in the binary, i don't know what it is
        self.dropped.items_count = data.read_u32::<LittleEndian>().unwrap();
        self.dropped.last_dropped_item_uid = data.read_u32::<LittleEndian>().unwrap();
        for _ in 0..self.dropped.items_count {
            let id = data.read_u16::<LittleEndian>().unwrap();
            let x = data.read_f32::<LittleEndian>().unwrap();
            let y = data.read_f32::<LittleEndian>().unwrap();
            let count = data.read_u8().unwrap();
            let flags = data.read_u8().unwrap();
            let uid = data.read_u32::<LittleEndian>().unwrap();
            self.dropped.items.push(DroppedItem {
                id,
                x,
                y,
                count,
                flags,
                uid,
            });
        }

        let base_weather = data.read_u16::<LittleEndian>().unwrap();
        data.read_u16::<LittleEndian>().unwrap(); // unknown
        let current_weather = data.read_u16::<LittleEndian>().unwrap();
        self.base_weather = WeatherType::from(base_weather);
        self.current_weather = WeatherType::from(current_weather);
    }

    fn get_extra_tile_data(
        &self,
        tile: &mut Tile,
        data: &mut Cursor<&[u8]>,
        item_type: u8,
        item_database: &Arc<RwLock<ItemDatabase>>,
    ) {
        match item_type {
            1 => {
                // TileType::Door
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut text = vec![0; str_len as usize];
                data.read_exact(&mut text).unwrap();
                let text = String::from_utf8_lossy(&text).to_string();
                let unknown_1 = data.read_u8().unwrap();

                tile.tile_type = TileType::Door { text, unknown_1 };
            }
            2 => {
                // TileType::Sign
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut text = vec![0; str_len as usize];
                data.read_exact(&mut text).unwrap();
                let text = String::from_utf8_lossy(&text).to_string();
                let _ = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::Sign { text };
            }
            3 => {
                // TileType::Lock
                let settings = data.read_u8().unwrap();
                let owner_uid = data.read_u32::<LittleEndian>().unwrap();
                let access_count = data.read_u32::<LittleEndian>().unwrap();
                let mut access_uids = Vec::new();
                for _ in 0..access_count {
                    access_uids.push(data.read_u32::<LittleEndian>().unwrap());
                }
                let minimum_level = data.read_u8().unwrap();
                let mut unknown_1 = [0; 7];
                data.read_exact(&mut unknown_1).unwrap();

                if tile.foreground_item_id == 5814 {
                    data.set_position(data.position() + 16);
                }

                tile.tile_type = TileType::Lock {
                    settings,
                    owner_uid,
                    access_count,
                    access_uids,
                    minimum_level,
                };
            }
            4 => {
                // TileType::Seed
                let time_passed = data.read_u32::<LittleEndian>().unwrap();
                let item_on_tree = data.read_u8().unwrap();
                let ready_to_harvest = {
                    let item_database = item_database.read().unwrap();
                    let item = item_database
                        .get_item(&(tile.foreground_item_id as u32))
                        .unwrap();
                    if item.grow_time <= time_passed {
                        true
                    } else {
                        false
                    }
                };
                let timer = Instant::now();
                let elapsed = timer.elapsed().add(Duration::from_secs(time_passed as u64));

                tile.tile_type = TileType::Seed {
                    time_passed,
                    item_on_tree,
                    ready_to_harvest,
                    elapsed,
                };
            }
            6 => {
                // TileType::Mailbox
                let str_len_1 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; str_len_1 as usize];
                data.read_exact(&mut unknown_1).unwrap();

                let str_len_2 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_2 = vec![0; str_len_2 as usize];
                data.read_exact(&mut unknown_2).unwrap();

                let str_len_3 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_3 = vec![0; str_len_3 as usize];
                data.read_exact(&mut unknown_3).unwrap();

                let unknown_4 = data.read_u8().unwrap();

                tile.tile_type = TileType::Mailbox {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                    unknown_2: String::from_utf8_lossy(&unknown_2).to_string(),
                    unknown_3: String::from_utf8_lossy(&unknown_3).to_string(),
                    unknown_4,
                };
            }
            7 => {
                // TileType::Bulletin
                let str_len_1 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; str_len_1 as usize];
                data.read_exact(&mut unknown_1).unwrap();

                let str_len_2 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_2 = vec![0; str_len_2 as usize];
                data.read_exact(&mut unknown_2).unwrap();

                let str_len_3 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_3 = vec![0; str_len_3 as usize];
                data.read_exact(&mut unknown_3).unwrap();

                let unknown_4 = data.read_u8().unwrap();

                tile.tile_type = TileType::Bulletin {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                    unknown_2: String::from_utf8_lossy(&unknown_2).to_string(),
                    unknown_3: String::from_utf8_lossy(&unknown_3).to_string(),
                    unknown_4,
                };
            }
            8 => {
                // TileType::Dice
                let symbol = data.read_u8().unwrap();

                tile.tile_type = TileType::Dice { symbol };
            }
            9 => {
                // TileType::ChemicalSource
                let time_passed = data.read_u32::<LittleEndian>().unwrap();
                let ready_to_harvest = {
                    let item_database = item_database.read().unwrap();
                    let item = item_database
                        .get_item(&(tile.foreground_item_id as u32))
                        .unwrap();
                    if time_passed >= item.grow_time {
                        true
                    } else {
                        false
                    }
                };
                let timer = Instant::now();
                let elapsed = timer.elapsed().add(Duration::from_secs(time_passed as u64));

                tile.tile_type = TileType::ChemicalSource { time_passed, ready_to_harvest, elapsed };
            }
            10 => {
                // TileType::AchievementBlock
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let tile_type = data.read_u8().unwrap();

                tile.tile_type = TileType::AchievementBlock {
                    unknown_1,
                    tile_type,
                };
            }
            11 => {
                // TileType::HearthMonitor
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut player_name = vec![0; str_len as usize];
                data.read_exact(&mut player_name).unwrap();
                let player_name = String::from_utf8_lossy(&player_name).to_string();

                tile.tile_type = TileType::HearthMonitor {
                    unknown_1,
                    player_name,
                };
            }
            12 => {
                // TileType::DonationBox
                let str_len_1 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; str_len_1 as usize];
                data.read_exact(&mut unknown_1).unwrap();

                let str_len_2 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_2 = vec![0; str_len_2 as usize];
                data.read_exact(&mut unknown_2).unwrap();

                let str_len_3 = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_3 = vec![0; str_len_3 as usize];
                data.read_exact(&mut unknown_3).unwrap();

                let unknown_4 = data.read_u8().unwrap();

                tile.tile_type = TileType::DonationBox {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                    unknown_2: String::from_utf8_lossy(&unknown_2).to_string(),
                    unknown_3: String::from_utf8_lossy(&unknown_3).to_string(),
                    unknown_4,
                };
            }
            14 => {
                // TileType::Mannequin
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut text = vec![0; str_len as usize];
                data.read_exact(&mut text).unwrap();
                let text = String::from_utf8_lossy(&text).to_string();
                let unknown_1 = data.read_u8().unwrap();
                let clothing_1 = data.read_u32::<LittleEndian>().unwrap();
                let clothing_2 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_3 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_4 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_5 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_6 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_7 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_8 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_9 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_10 = data.read_u16::<LittleEndian>().unwrap();

                tile.tile_type = TileType::Mannequin {
                    text,
                    unknown_1,
                    clothing_1,
                    clothing_2,
                    clothing_3,
                    clothing_4,
                    clothing_5,
                    clothing_6,
                    clothing_7,
                    clothing_8,
                    clothing_9,
                    clothing_10,
                };
            }
            15 => {
                // TileType::BunnyEgg
                let egg_placed = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::BunnyEgg { egg_placed };
            }
            16 => {
                // TileType::GamePack
                let team = data.read_u8().unwrap();

                tile.tile_type = TileType::GamePack { team };
            }
            17 => {
                // TileType::GameGenerator
                tile.tile_type = TileType::GameGenerator {};
            }
            18 => {
                // TileType::XenoniteCrystal
                let unknown_1 = data.read_u8().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::XenoniteCrystal {
                    unknown_1,
                    unknown_2,
                };
            }
            19 => {
                // TileType::PhoneBooth
                let clothing_1 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_2 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_3 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_4 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_5 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_6 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_7 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_8 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_9 = data.read_u16::<LittleEndian>().unwrap();

                tile.tile_type = TileType::PhoneBooth {
                    clothing_1,
                    clothing_2,
                    clothing_3,
                    clothing_4,
                    clothing_5,
                    clothing_6,
                    clothing_7,
                    clothing_8,
                    clothing_9,
                };
            }
            20 => {
                // TileType::Crystal
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; str_len as usize];
                data.read_exact(&mut unknown_1).unwrap();

                tile.tile_type = TileType::Crystal {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                };
            }
            21 => {
                // TileType::CrimeInProgress
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; str_len as usize];
                data.read_exact(&mut unknown_1).unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_3 = data.read_u8().unwrap();

                tile.tile_type = TileType::CrimeInProgress {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                    unknown_2,
                    unknown_3,
                };
            }
            23 => {
                // TileType::DisplayBlock
                let item_id = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::DisplayBlock { item_id };
            }
            24 => {
                // TileType::VendingMachine
                let item_id = data.read_u32::<LittleEndian>().unwrap();
                let price = data.read_i32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::VendingMachine { item_id, price };
            }
            25 => {
                // TileType::FishTankPort
                let flags = data.read_u8().unwrap();
                let fish_count = data.read_u32::<LittleEndian>().unwrap();
                let mut fishes = Vec::new();
                for _ in 0..(fish_count / 2) {
                    let fish_item_id = data.read_u32::<LittleEndian>().unwrap();
                    let lbs = data.read_u32::<LittleEndian>().unwrap();
                    fishes.push(FishInfo { fish_item_id, lbs });
                }
                tile.tile_type = TileType::FishTankPort { flags, fishes };
            }
            26 => {
                // TileType::SolarCollector
                let mut unknown_1 = [0; 5];
                data.read_exact(&mut unknown_1).unwrap();
                tile.tile_type = TileType::SolarCollector { unknown_1 };
            }
            27 => {
                // TileType::Forge
                let temperature = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::Forge { temperature };
            }
            28 => {
                // TileType::GivingTree
                let unknown_1 = data.read_u16::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::GivingTree {
                    unknown_1,
                    unknown_2,
                };
            }
            30 => {
                // TileType::SteamOrgan
                let instrument_type = data.read_u8().unwrap();
                let note = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::SteamOrgan {
                    instrument_type,
                    note,
                };
            }
            31 => {
                // TileType::SilkWorm
                let type_ = data.read_u8().unwrap();
                let name_len = data.read_u16::<LittleEndian>().unwrap();
                let mut name = vec![0; name_len as usize];
                data.read_exact(&mut name).unwrap();
                let name = String::from_utf8_lossy(&name).to_string();
                let age = data.read_u32::<LittleEndian>().unwrap();
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();
                let can_be_fed = data.read_u8().unwrap();
                let color = data.read_u32::<LittleEndian>().unwrap();
                let sick_duration = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::SilkWorm {
                    type_,
                    name,
                    age,
                    unknown_1,
                    unknown_2,
                    can_be_fed,
                    color: SilkWormColor {
                        a: (color >> 24) as u8,
                        r: ((color >> 16) & 0xFF) as u8,
                        g: ((color >> 8) & 0xFF) as u8,
                        b: (color & 0xFF) as u8,
                    },
                    sick_duration,
                };
            }
            32 => {
                // TileType::SewingMachine
                let bolt_len = data.read_u16::<LittleEndian>().unwrap();
                let mut bolt_id_list = Vec::new();
                for _ in 0..bolt_len {
                    let bolt_id = data.read_u32::<LittleEndian>().unwrap();
                    bolt_id_list.push(bolt_id);
                }
                tile.tile_type = TileType::SewingMachine { bolt_id_list };
            }
            33 => {
                // TileType::CountryFlag
                let country_len = data.read_u16::<LittleEndian>().unwrap();
                let mut country = vec![0; country_len as usize];
                data.read_exact(&mut country).unwrap();
                let country = String::from_utf8_lossy(&country).to_string();

                tile.tile_type = TileType::CountryFlag { country };
            }
            34 => {
                // TileType::LobsterTrap
                tile.tile_type = TileType::LobsterTrap;
            }
            35 => {
                // TileType::PaintingEasel
                let item_id = data.read_u32::<LittleEndian>().unwrap();
                let label_len = data.read_u16::<LittleEndian>().unwrap();
                let mut label = vec![0; label_len as usize];
                data.read_exact(&mut label).unwrap();
                let label = String::from_utf8_lossy(&label).to_string();

                tile.tile_type = TileType::PaintingEasel { item_id, label };
            }
            36 => {
                // TileType::PetBattleCage
                let label_len = data.read_u16::<LittleEndian>().unwrap();
                let mut label = vec![0; label_len as usize];
                data.read_exact(&mut label).unwrap();
                let label = String::from_utf8_lossy(&label).to_string();
                let base_pet = data.read_u32::<LittleEndian>().unwrap();
                let combined_pet_1 = data.read_u32::<LittleEndian>().unwrap();
                let combined_pet_2 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::PetBattleCage {
                    label,
                    base_pet,
                    combined_pet_1,
                    combined_pet_2,
                };
            }
            37 => {
                // TileType::PetTrainer
                let name_len = data.read_u16::<LittleEndian>().unwrap();
                let mut name = vec![0; name_len as usize];
                data.read_exact(&mut name).unwrap();
                let name = String::from_utf8_lossy(&name).to_string();
                let pet_total_count = data.read_u32::<LittleEndian>().unwrap();
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let mut pets_id = Vec::new();
                for _ in 0..pet_total_count {
                    let pet_id = data.read_u32::<LittleEndian>().unwrap();
                    pets_id.push(pet_id);
                }

                tile.tile_type = TileType::PetTrainer {
                    name,
                    pet_total_count,
                    unknown_1,
                    pets_id,
                };
            }
            38 => {
                // TileType::SteamEngine
                let temperature = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::SteamEngine { temperature };
            }
            39 => {
                // TileType::LockBot
                let time_passed = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::LockBot { time_passed };
            }
            40 => {
                // TileType::WeatherMachine
                let settings = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::WeatherMachine { settings };
            }
            41 => {
                // TileType::SpiritStorageUnit
                let ghost_jar_count = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::SpiritStorageUnit { ghost_jar_count };
            }
            42 => {
                // TileType::DataBedrock
                data.set_position(data.position() + 21);
                tile.tile_type = TileType::DataBedrock;
            }
            43 => {
                // TileType::Shelf
                let top_left_item_id = data.read_u32::<LittleEndian>().unwrap();
                let top_right_item_id = data.read_u32::<LittleEndian>().unwrap();
                let bottom_left_item_id = data.read_u32::<LittleEndian>().unwrap();
                let bottom_right_item_id = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::Shelf {
                    top_left_item_id,
                    top_right_item_id,
                    bottom_left_item_id,
                    bottom_right_item_id,
                };
            }
            44 => {
                // TileType::VipEntrance
                let unknown_1 = data.read_u8().unwrap();
                let owner_uid = data.read_u32::<LittleEndian>().unwrap();
                let access_count = data.read_u32::<LittleEndian>().unwrap();
                let mut access_uids = Vec::new();
                for _ in 0..access_count {
                    let uid = data.read_u32::<LittleEndian>().unwrap();
                    access_uids.push(uid);
                }

                tile.tile_type = TileType::VipEntrance {
                    unknown_1,
                    owner_uid,
                    access_uids,
                };
            }
            45 => {
                // TileType::ChallangeTimer
                tile.tile_type = TileType::ChallangeTimer;
            }
            47 => {
                // TileType::FishWallMount
                let label_len = data.read_u16::<LittleEndian>().unwrap();
                let mut label = vec![0; label_len as usize];
                data.read_exact(&mut label).unwrap();
                let label = String::from_utf8_lossy(&label).to_string();
                let item_id = data.read_u32::<LittleEndian>().unwrap();
                let lb = data.read_u8().unwrap();

                tile.tile_type = TileType::FishWallMount { label, item_id, lb };
            }
            48 => {
                // TileType::Portrait
                let label_len = data.read_u16::<LittleEndian>().unwrap();
                let mut label = vec![0; label_len as usize];
                data.read_exact(&mut label).unwrap();
                let label = String::from_utf8_lossy(&label).to_string();
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_3 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_4 = data.read_u32::<LittleEndian>().unwrap();
                let face = data.read_u32::<LittleEndian>().unwrap();
                let hat = data.read_u32::<LittleEndian>().unwrap();
                let hair = data.read_u32::<LittleEndian>().unwrap();
                let unknown_5 = data.read_u16::<LittleEndian>().unwrap();
                let unknown_6 = data.read_u16::<LittleEndian>().unwrap();

                tile.tile_type = TileType::Portrait {
                    label,
                    unknown_1,
                    unknown_2,
                    unknown_3,
                    unknown_4,
                    face,
                    hat,
                    hair,
                    unknown_5,
                    unknown_6,
                };
            }
            49 => {
                // TileType::GuildWeatherMachine
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let gravity = data.read_u32::<LittleEndian>().unwrap();
                let flags = data.read_u8().unwrap();

                tile.tile_type = TileType::GuildWeatherMachine {
                    unknown_1,
                    gravity,
                    flags,
                };
            }
            50 => {
                // TileType::FossilPrepStation
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::FossilPrepStation { unknown_1 };
            }
            51 => {
                // TileType::DnaExtractor
                tile.tile_type = TileType::DnaExtractor;
            }
            52 => {
                // TileType::Howler
                tile.tile_type = TileType::Howler;
            }
            53 => {
                // TileType::ChemsynthTank
                let current_chem = data.read_u32::<LittleEndian>().unwrap();
                let target_chem = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::ChemsynthTank {
                    current_chem,
                    target_chem,
                };
            }
            54 => {
                // TileType::StorageBlock
                let data_len = data.read_u16::<LittleEndian>().unwrap();
                let mut items = Vec::new();
                for _ in 0..(data_len / 13) {
                    data.set_position(data.position() + 3);
                    let id = data.read_u32::<LittleEndian>().unwrap();
                    data.set_position(data.position() + 2);
                    let amount = data.read_u32::<LittleEndian>().unwrap();
                    items.push(StorageBlockItemInfo { id, amount });
                }
                tile.tile_type = TileType::StorageBlock { items };
            }
            55 => {
                // TileType::CookingOven
                let temperature_level = data.read_u32::<LittleEndian>().unwrap();
                let ingredient_count = data.read_u32::<LittleEndian>().unwrap();
                let mut ingredients = Vec::new();
                for _ in 0..ingredient_count {
                    let item_id = data.read_u32::<LittleEndian>().unwrap();
                    let time_added = data.read_u32::<LittleEndian>().unwrap();
                    ingredients.push(CookingOvenIngredientInfo {
                        item_id,
                        time_added,
                    });
                }
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_3 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::CookingOven {
                    temperature_level,
                    ingredients,
                    unknown_1,
                    unknown_2,
                    unknown_3,
                };
            }
            56 => {
                // TileType::AudioRack
                let note_len = data.read_u16::<LittleEndian>().unwrap();
                let mut note = vec![0; note_len as usize];
                data.read_exact(&mut note).unwrap();
                let note = String::from_utf8_lossy(&note).to_string();
                let volume = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::AudioRack { note, volume };
            }
            57 => {
                // TileType::GeigerCharger
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::GeigerCharger { unknown_1 };
            }
            58 => {
                // TileType::AdventureBegins
                tile.tile_type = TileType::AdventureBegins;
            }
            59 => {
                // TileType::TombRobber
                tile.tile_type = TileType::TombRobber;
            }
            60 => {
                // TileType::BalloonOMatic
                let total_rarity = data.read_u32::<LittleEndian>().unwrap();
                let team_type = data.read_u8().unwrap();

                tile.tile_type = TileType::BalloonOMatic {
                    total_rarity,
                    team_type,
                };
            }
            61 => {
                // TileType::TrainingPort
                let fish_lb = data.read_u32::<LittleEndian>().unwrap();
                let fish_status = data.read_u16::<LittleEndian>().unwrap();
                let fish_id = data.read_u32::<LittleEndian>().unwrap();
                let fish_total_exp = data.read_u32::<LittleEndian>().unwrap();
                let fish_level = data.read_u32::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::TrainingPort {
                    fish_lb,
                    fish_status,
                    fish_id,
                    fish_total_exp,
                    fish_level,
                    unknown_2,
                };
            }
            62 => {
                // TileType::ItemSucker
                let item_id_to_suck = data.read_u32::<LittleEndian>().unwrap();
                let item_amount = data.read_u32::<LittleEndian>().unwrap();
                let flags = data.read_u16::<LittleEndian>().unwrap();
                let limit = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::ItemSucker {
                    item_id_to_suck,
                    item_amount,
                    flags,
                    limit,
                };
            }
            63 => {
                // TileType::CyBot
                let sync_timer = data.read_u32::<LittleEndian>().unwrap();
                let activated = data.read_u32::<LittleEndian>().unwrap();
                let command_data_count = data.read_u32::<LittleEndian>().unwrap();
                let mut command_datas = Vec::new();
                for _ in 0..command_data_count {
                    let command_id = data.read_u32::<LittleEndian>().unwrap();
                    let is_command_used = data.read_u32::<LittleEndian>().unwrap();
                    data.set_position(data.position() + 7);
                    command_datas.push(CyBotCommandData {
                        command_id,
                        is_command_used,
                    });
                }
                tile.tile_type = TileType::CyBot {
                    sync_timer,
                    activated,
                    command_datas,
                };
            }
            65 => {
                // TileType::GuildItem
                data.set_position(data.position() + 17);
                tile.tile_type = TileType::GuildItem;
            }
            66 => {
                // TileType::Growscan
                let unknown_1 = data.read_u8().unwrap();
                tile.tile_type = TileType::Growscan { unknown_1 };
            }
            67 => {
                // TileType::ContainmentFieldPowerNode
                let ghost_jar_count = data.read_u32::<LittleEndian>().unwrap();
                let unknown_1_size = data.read_u32::<LittleEndian>().unwrap();
                let mut unknown_1 = Vec::new();
                for _ in 0..unknown_1_size {
                    let value = data.read_u32::<LittleEndian>().unwrap();
                    unknown_1.push(value);
                }

                tile.tile_type = TileType::ContainmentFieldPowerNode {
                    ghost_jar_count,
                    unknown_1,
                };
            }
            68 => {
                // TileType::SpiritBoard
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_3 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::SpiritBoard {
                    unknown_1,
                    unknown_2,
                    unknown_3,
                };
            }
            72 => {
                // TileType::StormyCloud
                let sting_duration = data.read_u32::<LittleEndian>().unwrap();
                let is_solid = data.read_u32::<LittleEndian>().unwrap();
                let non_solid_duration = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::StormyCloud {
                    sting_duration,
                    is_solid,
                    non_solid_duration,
                };
            }
            73 => {
                // TileType::TemporaryPlatform
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                tile.tile_type = TileType::TemporaryPlatform { unknown_1 };
            }
            74 => {
                // TileType::SafeVault
                tile.tile_type = TileType::SafeVault;
            }
            75 => {
                // TileType::AngelicCountingCloud
                let is_raffling = data.read_u32::<LittleEndian>().unwrap();
                let unknown_1 = data.read_u16::<LittleEndian>().unwrap();
                let ascii_code = data.read_u8().unwrap();

                tile.tile_type = TileType::AngelicCountingCloud {
                    is_raffling,
                    unknown_1,
                    ascii_code,
                };
            }
            77 => {
                // TileType::InfinityWeatherMachine
                let interval_minutes = data.read_u32::<LittleEndian>().unwrap();
                let weather_machine_list_size = data.read_u32::<LittleEndian>().unwrap();
                let mut weather_machine_list = Vec::new();
                for _ in 0..weather_machine_list_size {
                    let weather_machine = data.read_u32::<LittleEndian>().unwrap();
                    weather_machine_list.push(weather_machine);
                }

                tile.tile_type = TileType::InfinityWeatherMachine {
                    interval_minutes,
                    weather_machine_list,
                };
            }
            79 => {
                // TileType::PineappleGuzzler
                tile.tile_type = TileType::PineappleGuzzler;
            }
            80 => {
                // TileType::KrakenGalaticBlock
                let pattern_index = data.read_u8().unwrap();
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let r = data.read_u8().unwrap();
                let g = data.read_u8().unwrap();
                let b = data.read_u8().unwrap();

                tile.tile_type = TileType::KrakenGalaticBlock {
                    pattern_index,
                    unknown_1,
                    r,
                    g,
                    b,
                };
            }
            81 => {
                // TileType::FriendsEntrance
                let owner_user_id = data.read_u32::<LittleEndian>().unwrap();
                let unknown_1 = data.read_u16::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u16::<LittleEndian>().unwrap();

                tile.tile_type = TileType::FriendsEntrance {
                    owner_user_id,
                    unknown_1,
                    unknown_2,
                };
            }
            _ => {
                tile.tile_type = TileType::Basic;
            }
        };
    }
}

#[test]
fn test_render_world() {
    use gtitem_r::load_from_file;
    use image::{ImageBuffer, Rgba};
    use std::fs::File;

    let item_database = Arc::new(RwLock::new(load_from_file("items.dat").unwrap()));
    let mut world = World::new(item_database);

    // get byte from world.dat file
    let mut file = File::open("world.dat").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    world.parse(&data);

    // world save to world.json
    let file = File::create("world.json").unwrap();
    serde_json::to_writer_pretty(file, &world).unwrap();

    let item_pixel_size = 32;
    let img_width = world.width * item_pixel_size;
    let img_height = world.height * item_pixel_size;
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(img_width as u32, img_height as u32);

    for x in 0..world.width {
        for y in 0..world.height {
            match &world.get_tile(x, y) {
                Some(tile) => {
                    let item_database = world.item_database.read().unwrap();
                    let item = {
                        let item = item_database
                            .get_item(&(tile.foreground_item_id as u32))
                            .unwrap();
                        item
                    };

                    let mut color = Rgba([0, 0, 0, 255]);
                    if item.name == "Blank" {
                        color = Rgba([96, 215, 242, 255]);
                        if tile.background_item_id != 0 {
                            let item = {
                                let item = item_database
                                    .get_item(&(tile.background_item_id as u32 + 1))
                                    .unwrap();
                                item
                            };

                            let colors = item.base_color;
                            let r = ((colors >> 24) & 0xFF) as u8;
                            let g = ((colors >> 16) & 0xFF) as u8;
                            let b = ((colors >> 8) & 0xFF) as u8;

                            color = Rgba([b, g, r, 255]);
                        }
                    } else {
                        let item = {
                            let item = item_database
                                .get_item(&(tile.foreground_item_id as u32 + 1))
                                .unwrap();
                            item
                        };

                        let colors = item.base_color;
                        let r = ((colors >> 24) & 0xFF) as u8;
                        let g = ((colors >> 16) & 0xFF) as u8;
                        let b = ((colors >> 8) & 0xFF) as u8;

                        color = Rgba([b, g, r, 255]);
                    }

                    for px in 0..item_pixel_size {
                        for py in 0..item_pixel_size {
                            let pixel_x = (x * item_pixel_size + px) as u32;
                            let pixel_y = (y * item_pixel_size + py) as u32;
                            img.put_pixel(pixel_x, pixel_y, color);
                        }
                    }
                }
                None => {
                    for px in 0..item_pixel_size {
                        for py in 0..item_pixel_size {
                            let pixel_x = (x * item_pixel_size + px) as u32;
                            let pixel_y = (y * item_pixel_size + py) as u32;
                            img.put_pixel(pixel_x, pixel_y, Rgba([255, 255, 0, 255]));
                        }
                    }
                    continue;
                }
            }
        }
    }

    img.save("output.png").unwrap();
}
